use std::sync::Arc;

use axum::{
    Extension, Router,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::MethodRouter,
};
use tokio::sync::RwLock;

mod router;

// Тип для хранения всех маршрутов
type RouteMap = Vec<Arc<RwLock<router::Route>>>;

// Создаём роутер с универсальным хендлером
async fn build_router(routes: RouteMap) -> Router {
    let mut router = Router::<()>::new();

    for route in routes.iter() {
        let route = route.read().await;

        router = router.route(
            &route.path,
            MethodRouter::new()
                .on(route.method.to_method_filter(), tmp_handler)
                .layer(Extension(route.clone())),
        );
    }

    router
}

async fn tmp_handler(Extension(route): Extension<router::Route>) -> impl IntoResponse {
    return Response::builder()
        .status(StatusCode::from_u16(route.status_code).unwrap_or(StatusCode::OK))
        .header("Content-Type", "application/json")
        .body(Body::from(route.response_body.clone().unwrap()))
        .unwrap();
}

#[tokio::main]
async fn main() {
    let input = include_str!("../../../examples/tmpjson.aestb");
    let routes = router::Route::from(input).expect("parse error");
    println!("{:#?}", routes);

    let routes = routes
        .iter()
        .cloned()
        .map(|e| Arc::new(RwLock::new(e)))
        .collect();
    let app = build_router(routes).await;

    println!("Mock server running on http://{}", "0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
