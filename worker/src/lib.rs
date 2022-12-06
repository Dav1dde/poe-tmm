use worker::{event, Cache, Context, Env, Method, Request, Response, Result, Url};

mod utils;

use utils::ResponseExt;

fn parse_options(url: Url, stu: tmm::SkillTreeUrl) -> tmm::Options {
    let mut options = tmm::Options {
        class: stu.class,
        ascendancy: stu.ascendancy,
        nodes: stu.nodes,
        ..Default::default()
    };

    for (k, v) in url.query_pairs() {
        if !v.chars().all(is_ascii_alphabetic_or_digit_or_hash) {
            continue;
        }

        match k.as_ref() {
            "backgroundColor" => options.background_color = Some(v.into_owned()),
            "color" => options.color = Some(v.into_owned()),
            "activeColor" => options.active_color = Some(v.into_owned()),
            "nodeColor" => options.node_color = Some(v.into_owned()),
            "nodeActiveColor" => options.node_active_color = Some(v.into_owned()),
            "keystoneColor" => options.keystone_color = Some(v.into_owned()),
            "keystoneActiveColor" => options.keystone_active_color = Some(v.into_owned()),
            "masteryColor" => options.mastery_color = Some(v.into_owned()),
            "masteryActiveColor" => options.mastery_active_color = Some(v.into_owned()),
            "connectionColor" => options.connection_color = Some(v.into_owned()),
            "connectionActiveColor" => options.connection_active_color = Some(v.into_owned()),
            _ => {}
        }
    }

    options
}

fn is_ascii_alphabetic_or_digit_or_hash(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '#'
}

#[event(fetch)]
pub async fn main(req: Request, _env: Env, ctx: Context) -> Result<Response> {
    if req.method() != Method::Get {
        return Response::error("Method not allowed", 405);
    }

    let cache = Cache::default();
    if let Some(response) = cache.get(&req, true).await? {
        return response
            .dup_headers() // cached response has immutable headers
            .with_header("Cf-Cache-Status", "HIT");
    }

    let path = req.path();
    let (version, stu) = match path.trim_start_matches('/').split_once('/') {
        Some((version, stu)) => (
            version.parse().unwrap_or_else(|_| tmm::Version::latest()),
            stu,
        ),
        _ => return Response::error("Not Found", 404),
    };

    let stu = match stu.parse::<tmm::SkillTreeUrl>() {
        Ok(stu) => stu,
        Err(_) => return Response::error("Invalid STU", 400),
    };

    let body = tmm::render_svg(version, parse_options(req.url()?, stu));

    let (response, response_for_cache) = Response::ok(body)?
        .with_content_type("image/svg+xml")?
        .cache_for(604800)? // 1 Week
        .cloned()?;

    ctx.wait_until(async move {
        let _ = cache.put(&req, response_for_cache).await;
    });

    response.with_header("Cf-Cache-Status", "MISS")
}
