use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::engine::uds_client;
use core_lib::config::SubnetraConfig;

const CONFIG_PATH: &str = "config.json";

#[derive(Deserialize, Serialize)]
pub struct PolicyRequest {
    pub src: String,
    pub dst: String,
    pub action: String,
    pub target: u32,
}

/// GET /api/config
/// 读取底层 JSON 配置
#[handler]
pub async fn get_config(res: &mut Response) {
    match SubnetraConfig::from_file(CONFIG_PATH) {
        Ok(config) => {
            res.render(Json(config));
        }
        Err(e) => {
            // 如果不存在，返回一个默认的骨架配置
            let default_config = SubnetraConfig {
                mode: "raw_direct".to_string(),
                listen_port: 51820,
                tun: "snr0".to_string(),
                local_id: 1,
                public_key: "".to_string(),
                private_key: "".to_string(),
                preshared_key: None,
                peers: vec![],
            };
            res.render(Json(default_config));
        }
    }
}

/// POST /api/config
/// 写入底层 JSON 配置
#[handler]
pub async fn save_config(req: &mut Request, res: &mut Response) {
    match req.parse_json::<SubnetraConfig>().await {
        Ok(config) => {
            match config.save_to_file(CONFIG_PATH) {
                Ok(_) => {
                    res.render(Text::Json(json!({"status": "ok", "msg": "Config saved successfully"}).to_string()));
                }
                Err(e) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Text::Json(json!({"error": "Failed to save config", "details": e.to_string()}).to_string()));
                }
            }
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Text::Json(json!({"error": "Invalid config format", "details": e.to_string()}).to_string()));
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