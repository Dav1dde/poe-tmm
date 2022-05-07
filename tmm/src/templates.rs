#[cfg(feature = "tree-3_17")]
mod tree3_17 {
    include!(concat!(env!("OUT_DIR"), "/tree3_17.rs"));
}

pub enum Version {
    #[cfg(feature = "tree-3_17")]
    V3_17,
}

pub struct Options {
    pub nodes: Vec<u16>,
}

pub fn render_svg(version: Version, options: Options) -> String {
    match version {
        #[cfg(feature = "tree-3_17")]
        Version::V3_17 => tree3_17::Tree {
            nodes: options.nodes,
        }
        .to_string(),
    }
}
