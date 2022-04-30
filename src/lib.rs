use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;
use shuttle_service::Error;

async fn hello_world() -> &'static str {
    "Hello shuttle."
}

#[shuttle_service::main]
async fn axum() -> Result<SyncWrapper<Router>, shuttle_service::Error> {
    let r = Router::new()
            .route("/hello", get(hello_world));
    let sync_wrapper = SyncWrapper::new(r);
    Ok(sync_wrapper)
}

