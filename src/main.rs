#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/home", get(|| async { "Hello, world!" }));

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8080";
    let socket_addr = host.parse::<std::net::SocketAddr>().unwrap();
    // 绑定端口 启动服务
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
