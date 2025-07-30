use std::{sync::Arc, time::Duration};

use anyhow::Context;
use axum::{
    extract::{MatchedPath, Path, State},
    http::{Request, Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use opentelemetry::metrics::Histogram;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing::{Span, debug, info, info_span, instrument, trace, warn};

use crate::{
    business::{NewUser, ReadUser, UserManager},
    cfg::Cfg,
};

pub async fn host_server(cfg: Cfg) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", cfg.port);

    trace!("Trying to bind to local port on {addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("could not create TCP listener for web server")?;

    info!(
        "Web server will be listening on {}",
        listener
            .local_addr()
            .context("Cannot access address of local web server socket")?
    );

    let user_manager = Arc::new(Mutex::new(UserManager::new()));

    let app = axum::Router::new()
        .route("/users/add/{name}", post(add_user))
        .route("/users/read/{name}", get(read_user))
        .with_state(user_manager)
        .route("/hello", get(hello_route))
        // -- Create a tracing layer that generates nicely formatted HTTP traces-
        // -- The logic displays how to fill a custom `correlation_id` field on the automatically
        // -- created spans.
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        correlation_id = tracing::field::Empty, // <-- Create an empty field on the span here
                    )
                })
                .on_request(|request: &Request<_>, span: &Span| {
                    let id = request
                        .headers()
                        .get("correlation_id")
                        .and_then(|a| a.to_str().ok());
                    // --> Fill the empty "correlation_id" span with
                    // a) the value included in the "correlation_id" header or
                    // b) with a newly generated ID.
                    if let Some(id) = id {
                        span.record("correlation_id", id);
                    } else {
                        let id = uuid::Uuid::new_v4().to_string();
                        span.record("correlation_id", id);
                    }
                })
                .on_response(|_response: &Response<_>, latency: Duration, _span: &Span| {
                    // The opentelemetry sdk docs specifically advice against creating an instrument like this
                    // within a hot loop. In a real application, we would instantiate the histogram once.
                    let histogram = build_latency_histogram();
                    histogram.record(latency.as_micros() as f64, &[]);
                    debug!("latency micros: {:#?}", latency.as_micros());
                }),
        );

    axum::serve(listener, app).await.context("server shut down")
}

#[instrument(name = "my_hello_span", level = tracing::Level::WARN)]
async fn hello_route() -> &'static str {
    info!("Within hello route");
    trace!("Try applying `trace` level filtering to only this function :-)");
    "hello"
}

#[instrument(skip(user_manager))]
async fn add_user(
    State(user_manager): State<Arc<Mutex<UserManager>>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    info!("Create new user with name {name}...");

    if let Err(e) = user_manager.lock().await.create(NewUser::new(&name)) {
        warn!("Could not create user with name {name}:\n{e:?}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

#[instrument(skip(user_manager), fields(user_uuid))]
async fn read_user(
    State(user_manager): State<Arc<Mutex<UserManager>>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    info!("Read user with name {name}...");

    let user = match user_manager.lock().await.read_by_name(ReadUser::new(&name)) {
        Ok(Some(user)) => user,
        Ok(None) => return Ok((StatusCode::NO_CONTENT, format!("no user found"))),
        Err(e) => {
            warn!("Could not read user with name {name}:\n{e:?}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // add uuid to span
    tracing::Span::current().record("user_uuid", user.id.to_string());

    Ok((StatusCode::OK, format!("{}:{}", user.id, user.name)))
}

/// Build a new instance of the histogram, that records latencies of our HTTP requests.
fn build_latency_histogram() -> Histogram<f64> {
    let meter = opentelemetry::global::meter("server_measurements");
    meter
        .f64_histogram("http.server.latency")
        .with_description("Latency of HTTP requests")
        .with_unit("us")
        .build()
}
