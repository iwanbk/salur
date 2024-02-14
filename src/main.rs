use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() ->  String{
    let body = reqwest::get("https://httpbin.org/ip")
        .await.unwrap()
        .text()
        .await.unwrap();

    println!("body = {:?}", body);
    body
}