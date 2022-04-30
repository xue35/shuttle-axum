use axum::{routing::get, Router,http::{StatusCode}};
use sync_wrapper::SyncWrapper;
use shuttle_service::Error;

async fn hello_world() -> &'static str {
    "Hello shuttle."
}

async fn index() -> StatusCode {
    StatusCode::NOT_FOUND
}

#[shuttle_service::main]
async fn axum() -> Result<SyncWrapper<Router>, shuttle_service::Error> {
    let r = Router::new()
            .route("/", get(index))
            .route("/hello", get(hello_world));

    let sync_wrapper = SyncWrapper::new(r);
    Ok(sync_wrapper)
}

