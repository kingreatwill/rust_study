use warp::Filter;
// cargo run --example warp
#[tokio::main]
async fn main() {
    // http://127.0.0.1:3030/hello/sdf
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    // http://127.0.0.1:3030/.within/health
    let health = warp::path!(".within" / "health")
        .map(|| "OK");
    let routes = hello.or(health);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
