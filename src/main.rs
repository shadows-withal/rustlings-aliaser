use axum::{Router};
use axum::extract::Host;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let host = option_env!("HOST").unwrap_or("127.0.0.1:3032");
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Host(host): Host) -> impl IntoResponse {
    match host.as_str() {
        "sh.rustlings.cool" => {
            Redirect::permanent("https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh").into_response()
        },
        "ps1.rustlings.cool" => {
            Redirect::permanent("https://raw.githubusercontent.com/rust-lang/rustlings/main/install.ps1").into_response()
        }
        _ => {
            StatusCode::NOT_FOUND.into_response()
        }
    }
}
