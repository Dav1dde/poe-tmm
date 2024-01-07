use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub out: std::path::PathBuf,
    pub tree: Vec<Tree>,
}

#[derive(Debug, Deserialize)]
pub struct Tree {
    pub name: String,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Deserialize)]
pub enum Location {
    #[serde(rename = "url")]
    Url(String),
    #[serde(rename = "path")]
    Path(std::path::PathBuf),
}

impl Location {
    pub fn read(&self) -> anyhow::Result<String> {
        match self {
            Location::Url(url) => Ok(ureq::get(url).call()?.into_string()?),
            Location::Path(path) => Ok(std::fs::read_to_string(path)?),
        }
    }
}
