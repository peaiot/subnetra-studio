use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::engine::uds_client;
use core_lib::config::SubnetraConfig;
use crate::IS_MOCK;
use std::process::Command;

#[cfg(unix)]
pub const CONFIG_PATH: &str = "/etc/subnetra/config.json";
#[cfg(unix)]
pub const WEB_CONFIG_PATH: &str = "/etc/subnetra/web.json";

#[cfg(not(unix))]
pub const CONFIG_PATH: &str = "config.json";
#[cfg(not(unix))]
pub const WEB_CONFIG_PATH: &str = "web.json";


#[derive(Deserialize, Serialize)]
pub struct PolicyRequest {
    pub src: String,
    pub dst: String,
    pub action: String,
    pub target: u32,
}

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct AuthKeyResponse {
    pub auth_key: String,
}

/// GET /api/auth-key
/// 生成用于边缘节点准入的邀请码 (包含 Hub IP, 监听端口, PSK)
#[handler]
pub async fn generate_auth_key(res: &mut Response) {
    let mut hub_ip = "127.0.0.1".to_string();
    let mut listen_port = 51820;
    let mut psk = "".to_string();

    // 从 config.json 中读取真实信息
    if let Ok(config) = SubnetraConfig::from_file(CONFIG_PATH) {
        listen_port = config.listen_port;
        // 提取 dummy-peer 的 PSK
        if let Some(peer) = config.peers.first() {
            psk = peer.psk.clone();
        }
        
        // 尝试获取本机的公网或局域网 IP
        // 这里简单通过调用 curl ifconfig.me 获取，如果失败则回退到 127.0.0.1
        if let Ok(output) = std::process::Command::new("curl")
            .args(&["-s", "ifconfig.me"])
            .output() 
        {
            if output.status.success() {
                let ip = String::from_utf8_lossy(&output.stdout).to_string();
                if !ip.trim().is_empty() {
                    hub_ip = ip.trim().to_string();
                }
            }
        }
    }

    if psk.is_empty() {
        psk = "fallback_psk".to_string();
    }

    let raw = format!("{}:{}|{}", hub_ip, listen_port, psk);
    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, raw.as_bytes());

    res.render(Text::Json(json!({"auth_key": b64}).to_string()));
}

/// POST /api/login
/// 验证密码并返回 Token
#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    if let Ok(body) = req.parse_json::<LoginRequest>().await {
        // 读取 Web 端专属的 web.json 里的密码，或者默认 admin
        let mut expected_password = "admin".to_string();
        if let Ok(web_cfg_content) = std::fs::read_to_string(WEB_CONFIG_PATH) {
            if let Ok(web_cfg) = serde_json::from_str::<serde_json::Value>(&web_cfg_content) {
                if let Some(pass) = web_cfg.get("admin_password").and_then(|v| v.as_str()) {
                    expected_password = pass.to_string();
                }
            }
        }

        if body.password == expected_password {
            res.render(Text::Json(json!({"token": "subnetra-admin-token"}).to_string()));
            return;
        }
    }
    res.status_code(StatusCode::UNAUTHORIZED);
    res.render(Text::Json(json!({"error": "Invalid password"}).to_string()));
}

/// 拦截器：验证 Authorization Token
#[handler]
pub async fn auth_middleware(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(auth) = req.header::<String>("authorization") {
        if auth == "Bearer subnetra-admin-token" {
            ctrl.call_next(req, depot, res).await;
            return;
        }
    }
    res.status_code(StatusCode::UNAUTHORIZED);
    res.render(Text::Json(json!({"error": "Unauthorized"}).to_string()));
    ctrl.skip_rest();
}

/// GET /api/config
/// 读取底层 JSON 配置
#[handler]
pub async fn get_config(res: &mut Response) {
    if *IS_MOCK.get().unwrap_or(&false) {
        let dummy = SubnetraConfig {
            negotiation_version: 1,
            local_tun_mtu: 1360,
            local_id: 1,
            listen_port: 51820,
            virtual_subnet: "10.0.0.0/24".to_string(),
            local_tun_ip: "".to_string(),
            role: "hub".to_string(),
            local_routes: vec![],
            remote_routes: vec![],
            keepalive_secs: None,
            peers: vec![],
        };
        res.render(Text::Json(serde_json::to_string(&dummy).unwrap()));
        return;
    }

    match SubnetraConfig::from_file(CONFIG_PATH) {
        Ok(config) => res.render(Text::Json(serde_json::to_string(&config).unwrap())),
        Err(_) => {
            let default_config = SubnetraConfig {
                negotiation_version: 1,
                local_tun_mtu: 1360,
                local_id: 1,
                listen_port: 51820,
                virtual_subnet: "10.0.0.0/24".to_string(),
                local_tun_ip: "".to_string(),
                role: "hub".to_string(),
                local_routes: vec![],
                remote_routes: vec![],
                keepalive_secs: None,
                peers: vec![],
            };
            res.render(Text::Json(serde_json::to_string(&default_config).unwrap()));
        }
    }
}

/// GET /api/status
/// 读取底层引擎的 JSON 状态
#[handler]
pub async fn get_status(res: &mut Response) {
    match uds_client::fetch_status().await {
        Ok(status_json) => {
            // 直接将从 uds 读取的 json 字符串渲染出去，设置 Content-Type 为 application/json
            res.render(Text::Json(status_json));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Text::Json(json!({
                "error": "Failed to connect to subnetrad engine",
                "details": e.to_string()
            }).to_string()));
        }
    }
}

/// GET /api/policy
/// 获取引擎内存中的路由策略
#[handler]
pub async fn get_policies(res: &mut Response) {
    match uds_client::fetch_policies().await {
        Ok(policies) => {
            res.render(Text::Plain(policies));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Text::Json(json!({"error": "Failed to fetch policies", "details": e.to_string()}).to_string()));
        }
    }
}

/// GET /api/logs
/// 获取后台日志 (真实读取 journalctl 或 mock)
#[handler]
pub async fn get_logs(res: &mut Response) {
    if *IS_MOCK.get().unwrap_or(&false) {
        let mock_logs = r#"
2026-06-10 09:00:00 [INFO] Subnetrad engine started v1.0.0 (MOCK)
2026-06-10 09:00:01 [INFO] Loaded config from /etc/subnetra/config.json
2026-06-10 09:00:01 [INFO] Listening on UDP 0.0.0.0:51820
2026-06-10 09:01:23 [INFO] Handshake complete with peer 2 (MacBook-Pro-CEO)
2026-06-10 09:15:45 [WARN] Peer 4 connection timeout
2026-06-10 10:30:12 [INFO] Handshake complete with peer 3 (办公室-开发服务器)
"#;
        res.render(Text::Plain(mock_logs));
        return;
    }

    // 生产环境：尝试通过 journalctl 读取 subnetra 服务日志
    match Command::new("journalctl")
        .args(&["-u", "subnetrad", "-n", "100", "--no-pager"])
        .output()
    {
        Ok(output) => {
            let logs = String::from_utf8_lossy(&output.stdout).to_string();
            if logs.trim().is_empty() {
                res.render(Text::Plain("No logs found for subnetrad service via journalctl."));
            } else {
                res.render(Text::Plain(logs));
            }
        }
        Err(e) => {
            res.render(Text::Plain(format!("Failed to read logs: {}", e)));
        }
    }
}
/// POST /api/policy
/// 下发路由策略到引擎
#[handler]
pub async fn add_policy(req: &mut Request, res: &mut Response) {
    match req.parse_json::<PolicyRequest>().await {
        Ok(policy) => {
            match uds_client::add_policy(&policy.src, &policy.dst, &policy.action, policy.target).await {
                Ok(_) => {
                    res.render(Text::Json(json!({"status": "ok", "msg": "Policy added and saved"}).to_string()));
                }
                Err(e) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Text::Json(json!({"error": "Failed to add policy", "details": e.to_string()}).to_string()));
                }
            }
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Text::Json(json!({"error": "Invalid policy format", "details": e.to_string()}).to_string()));
        }
    }
}