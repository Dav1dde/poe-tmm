mod stu;
mod templates;

pub use stu::SkillTreeUrl;

pub type Nodes = Vec<u16>;

pub enum Version {
    #[cfg(feature = "tree-3_17")]
    V3_17,
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
        #[cfg(feature = "tree-3_17")]
        Version::V3_17 => templates::tree3_17::Tree::from(options).to_string(),
    }
}
