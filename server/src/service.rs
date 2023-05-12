use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use anyhow::Context;
use axum::{
    error_handling::HandleErrorLayer,
    http::{header, Method},
    routing::{get, get_service},
    Router,
};
use tower::ServiceBuilder;
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};

use crate::handler::{list_file, temp};

#[tokio::main(flavor = "multi_thread")]
pub async fn run() -> shared::Result<()> {
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .context("Failed to create rate rate limiter")?,
    );

    let rate_limit_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e| async move { display_error(e) }))
        .layer(GovernorLayer {
            config: Box::leak(governor_conf),
        });
    let cors = CorsLayer::new()
        .allow_headers(vec![
            header::ACCEPT,
            header::ACCEPT_LANGUAGE,
            header::AUTHORIZATION,
            header::CONTENT_LANGUAGE,
            header::CONTENT_TYPE,
            header::USER_AGENT,
        ])
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            // Method::PUT,
            // Method::DELETE,
            // Method::HEAD,
            // Method::OPTIONS,
            // Method::CONNECT,
            // Method::PATCH,
            // Method::TRACE,
        ])
        .allow_origin(Any);
    let app = Router::new()
        .nest_service("/", get_service(ServeDir::new("asset")))
        .route("/ls", get(list_file::ls))
        .route(
            "/api/chat-process",
            get(temp::temp).route_layer(rate_limit_layer.clone()),
        )
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let port = "8080".parse::<u16>()?;
    let addr_v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), port as _);
    log::info!("Listening on {addr_v6}");
    axum::Server::bind(&addr_v6)
        .serve(
            app.clone()
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .ok();

    log::debug!("Failed to start listening on ipv6, downgrade to ipv4");

    let addr_v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port as _);
    log::info!("Listening on {addr_v4}");
    axum::Server::bind(&addr_v4)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}
