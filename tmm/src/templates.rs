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
#[cfg(feature = "tree-3_20")]
pub(crate) mod tree3_20 {
    include!(concat!(env!("OUT_DIR"), "/tree3_20.rs"));
}
#[cfg(feature = "tree-3_21")]
pub(crate) mod tree3_21 {
    include!(concat!(env!("OUT_DIR"), "/tree3_21.rs"));
}
#[cfg(feature = "tree-3_22")]
pub(crate) mod tree3_22 {
    include!(concat!(env!("OUT_DIR"), "/tree3_22.rs"));
}
#[cfg(feature = "tree-3_23")]
pub(crate) mod tree3_23 {
    include!(concat!(env!("OUT_DIR"), "/tree3_23.rs"));
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

                let mut nodes = options.nodes;
                if let Some(start_node) =
                    Self::ascendancy_start_node(options.class, options.ascendancy)
                {
                    nodes.push(start_node);
                }

                let alternate_ascendancy = options
                    .alternate_ascendancy
                    .and_then(|aa| Self::alternate_ascendancy_name(options.class, aa))
                    .map(std::borrow::Cow::Borrowed);

                $name {
                    ascendancy: Self::ascendancy_name(options.class, options.ascendancy)
                        .unwrap_or("unknown")
                        .into(),
                    alternate_ascendancy,
                    nodes,
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
