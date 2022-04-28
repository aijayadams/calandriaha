// If we can't connect, serve all configured shards as standalone.
// If we can connect, and the remove server is serving our shards
//   - Stop serving shards.
//   - Wait for remote server to stop serving out shards.
//   - Serve our shards.
// If we can connect, and the other server is ready to serve, stop serving it's prefered shards

use tokio::time::{sleep, Duration};

use tonic::transport::Server;
use tonic::{Request, Response, Status};

use calandria_ha::calha::calha;
use calandria_ha::ping;
use calha::get_state_server::{GetState, GetStateServer};
use calha::{GetStateRequest, GetStateResponse};

#[derive(Default)]
pub struct HAStateServer {}

fn build_response() -> GetStateResponse {
    return GetStateResponse {
        ready_to_serve: true,
        prefered_serving_shard: 0,
        direct_connect_healthy: true,
        gateway_v4_healthy: true,
        gateway_v6_healthy: false,
        serving_shards: [0].to_vec(),
    };
}

#[tonic::async_trait]
impl GetState for HAStateServer {
    async fn get_state(
        &self,
        _: Request<GetStateRequest>,
    ) -> Result<Response<GetStateResponse>, Status> {
        let response = build_response();
        Ok(Response::new(response))
    }
}


async fn start_server(addr: std::net::SocketAddr) -> Result<(), tonic::transport::Error> {
    let primary_state_server = HAStateServer::default();
    return Server::builder()
        .add_service(GetStateServer::new(primary_state_server))
        .serve(addr).await;

}

async fn print_stuff(){
    loop{
        sleep(Duration::from_millis(1000)).await;
        let (localhost, google, rubbish) = tokio::join!(
            ping::ping("::1".parse().unwrap()),
            ping::ping("2001:4860:4860::8888".parse().unwrap()),
            ping::ping("10.4.5.5".parse().unwrap())
        );
        
        println!("Ping {}, {}, {}", localhost, google, rubbish);

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "0.0.0.0:50051".parse()?;
        let _result = tokio::join!(
            start_server(addr),
            print_stuff());


    println!("Does execution get here?");

    Ok(())
}
