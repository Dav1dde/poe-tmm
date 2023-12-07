use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    http::Response,
    response::IntoResponse,
    routing::get,
    Router,
};
use bpaf::Bpaf;
use serde::Deserialize;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Args {
    #[bpaf(env("LISTEN_ADDR"), fallback(SocketAddr::from(([127, 0, 0, 1], 3000))))]
    addr: SocketAddr,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().run();

    tracing_subscriber::fmt::init();

    let app = Router::new().route("/:version/:stu", get(tree));

    tracing::info!("listening on {}", args.addr);
    axum::Server::bind(&args.addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(elegant_departure::tokio::depart().on_termination())
        .await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TreeOptions {
    background_color: Option<Color>,
    color: Option<Color>,
    active_color: Option<Color>,
    node_color: Option<Color>,
    node_active_color: Option<Color>,
    connection_color: Option<Color>,
    connection_active_color: Option<Color>,
}

#[derive(Debug)]
struct Color(String);

impl From<Color> for String {
    fn from(color: Color) -> Self {
        color.0
    }
}

impl std::ops::Deref for Color {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let color = String::deserialize(deserializer)?;
        let valid = color
            .chars()
            .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '#');
        if !valid {
            return Err(serde::de::Error::custom("invalid color"));
        }

        Ok(Color(color))
    }
}

async fn tree(
    Path((version, stu)): Path<(tmm::Version, tmm::SkillTreeUrl)>,
    Query(query): Query<TreeOptions>,
) -> impl IntoResponse {
    tracing::debug!("attempting to render tree version {version:?}: {stu:?}");

    let options = tmm::Options {
        class: stu.class,
        ascendancy: stu.ascendancy,
        alternate_ascendancy: stu.alternate_ascendancy,
        nodes: stu.nodes,
        background_color: query.background_color.map(Into::into),
        color: query.color.map(Into::into),
        active_color: query.active_color.map(Into::into),
        node_color: query.node_color.map(Into::into),
        node_active_color: query.node_active_color.map(Into::into),
        connection_color: query.connection_color.map(Into::into),
        connection_active_color: query.connection_active_color.map(Into::into),
    };

    let svg = tmm::render_svg(version, options);

    Response::builder()
        .status(200)
        .header("Content-Type", "image/svg+xml")
        .header("Cache-Control", "max-age=604800")
        .body(svg)
        .unwrap()
}
