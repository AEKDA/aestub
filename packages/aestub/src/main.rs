use std::{net::SocketAddr, sync::Arc};

use axum::{
    Router,
    body::Body,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
    routing::any,
};
use router::Method;
use tokio::sync::RwLock;

mod router;

// Тип для хранения всех маршрутов
type RouteMap = Arc<RwLock<Vec<router::Route>>>;

// Создаём роутер с универсальным хендлером
fn build_router(routes: RouteMap) -> Router {
    Router::new().fallback(any(move |req| handle_request(req, routes.clone())))
}

async fn handle_request(req: Request<Body>, routes: RouteMap) -> impl IntoResponse {
    let method = req.method().clone();
    let path = req.uri().path();

    let routes = routes.read().await;

    // Ищем подходящий мок
    for route in routes.iter() {
        // FIXME
        if route.path == path && route.method == Method::from(method.as_str()).unwrap() {
            return Response::builder()
                .status(StatusCode::from_u16(route.status_code).unwrap_or(StatusCode::OK))
                .header("Content-Type", "application/json")
                .body(Body::from(route.response_body.clone().unwrap()))
                .unwrap();
        }
    }

    // Если не найден
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Route not found"))
        .unwrap()
}

#[tokio::main]
async fn main() {
    let input = include_str!("../../../examples/tmpjson.aestb");
    let routes = router::Route::from(input).expect("parse error");
    println!("{:#?}", routes);

    let routes = Arc::new(RwLock::new(routes));
    let app = build_router(routes);

    println!("Mock server running on http://{}", "0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
