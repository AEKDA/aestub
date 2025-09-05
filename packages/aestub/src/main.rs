use std::{net::ToSocketAddrs, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

mod config;
mod handlers;
mod router;

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let input = std::fs::read_to_string(config.file).unwrap();
    let routes = router::Route::from(&input).expect("file parse error");
    println!("{:#?}", routes);

    let routes = routes
        .iter()
        .cloned()
        .map(|e| Arc::new(RwLock::new(e)))
        .collect();
    let app = handlers::build(routes).await;

    let address = (config.host, config.port)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    println!("Mock server running on http://{:?}", &address);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
