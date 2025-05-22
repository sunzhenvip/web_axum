#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/home", get(|| async { "Hello, world!" }));
}
