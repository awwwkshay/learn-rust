use axum::Router;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }));

    let addr = "0.0.0.0:3001";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, router).await.unwrap();
}
