use std::borrow::Cow;

#[cfg(feature = "tree-3_15")]
pub(crate) mod tree3_15 {
    include!(concat!(env!("OUT_DIR"), "/tree3_15.rs"));
}
#[cfg(feature = "tree-3_16")]
pub(crate) mod tree3_16 {
    include!(concat!(env!("OUT_DIR"), "/tree3_16.rs"));
}
#[cfg(feature = "tree-3_17")]
pub(crate) mod tree3_17 {
    include!(concat!(env!("OUT_DIR"), "/tree3_17.rs"));
}
#[cfg(feature = "tree-3_18")]
pub(crate) mod tree3_18 {
    include!(concat!(env!("OUT_DIR"), "/tree3_18.rs"));
}
#[cfg(feature = "tree-3_19")]
pub(crate) mod tree3_19 {
    include!(concat!(env!("OUT_DIR"), "/tree3_19.rs"));
}

const DEFAULT_BACKGROUND_COLOR: Cow<'static, str> = Cow::Borrowed("rgba(0, 0, 0, 0)");
const DEFAULT_COLOR: Cow<'static, str> = Cow::Borrowed("#64748b");
const DEFAULT_ACTIVE_COLOR: Cow<'static, str> = Cow::Borrowed("#0ea5e9");
const DEFAULT_ACTIVE_NODE_COLOR: Cow<'static, str> = Cow::Borrowed("#38bdf8");

macro_rules! template_impl {
    ($name:ident) => {
        impl $name {}

        impl From<crate::Options> for $name {
            fn from(options: crate::Options) -> $name {
                let background_color = options
                    .background_color
                    .map(std::borrow::Cow::Owned)
                    .unwrap_or(DEFAULT_BACKGROUND_COLOR);

                let node_color = options
                    .node_color
                    .or_else(|| options.color.clone())
                    .map(std::borrow::Cow::Owned)
                    .unwrap_or(DEFAULT_COLOR);

                let node_active_color = options
                    .node_active_color
                    .or_else(|| options.active_color.clone())
                    .map(std::borrow::Cow::Owned)
                    .unwrap_or(DEFAULT_ACTIVE_NODE_COLOR);

                let connection_color = options
                    .connection_color
                    .or(options.color)
                    .map(std::borrow::Cow::Owned)
                    .unwrap_or(DEFAULT_COLOR);

                let connection_active_color = options
                    .connection_active_color
                    .or(options.active_color)
                    .map(std::borrow::Cow::Owned)
                    .unwrap_or(DEFAULT_ACTIVE_COLOR);

                $name {
                    ascendancy: to_ascendancy_name(options.class, options.ascendancy)
                        .unwrap_or("unknown")
                        .into(),
                    nodes: options.nodes,
                    background_color,
                    node_color,
                    node_active_color,
                    connection_color,
                    connection_active_color,
                }
            }
        }
    };
}
use template_impl;

fn to_ascendancy_name(class: u8, ascendancy: u8) -> Option<&'static str> {
    let asc = match (class, ascendancy) {
        (0, 1) => "Ascendant",
        (1, 1) => "Juggernaut",
        (1, 2) => "Berserker",
        (1, 3) => "Chieftain",
        (2, 1) => "Raider",
        (2, 2) => "Deadeye",
        (2, 3) => "Pathfinder",
        (3, 1) => "Occultist",
        (3, 2) => "Elementalist",
        (3, 3) => "Necromancer",
        (4, 1) => "Slayer",
        (4, 2) => "Gladiator",
        (4, 3) => "Champion",
        (5, 1) => "Inquisitor",
        (5, 2) => "Hierophant",
        (5, 3) => "Guardian",
        (6, 1) => "Assassin",
        (6, 2) => "Trickster",
        (7, 3) => "Saboteur",
        _ => return None,
    };

    Some(asc)
}
