cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use axum::{
        body::{boxed, Body, BoxBody},
        extract::State,
        http::{Request, Response, StatusCode, Uri},
        response::IntoResponse,
        response::Response as AxumResponse,
        routing::post,
        Router,
    };
use tower::ServiceExt;
use tower_http::services::ServeDir;
use sqlx::SqlitePool;
use leptos::*;
use leptos::logging::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

use rustsweeper::app::App;

const DATABASE_URL: &str = "";

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).expect("logging initializes");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let _pool = sqlx::SqlitePool::connect(DATABASE_URL).await.expect("sqlite ready for connections");

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("axum server binds to addr");
}

pub async fn file_and_error_handler(uri: Uri, State(options): State<LeptosOptions>, req: Request<Body>) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream(options.to_owned(), move || view!{ <App />});
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}

} else {

pub fn main() {
}

}}
