#[cfg(unix)]
use std::os::unix::net::UnixDatagram;
use std::io;
use crate::IS_MOCK;

#[cfg(unix)]
const SOCK_PATH: &str = "/var/run/subnetra.sock";

// 通用的 Mock 数据生成器
fn get_mock_status() -> String {
    r#"{
        "schema_version": 1,
        "version": "1.0.0",
        "mode": "raw_direct",
        "local_id": 1,
        "listen_port": 51820,
        "tun": "snr0",
        "peers": [
            {
                "id": 2,
                "name": "MacBook-Pro-CEO",
                "endpoint": "192.168.1.100:51820",
                "allowed_src": "10.0.0.2/32",
                "last_seen_wall_ns": 1700000000000000000,
                "last_seen_age_seconds": 5,
                "online": true
            },
            {
                "id": 3,
                "name": "办公室-开发服务器",
                "endpoint": "203.0.113.5:51820",
                "allowed_src": "10.0.0.3/32",
                "last_seen_wall_ns": 1700000000000000000,
                "last_seen_age_seconds": 12,
                "online": true
            },
            {
                "id": 4,
                "name": "家里电脑-游戏主机",
                "endpoint": "198.51.100.12:51820",
                "allowed_src": "10.0.0.4/32",
                "last_seen_wall_ns": 0,
                "last_seen_age_seconds": null,
                "online": false
            }
        ],
        "counters": {
            "tun_rx_packets": 1024,
            "tun_rx_bytes": 1048576,
            "udp_tx_packets": 1024,
            "udp_tx_bytes": 1048576
        }
    }"#.to_string()
}

#[cfg(unix)]
pub async fn fetch_status() -> io::Result<String> {
    if *IS_MOCK.get().unwrap_or(&false) {
        return Ok(get_mock_status());
    }
    // Unix 平台逻辑：使用绑定的 socket，因为引擎要求 addressable (bound) client
    let temp_sock = format!("/tmp/subnetra-web-status-{}.sock", std::process::id());
    let _ = std::fs::remove_file(&temp_sock);
    let socket = UnixDatagram::bind(&temp_sock)?;
    socket.connect(SOCK_PATH)?;
    
    // 设置超时以防止底层引擎不响应导致线程永久卡死
    socket.set_read_timeout(Some(std::time::Duration::from_secs(2)))?;
    
    let cmd = b"status --json\n";
    socket.send(cmd)?;
    let mut buf = vec![0u8; 32 * 1024];
    
    let result = socket.recv(&mut buf);
    let _ = std::fs::remove_file(&temp_sock);
    
    let len = result?;
    let response = String::from_utf8_lossy(&buf[..len]).to_string();
    Ok(response)
}

#[cfg(unix)]
pub async fn add_policy(src: &str, dst: &str, action: &str, target: u32) -> io::Result<()> {
    if *IS_MOCK.get().unwrap_or(&false) {
        println!("Mock policy add: src={} dst={} action={} target={}", src, dst, action, target);
        return Ok(());
    }
    // 真正的子进程调用 subnetrad CLI，不直接写文件
    let status = std::process::Command::new("subnetrad")
        .args(&["policy", "add", "--src", src, "--dst", dst, "--action", action, "--target", &target.to_string()])
        .status()?;
        
    if status.success() {
        // Trigger save via CLI
        let save_status = std::process::Command::new("subnetrad")
            .args(&["policy", "save"])
            .status()?;
        if save_status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to save policy via subnetrad CLI"))
        }
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to add policy via subnetrad CLI"))
    }
}

#[cfg(unix)]
pub async fn fetch_policies() -> io::Result<String> {
    if *IS_MOCK.get().unwrap_or(&false) {
        return Ok("10.0.0.0/24 -> 10.0.0.2/32 [forward target=2]\n".to_string());
    }
    let temp_sock = format!("/tmp/subnetra-web-policy-{}.sock", std::process::id());
    let _ = std::fs::remove_file(&temp_sock);
    let socket = UnixDatagram::bind(&temp_sock)?;
    socket.connect(SOCK_PATH)?;
    
    socket.set_read_timeout(Some(std::time::Duration::from_secs(2)))?;
    
    let cmd = b"policy show\n";
    socket.send(cmd)?;
    let mut buf = vec![0u8; 32 * 1024];
    let result = socket.recv(&mut buf);
    let _ = std::fs::remove_file(&temp_sock);
    
    let len = result?;
    let response = String::from_utf8_lossy(&buf[..len]).to_string();
    Ok(response)
}

#[cfg(not(unix))]
pub async fn fetch_status() -> io::Result<String> {
    if !*IS_MOCK.get().unwrap_or(&false) {
        return Err(io::Error::new(io::ErrorKind::Other, "Production mode (real UDS) is not supported on Windows. Please use --mock."));
    }
    Ok(get_mock_status())
}

#[cfg(not(unix))]
pub async fn add_policy(src: &str, dst: &str, action: &str, target: u32) -> io::Result<()> {
    if !*IS_MOCK.get().unwrap_or(&false) {
        return Err(io::Error::new(io::ErrorKind::Other, "Production mode (real UDS) is not supported on Windows. Please use --mock."));
    }
    println!("Mock policy add: src={} dst={} action={} target={}", src, dst, action, target);
    println!("Mock policy save triggered.");
    Ok(())
}

#[cfg(not(unix))]
pub async fn fetch_policies() -> io::Result<String> {
    if !*IS_MOCK.get().unwrap_or(&false) {
        return Err(io::Error::new(io::ErrorKind::Other, "Production mode (real UDS) is not supported on Windows. Please use --mock."));
    }
    Ok("10.0.0.0/24 -> 10.0.0.2/32 [forward target=2]\n".to_string())
}