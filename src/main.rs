cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
use axum::{
    body::{boxed, Body, BoxBody},
    extract::{Path, State, RawQuery, FromRef},
    http::{Request, Response, StatusCode, Uri, header::HeaderMap},
    response::IntoResponse,
    response::Response as AxumResponse,
    routing::get,
    Router,
};
use leptos::logging::*;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::SqlitePool;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use rustsweeper::app::App;

#[derive(FromRef, Debug, Clone)]
struct AppState {
    leptos_options: LeptosOptions,
    db_pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).expect("logging initializes");

    let leptos_options = get_configuration(None)
        .await
        .expect("configuration exists")
        .leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let db_url = dotenvy::var("DATABASE_URL").expect(".env exists");
    let db_pool = SqlitePool::connect(&db_url)
        .await
        .expect("sqlite ready for connections");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("database migrated");

    let state = {
        let db_pool = db_pool.clone();
        AppState {
            leptos_options,
            db_pool,
        }
    };

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_context(&state, routes, move || {
            provide_context(db_pool.clone());
        }, App)
        .fallback(file_and_error_handler)
        .with_state(state);

    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("axum server binds to addr");
}

async fn server_fn_handler(
    State(db_pool): State<SqlitePool>,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<Body>,
) -> impl IntoResponse {
    leptos_axum::handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move || {
            provide_context(db_pool.clone());
        },
        request,
    )
    .await
}

async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler =
            leptos_axum::render_app_to_stream(options.to_owned(), move || view! { <App />});
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}

} else {

pub fn main() {}

}}
