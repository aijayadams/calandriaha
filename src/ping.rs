use tokio::time::Duration;

pub async fn ping(addr: std::net::IpAddr) -> bool {
    let pinger = tokio_icmp_echo::Pinger::new().await.unwrap();
    match pinger.ping(addr, 6969, 1, Duration::from_millis(500)).await {
        Ok(Some(_)) => return true,
        Ok(..) => return false,
        Err(_) => return false,
    };
}
