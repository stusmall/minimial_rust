pub mod dao;
pub mod error;
pub mod handlers;

use crate::dao::Dao;
use crate::handlers::healthz::health_check;
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::routing::get;
use axum::Router;
use std::error::Error;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use crate::handlers::v1::ApiDoc;

pub async fn run<DAO: Dao>(dao: DAO, port: u16) -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug,warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        //TODO: move swagger the handler::v1 module
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1", handlers::v1::build_router::<DAO>())
        .with_state(dao)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);
                // If you want to add a unique request ID, this can be done here.
                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        // This route should be added after we set up tracing.  We don't want constant spam for healthz
        .route("/healthz", get(health_check));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
