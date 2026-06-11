use core_lib::config::{PeerConfig, SubnetraConfig};
use rand::Rng;
use serde_json::json;
use std::fs;
use std::path::Path;

pub fn init_config_if_not_exists(config_path: &str, web_config_path: &str) {
    // 检查并创建 web 配置文件
    if !Path::new(web_config_path).exists() {
        if let Some(parent) = Path::new(web_config_path).parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("❌ 严重错误: 无法创建配置目录 {}。请检查是否有足够的权限 (请使用 sudo 运行): {}", parent.display(), e);
            }
        }
        
        let web_cfg = json!({
            "admin_password": "admin"
        });
        
        if let Err(e) = fs::write(
            web_config_path,
            serde_json::to_string_pretty(&web_cfg).unwrap(),
        ) {
            eprintln!("❌ 严重错误: 无法写入配置文件 {}。请使用 sudo 运行: {}", web_config_path, e);
        } else {
            println!("Created default web config at {}", web_config_path);
        }
    }

    // 检查并创建核心引擎配置文件
    if !Path::new(config_path).exists() {
        if let Some(parent) = Path::new(config_path).parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("❌ 严重错误: 无法创建配置目录 {}。请检查是否有足够的权限 (请使用 sudo 运行): {}", parent.display(), e);
            }
        }

        // 生成随机 32 字节的 hex 字符串作为 dummy peer 的 PSK (长度为 64 个字符)
        let mut rng = rand::rng();
        let mut secret_bytes = [0u8; 32];
        rng.fill_bytes(&mut secret_bytes);
        let psk_hex: String = secret_bytes.iter().map(|b| format!("{:02x}", b)).collect();

        let default_config = SubnetraConfig {
            negotiation_version: 1,
            local_tun_mtu: 1360,
            listen_port: 51820,
            virtual_subnet: "10.0.0.0/24".to_string(),
            local_tun_ip: "".to_string(),
            local_id: 1,
            role: "hub".to_string(),
            local_routes: vec![],
            remote_routes: vec![],
            keepalive_secs: None,
            peers: vec![
                PeerConfig {
                    id: 2,
                    name: "dummy-peer-for-startup".to_string(),
                    endpoint: "127.0.0.1:51821".to_string(),
                    allowed_src: "10.0.0.2/32".to_string(),
                    psk: psk_hex,
                }
            ],
        };

        if let Err(e) = fs::write(
            config_path,
            serde_json::to_string_pretty(&default_config).unwrap(),
        ) {
            eprintln!("❌ 严重错误: 无法写入配置文件 {}。请使用 sudo 运行: {}", config_path, e);
        } else {
            println!("Created default subnetrad config at {}", config_path);
        }
    }
}

#[cfg(unix)]
pub fn setup_and_start_environment(config_path: &str, web_config_path: &str) {
    // 1. 初始化配置文件
    init_config_if_not_exists(config_path, web_config_path);

    // 2. 检查核心引擎是否安装
    println!("🔍 [自动管家] 检查核心引擎 (subnetrad) 安装状态...");
    let is_installed = std::process::Command::new("subnetrad")
        .arg("--version")
        .output()
        .is_ok() || Path::new("/usr/local/bin/subnetrad").exists();

    if !is_installed {
        println!("📦 [自动管家] 未检测到 subnetrad，正在全自动下载并安装...");
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg("curl -fsSL https://down.16861.com/subnetra/install.sh | sh -s -- --yes")
            .status();
        match status {
            Ok(s) if s.success() => println!("✅ [自动管家] subnetrad 安装成功!"),
            _ => eprintln!("❌ [自动管家] subnetrad 安装失败，请检查网络或权限。"),
        }
    } else {
        println!("✅ [自动管家] subnetrad 已安装。");
    }

    // 3. 检查核心引擎运行状态，如果没有运行则自动拉起
    println!("🚀 [自动管家] 检查核心引擎运行状态...");
    let sock_path = "/var/run/subnetra.sock";
    let is_running = if Path::new(sock_path).exists() {
        if let Ok(socket) = std::os::unix::net::UnixDatagram::unbound() {
            socket.connect(sock_path).is_ok()
        } else {
            false
        }
    } else {
        false
    };

    if !is_running {
        println!("⚙️ [自动管家] 核心引擎未运行，正在后台启动 subnetrad...");
        let exe_path = if Path::new("/usr/local/bin/subnetrad").exists() {
            "/usr/local/bin/subnetrad"
        } else {
            "subnetrad"
        };

        match std::process::Command::new(exe_path)
            .arg("--config")
            .arg(config_path)
            .spawn()
        {
            Ok(child) => {
                println!("✅ [自动管家] subnetrad 已成功拉起 (PID: {})！", child.id());
                // 等待 1 秒，让引擎有时间初始化并创建通信 socket
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(e) => eprintln!("❌ [自动管家] 启动 subnetrad 失败: {}", e),
        }
    } else {
        println!("✅ [自动管家] subnetrad 引擎已在后台运行中。");
    }
}

#[cfg(not(unix))]
pub fn setup_and_start_environment(config_path: &str, web_config_path: &str) {
    init_config_if_not_exists(config_path, web_config_path);
    println!("⚠️ [自动管家] 当前非 Unix 环境，跳过核心引擎的自动化安装和拉起。");
}
