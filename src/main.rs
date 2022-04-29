// If we can't connect, serve all configured shards as standalone.
// If we can connect, and the remove server is serving our shards
//   - Stop serving shards.
//   - Wait for remote server to stop serving out shards.
//   - Serve our shards.
// If we can connect, and the other server is ready to serve, stop serving it's prefered shards
use log::{debug, info, warn};
use std::sync::{Arc, Mutex};

use tokio::time::{sleep, Duration, timeout};

use tonic::transport::Server;
use tonic::{Request, Response, Status};

use calandria_ha::calha::calha;
use calandria_ha::ping;
use calha::get_state_client::GetStateClient;
use calha::get_state_server::{GetState, GetStateServer};
use calha::{GetStateRequest, GetStateResponse, StateConfiguration};

#[derive(Default)]
pub struct HAStateServer {
    state: Arc<Mutex<GetStateResponse>>,
}

#[tonic::async_trait]
impl GetState for HAStateServer {
    async fn get_state(
        &self,
        _: Request<GetStateRequest>,
    ) -> Result<Response<GetStateResponse>, Status> {
        debug!("Serve GetStateResponse: Aquire Lock");
        let response = self.state.lock().unwrap().clone();
        debug!("Serve GetStateResponse: Release Lock");

        return Ok(Response::new(response));
    }
}

async fn start_server(
    config: StateConfiguration,
    state_return: Arc<Mutex<GetStateResponse>>,
) -> Result<(), tonic::transport::Error> {
    let primary_state_server = HAStateServer {
        state: state_return,
    };
    // Todo: There is probably a better way to get a [:::] socket
    let serving_addr = std::net::SocketAddr::new(
        std::net::IpAddr::V6(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
        u16::try_from(config.port).ok().unwrap(),
    );
    info!("Starting status server on {:#}", serving_addr);
    return Server::builder()
        .add_service(GetStateServer::new(primary_state_server))
        .serve(serving_addr)
        .await;
}


async fn make_remote_state_request(
    state_return: Arc<Mutex<GetStateResponse>>,
    remote_status_addr: tonic::transport::Uri,
) -> Result<(), Box<dyn std::error::Error>> {

        let mut client = match timeout(Duration::from_millis(500), GetStateClient::connect(remote_status_addr)).await {
            Ok(c) => c.unwrap(),
            Err(e) => return Err(Box::new(e)),
        };
     
        let mut request = tonic::Request::new(GetStateRequest {});
        request.set_timeout(Duration::from_secs(1));
        let response = client.get_state(request).await;
        debug!("Status Update RESPONSE={:#?}", response);
        debug!("Updating GetStateResponse: Aquire Lock");
        // Todo: Do something useful with the response
        state_return.lock().unwrap().prefered_serving_shard += 1;
        debug!("Updating GetStateResponse: Release Lock");
        Ok(())
}

async fn get_remote_state(
    config: StateConfiguration,
    state_return: Arc<Mutex<GetStateResponse>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Todo: Configure Poll Period
        sleep(Duration::from_millis(5000)).await;
        let remote_status_addr = format!(
            "https://[{}]:{}",
            config.direct_connect_target,
            u16::try_from(config.port).ok().unwrap()
        );
        info!("Request status update from {:#}", remote_status_addr);
        
        match make_remote_state_request(state_return.clone(), remote_status_addr.parse().unwrap()).await{
            Ok(_) => (info!("Remote State Updated.")),
            Err(_) => warn!("Remote State Failed! Going back to try again.")
        };

    }
}

async fn update_local_state(
    config: StateConfiguration,
    state_return: Arc<Mutex<GetStateResponse>>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("interrogate local system to determine local state");
    loop {
        // Todo: Configure Poll Period
        sleep(Duration::from_millis(5000)).await;
        let (direct, v4gw, v6gw) = tokio::join!(
            ping::ping(config.direct_connect_target.parse().unwrap()),
            ping::ping(config.gateway_v4_target.parse().unwrap()),
            ping::ping(config.gateway_v6_target.parse().unwrap())
        );
        debug!("Ping: Direct Connect Target Healthy: {}", direct);
        debug!("Ping: IPv4 Gateway Target Healthy: {}", v4gw);
        debug!("Ping: IPv6 Gateway Target Healthy: {}", v6gw);

        debug!("Updating GetStateResponse: Aquire Lock");
        let mut state_response_raw = state_return.lock().unwrap();
        state_response_raw.direct_connect_healthy = direct;
        state_response_raw.gateway_v4_healthy = v4gw;
        state_response_raw.gateway_v6_healthy = v6gw;
        // Force the response out of scope to realase the mutex
        // Todo: There is probably a block syntax for doing this that doesn't require the explicit descoping of state_response_raw
        drop(state_response_raw);
        debug!("Updating GetStateResponse: Release Lock");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Todo: Accept CLI flags to configure
    let config = StateConfiguration {
        allow_serve: true,
        prefered_serving_shard: 0,
        direct_connect_target: "::1".to_string(),
        gateway_v4_target: "192.168.2.3".to_string(),
        gateway_v6_target: "2001:4860:4860::8888".to_string(),
        port: 6969,
    };

    // Make response object available to server and update_state()
    let states_vec = GetStateResponse {
        ready_to_serve: true,
        prefered_serving_shard: 1,
        direct_connect_healthy: false,
        gateway_v4_healthy: false,
        gateway_v6_healthy: false,
        serving_shards: Vec::new(),
    };
    let states = Arc::new(Mutex::new(states_vec));

    let _result = tokio::join!(
        start_server(config.clone(), states.clone()),
        update_local_state(config.clone(), states.clone()),
        get_remote_state(config.clone(), states.clone())
    );
    // Todo: Work out how signal handlers work and give a conservative response before dieing

    Ok(())
}
