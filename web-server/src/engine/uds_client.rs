#[cfg(unix)]
use std::os::unix::net::UnixDatagram;
use std::io;

#[cfg(unix)]
const SOCK_PATH: &str = "/var/run/subnetra.sock";

#[cfg(unix)]
pub async fn fetch_status() -> io::Result<String> {
    // Unix 平台逻辑
    let socket = UnixDatagram::unbound()?;
    socket.connect(SOCK_PATH)?;
    let cmd = b"status --json\n";
    socket.send(cmd)?;
    let mut buf = vec![0u8; 32 * 1024]; 
    let len = socket.recv(&mut buf)?;
    let response = String::from_utf8_lossy(&buf[..len]).to_string();
    Ok(response)
}

#[cfg(unix)]
pub async fn add_policy(src: &str, dst: &str, action: &str, target: u32) -> io::Result<()> {
    let socket = UnixDatagram::unbound()?;
    socket.connect(SOCK_PATH)?;
    // command format: policy add --src <src> --dst <dst> --action <action> --target <target>
    let cmd = format!("policy add --src {} --dst {} --action {} --target {}\n", src, dst, action, target);
    socket.send(cmd.as_bytes())?;
    
    // Trigger save to policy file
    let save_cmd = b"save\n";
    socket.send(save_cmd)?;
    Ok(())
}

#[cfg(unix)]
pub async fn fetch_policies() -> io::Result<String> {
    let socket = UnixDatagram::unbound()?;
    socket.connect(SOCK_PATH)?;
    let cmd = b"policy show\n";
    socket.send(cmd)?;
    let mut buf = vec![0u8; 32 * 1024]; 
    let len = socket.recv(&mut buf)?;
    let response = String::from_utf8_lossy(&buf[..len]).to_string();
    Ok(response)
}

#[cfg(not(unix))]
pub async fn fetch_status() -> io::Result<String> {
    // Windows/Mock 逻辑：因为底层引擎运行在 Linux/RouterOS
    // Windows 开发环境下直接返回 Mock JSON 以供前端调试
    Ok(r#"{
        "schema_version": 1,
        "version": "1.0.0",
        "mode": "raw_direct",
        "local_id": 1,
        "listen_port": 51820,
        "tun": "snr0",
        "peers": [
            {
                "id": 2,
                "name": "mock-spoke",
                "endpoint": "192.168.1.100:51820",
                "allowed_src": "10.0.0.2/32",
                "last_seen_wall_ns": 1700000000000000000,
                "last_seen_age_seconds": 5,
                "online": true
            }
        ],
        "counters": {
            "tun_rx_packets": 1024,
            "tun_rx_bytes": 1048576,
            "udp_tx_packets": 1024,
            "udp_tx_bytes": 1048576
        }
    }"#.to_string())
}

#[cfg(not(unix))]
pub async fn add_policy(src: &str, dst: &str, action: &str, target: u32) -> io::Result<()> {
    // Windows/Mock logic
    println!("Mock policy add: src={} dst={} action={} target={}", src, dst, action, target);
    println!("Mock policy save triggered.");
    Ok(())
}

#[cfg(not(unix))]
pub async fn fetch_policies() -> io::Result<String> {
    // Return a mock policy list string
    Ok("10.0.0.0/24 -> 10.0.0.2/32 [forward target=2]\n".to_string())
}