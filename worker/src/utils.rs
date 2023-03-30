use worker::{Response, Result};

pub trait ResponseExt: Sized {
    fn cache_for(self, ttl: u32) -> Result<Self> {
        self.with_header("Cache-Control", &format!("max-age={ttl}"))
    }
    fn with_content_type(self, content_type: &str) -> Result<Self> {
        self.with_header("Content-Type", content_type)
    }
    fn with_etag(self, entity_id: &str) -> Result<Self> {
        let entity_id = format!("\"{}\"", entity_id.trim_matches('"'));
        self.with_header("Etag", &entity_id)
    }

    fn dup_headers(self) -> Self;
    fn with_header(self, name: &str, value: &str) -> Result<Self>;
}

impl ResponseExt for Response {
    fn dup_headers(self) -> Self {
        let headers = self.headers().clone();
        self.with_headers(headers)
    }

    fn with_header(mut self, name: &str, value: &str) -> Result<Self> {
        self.headers_mut().set(name, value)?;
        Ok(self)
    }
}
