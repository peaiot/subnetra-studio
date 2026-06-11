use salvo::prelude::*;
use salvo::cors::Cors;
use salvo::http::Method;
use clap::Parser;
use std::sync::OnceLock;

mod api;
mod engine;
mod init;

pub static IS_MOCK: OnceLock<bool> = OnceLock::new();

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 以模拟数据模式运行（用于无真实环境的测试）
    #[arg(long, default_value_t = false)]
    mock: bool,
}

#[handler]
async fn serve_index(res: &mut Response) {
    // 强制 Rust 编译器使 include_str! 缓存失效，重新打包最新的 index.html
    res.render(Text::Html(include_str!("../public/index.html")));
}

#[handler]
async fn serve_vue(res: &mut Response) {
    res.render(Text::Js(include_str!("../public/assets/vue.global.js")));
}

#[handler]
async fn serve_tailwind(res: &mut Response) {
    res.render(Text::Js(include_str!("../public/assets/tailwind.js")));
}

#[handler]
async fn serve_remix_css(res: &mut Response) {
    res.render(Text::Css(include_str!("../public/assets/remixicon.css")));
}

#[handler]
async fn serve_remix_woff2(res: &mut Response) {
    let bytes = include_bytes!("../public/assets/remixicon.woff2");
    res.add_header("content-type", "font/woff2", true).unwrap();
    res.write_body(bytes.to_vec()).unwrap();
}

#[handler]
async fn serve_remix_woff(res: &mut Response) {
    let bytes = include_bytes!("../public/assets/remixicon.woff");
    res.add_header("content-type", "font/woff", true).unwrap();
    res.write_body(bytes.to_vec()).unwrap();
}

#[handler]
async fn serve_remix_ttf(res: &mut Response) {
    let bytes = include_bytes!("../public/assets/remixicon.ttf");
    res.add_header("content-type", "font/ttf", true).unwrap();
    res.write_body(bytes.to_vec()).unwrap();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // 解析命令行参数并设置全局 mock 状态
    let args = Args::parse();
    IS_MOCK.set(args.mock).unwrap();
    if args.mock {
        println!("Starting in MOCK mode (simulated data).");
    } else {
        println!("Starting in PRODUCTION mode (real subnetrad interaction).");
        // 如果是生产环境，检查并执行“全自动管家”逻辑（自动初始化配置、自动安装引擎、自动拉起引擎）
        init::setup_and_start_environment(api::CONFIG_PATH, api::WEB_CONFIG_PATH);
    }

    // 1. 配置跨域 (CORS) 允许前端分离开发及 Nginx 代理
    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();

    // 2. API 路由分组 (只保留核心数据接口)
    let api_router = Router::new()
        .push(Router::with_path("login").post(api::login))
        .push(
            Router::new()
                .hoop(api::auth_middleware)
                .push(Router::with_path("status").get(api::get_status))
                .push(Router::with_path("config").get(api::get_config))
                .push(Router::with_path("logs").get(api::get_logs))
                .push(Router::with_path("auth-key").get(api::generate_auth_key))
                .push(
                    Router::with_path("policy")
                        .get(api::get_policies)
                        .post(api::add_policy)
                )
        );

    // 3. 挂载路由 (将静态资源和 API 分别挂载)
    let router = Router::new()
        .hoop(cors)
        .get(serve_index)
        .push(Router::with_path("assets/vue.global.js").get(serve_vue))
        .push(Router::with_path("assets/tailwind.js").get(serve_tailwind))
        .push(Router::with_path("assets/remixicon.css").get(serve_remix_css))
        .push(Router::with_path("assets/remixicon.woff2").get(serve_remix_woff2))
        .push(Router::with_path("assets/remixicon.woff").get(serve_remix_woff))
        .push(Router::with_path("assets/remixicon.ttf").get(serve_remix_ttf))
        .push(Router::with_path("api").push(api_router));

    // 4. 启动 Salvo API 服务器 (遵守规则：使用非常规高端口 25820 避免冲突)
    let acceptor = TcpListener::new("0.0.0.0:25820").bind().await;
    println!("Subnetra Web Console is running at http://0.0.0.0:25820");
    Server::new(acceptor).serve(router).await;
}