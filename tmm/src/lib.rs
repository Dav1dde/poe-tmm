mod error;
mod stu;
mod templates;

pub use stu::SkillTreeUrl;

pub type Nodes = Vec<u16>;

pub enum Version {
    #[cfg(feature = "tree-3_15")]
    V3_15,
    #[cfg(feature = "tree-3_16")]
    V3_16,
    #[cfg(feature = "tree-3_17")]
    V3_17,
}

impl Version {
    pub fn latest() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "tree-3_17")] {
                Self::V3_17
            } else if #[cfg(feature = "tree-3_16")] {
                Self::V3_16
            } else if #[cfg(feature = "tree-3_15")] {
                Self::V3_15
            }
        }
    }
}

impl std::str::FromStr for Version {
    type Err = error::ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            #[cfg(feature = "tree-3_15")]
            "3.15" | "3_15" => Self::V3_15,
            #[cfg(feature = "tree-3_16")]
            "3.16" | "3_16" => Self::V3_16,
            #[cfg(feature = "tree-3_17")]
            "3.17" | "3_17" => Self::V3_17,
            _ => return Err(error::ParseVersionError {}),
        };

        Ok(r)
    }
}

#[derive(Default)]
pub struct Options {
    pub nodes: Nodes,
    pub background_color: Option<String>,
    pub color: Option<String>,
    pub active_color: Option<String>,
    pub node_color: Option<String>,
    pub node_active_color: Option<String>,
    pub connection_color: Option<String>,
    pub connection_active_color: Option<String>,
}

pub fn render_svg(version: Version, options: Options) -> String {
    match version {
        #[cfg(feature = "tree-3_15")]
        Version::V3_15 => templates::tree3_15::Tree::from(options).to_string(),
        #[cfg(feature = "tree-3_16")]
        Version::V3_16 => templates::tree3_16::Tree::from(options).to_string(),
        #[cfg(feature = "tree-3_17")]
        Version::V3_17 => templates::tree3_17::Tree::from(options).to_string(),
    }
}
