use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let router = Router::new();
    let app = router.route("/", get(|| async {
        println!("get request");
        "Hello, World!"
    }));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:8088").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
