use surge_ping::{Client, Config, ICMP};
use tokio::time::Duration;

pub async fn ping(addr: std::net::IpAddr) -> bool {
    let mut config_builder = Config::builder();

    /*if let Some(interface) = opt.iface {
        config_builder = config_builder.interface(&interface);
    }*/

    println!("ping {}, v6? {}", addr, addr.is_ipv6());
    if addr.is_ipv6() {
        config_builder = config_builder.kind(ICMP::V6);
    }
    let config = config_builder.build();

    let client = Client::new(&config).await.unwrap();
    let mut pinger = client.pinger(addr).await;
    pinger
        .ident(1)
        .size(10)
        .timeout(Duration::from_millis(1000));
    match pinger.ping(0).await {
        Ok((packet, rtt)) => {
            println!("{:?} {:0.2?}", packet, rtt);
            return true;
        }

        Err(e) => {
            println!("{}", e);
            return false;
        }
    };
}
