use std::sync::Arc;

use axum::{
    Extension, Router,
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::MethodRouter,
};
use tokio::{signal, sync::RwLock};

use crate::router;

// Тип для хранения всех маршрутов
type RouteMap = Vec<Arc<RwLock<router::Route>>>;

// Создаём роутер с универсальным хендлером
pub async fn build(routes: RouteMap) -> Router {
    let mut router = Router::<()>::new();

    for route in routes.iter() {
        let route = route.read().await;

        router = router.route(
            &route.path,
            MethodRouter::new()
                .on(route.method.to_method_filter(), handler)
                .layer(Extension(route.clone())),
        );
    }

    router
}

async fn handler(Extension(route): Extension<router::Route>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::from_u16(route.status_code).unwrap_or(StatusCode::OK))
        .header("Content-Type", "application/json")
        .body(Body::from(route.response_body.clone().unwrap()))
        .unwrap()
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
