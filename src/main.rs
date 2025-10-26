use axum::routing::get;
use axum::Router;
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::data::get_data_by_id,
        routes::test::test,
        routes::download::download
    ),
    components(schemas(routes::data::Person))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter(None, log::LevelFilter::Info)
            .init();
    }

    let app = Router::new()
        .route("/data/:id", get(routes::data::get_data_by_id))
        .route("/test", get(routes::test::test))
        .route("/download/:filename", get(routes::download::download))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:8080")
        .await
        .unwrap();

    info!("Listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(wait_for_signal())
        .await
        .unwrap();

    info!("Closing!");
}

async fn wait_for_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sigint = signal(SignalKind::interrupt()).unwrap();

        tokio::select! {
            _ = sigterm.recv() => info!("Received SIGTERM."),
            _ = sigint.recv() => info!("Received SIGINT."),
        }
    }

    #[cfg(windows)]
    {
        let _ = signal::ctrl_c().await;
        info!("Received Ctrl+C.");
    }
}
