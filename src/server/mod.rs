mod api;
mod error;
mod state;
mod types;

use std::sync::Arc;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use tower_http::limit::RequestBodyLimitLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub use self::api::*;
pub use self::state::*;
pub use self::types::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        search_handler,
        reload_handler,
        add_image_handler,
        build_handler,
        stats_handler,
        reset_stats_handler,
        metrics_handler
    ),
    components(schemas(
        SearchForm,
        SearchResponse,
        ReloadRequest,
        AddImageForm,
        BuildRequest,
        StatsResponse
    )),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

/// 构建API服务器
pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/search", post(search_handler))
        .route("/reload", post(reload_handler))
        .route("/add", post(add_image_handler))
        .route("/build", post(build_handler))
        .route("/stats", get(stats_handler))
        .route("/reset_stats", post(reset_stats_handler))
        .route("/metrics", get(metrics_handler))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(DefaultBodyLimit::disable())
        // 上传限制：50M
        .layer(RequestBodyLimitLayer::new(1024 * 1024 * 50))
        .with_state(state)
}
