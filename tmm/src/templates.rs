use std::borrow::Cow;

#[cfg(feature = "tree-3_17")]
pub(crate) mod tree3_17 {
    include!(concat!(env!("OUT_DIR"), "/tree3_17.rs"));
}

const DEFAULT_BACKGROUND_COLOR: Cow<'static, str> = Cow::Borrowed("rgba(0, 0, 0, 0)");
const DEFAULT_COLOR: Cow<'static, str> = Cow::Borrowed("#aaaaaa");
const DEFAULT_ACTIVE_COLOR: Cow<'static, str> = Cow::Borrowed("#dd00dd");

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
                    .unwrap_or(DEFAULT_ACTIVE_COLOR);

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
