use salvo::prelude::*;
use salvo::cors::Cors;
use salvo::http::Method;

mod api;
mod engine;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // 1. 配置跨域 (CORS) 允许前端分离开发及 Nginx 代理
    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();

    // 2. API 路由分组 (只保留核心数据接口)
    let api_router = Router::new()
        .push(Router::with_path("status").get(api::get_status))
        .push(
            Router::with_path("config")
                .get(api::get_config)
                .post(api::save_config),
        )
        .push(
            Router::with_path("policy")
                .get(api::get_policies)
                .post(api::add_policy)
        );

    // 3. 挂载路由
    let router = Router::new()
        .hoop(cors)
        .push(Router::with_path("api").push(api_router));

    // 4. 启动 Salvo API 服务器
    let acceptor = TcpListener::new("0.0.0.0:8080").bind().await;
    println!("Subnetra Web API is running at http://0.0.0.0:8080");
    Server::new(acceptor).serve(router).await;
}