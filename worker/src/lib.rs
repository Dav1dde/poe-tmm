use worker::{event, Context, Env, Headers, Method, Request, Response, Result};

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    if req.method() != Method::Get {
        return Response::error("Method not allowed", 405);
    }

    let path = req.path();
    let (version, stu) = match path.trim_start_matches('/').split_once('/') {
        Some(("3.17", stu)) => {
            let stu = match stu.parse::<tmm::SkillTreeUrl>() {
                Ok(stu) => stu,
                Err(_) => return Response::error("Invalid STU", 400),
            };
            (tmm::Version::V3_17, stu)
        }
        _ => {
            return Response::error("Not Found", 404);
        }
    };

    let body = tmm::render_svg(version, tmm::Options { nodes: stu.nodes });
    let mut headers = Headers::new();
    headers.set("Content-Type", "image/svg+xml")?;
    Ok(Response::ok(body)?.with_headers(headers))
}
