slint::include_modules!();

use base64::{Engine as _, engine::general_purpose::STANDARD};
use core_lib::config::{PeerConfig, SubnetraConfig};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_connect_clicked({
        let ui_handle = ui_handle.clone();
        move |auth_key| {
            let ui = ui_handle.unwrap();
            
            if ui.get_is_connected() {
                ui.set_is_connected(false);
                ui.set_connection_status("已断开".into());
                ui.set_error_message("".into());
                return;
            }

            if auth_key.trim().is_empty() {
                ui.set_error_message("邀请码不能为空".into());
                return;
            }

            // 尝试解码 Base64 邀请码
            let decoded = match STANDARD.decode(auth_key.trim()) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(s) => s,
                    Err(_) => {
                        ui.set_error_message("邀请码格式错误 (非UTF-8)".into());
                        return;
                    }
                },
                Err(_) => {
                    ui.set_error_message("邀请码格式错误 (非Base64)".into());
                    return;
                }
            };

            // 解析 "hub_ip:listen_port|psk"
            let parts: Vec<&str> = decoded.split('|').collect();
            if parts.len() != 2 {
                ui.set_error_message("邀请码内容无效".into());
                return;
            }

            let endpoint = parts[0].to_string();
            let psk = parts[1].to_string();

            // 生成本地配置 config.json
            let config = SubnetraConfig {
                negotiation_version: 1,
                local_tun_mtu: 1360,
                listen_port: 51821, // 本地监听高端口
                virtual_subnet: "10.0.0.3/32".to_string(),
                local_tun_ip: "10.0.0.3/32".to_string(),
                local_id: 3, // 模拟分配为3
                role: "spoke".to_string(),
                local_routes: vec!["10.0.0.3/32".to_string()],
                remote_routes: vec!["10.0.0.0/24".to_string()],
                keepalive_secs: Some(20),
                peers: vec![PeerConfig {
                    id: 1, // Hub ID
                    name: "Hub-Node".to_string(),
                    endpoint,
                    allowed_src: "10.0.0.0/24".to_string(),
                    psk,
                }],
            };

            match config.save("config.json") {
                Ok(_) => {
                    ui.set_error_message("".into());
                    ui.set_is_connected(true);
                    ui.set_connection_status("已生成 config.json，准备就绪".into());
                    
                    // TODO: 在这里通过 std::process::Command 启动 Windows 版 subnetrad.exe 或执行相关隧道逻辑
                    // 目前演示配置已生成
                }
                Err(e) => {
                    ui.set_error_message(format!("保存配置失败: {}", e).into());
                }
            }
        }
    });

    ui.run()
}
