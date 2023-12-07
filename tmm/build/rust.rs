use std::io::Write;

use crate::tree::Tree;

const FILTERS: &str = r#"
mod filters {
    use super::*;
    pub fn connections(nodes: &[u16]) -> ::askama::Result<Vec<(u16, u16)>> {
        let mut result = Vec::new();
        for (a, b) in CONNECTIONS {
            if nodes.contains(&a) && nodes.contains(&b) {
                result.push((a, b));
            }
        }
        Ok(result)
    }
}
"#;

pub fn render(tree: &Tree, path: &str, output: &mut dyn Write) -> anyhow::Result<()> {
    macro_rules! w {
        ($($tts:tt)*) => {
            writeln!(output, $($tts)*)?
        }
    }

    w!(r#"// @generated"#);
    w!(r#"
       use super::*;
       type CowString = std::borrow::Cow<'static, str>;
    "#);

    let connections = tree
        .connections
        .iter()
        .map(|con| format!("({}, {})", con.a.id.min(con.b.id), con.a.id.max(con.b.id)))
        .collect::<Vec<_>>()
        .join(", ");
    w!(
        r#"const CONNECTIONS: [(u16, u16); {}] = [{}];"#,
        tree.connections.len(),
        connections
    );

    w!(r#"
       #[derive(::askama::Template, Debug)]
       #[template(path = "{path}", escape = "html")]
        pub struct Tree {{
            pub ascendancy: CowString,
            pub alternate_ascendancy: Option<CowString>,
            pub nodes: Vec<u16>,
            pub background_color: CowString,
            pub node_color: CowString,
            pub node_active_color: CowString,
            pub connection_color: CowString,
            pub connection_active_color: CowString,
        }}
    "#);

    w!("template_impl!(Tree);");

    w!("{}", FILTERS);

    w!(r#"
        impl Tree {{
            pub(crate) fn ascendancy_start_node(class: u8, ascendancy: u8) -> Option<u16> {{
                match (class, ascendancy) {{
    "#);
    for info in tree.ascendancies.values() {
        w!(
            "({}, {}) => Some({}),",
            info.class,
            info.ascendancy,
            info.start_node
        );
    }
    w!(r#"
                    _ => None,
                }}
            }}

            pub(crate) fn ascendancy_name(class: u8, ascendancy: u8) -> Option<&'static str> {{
                match (class, ascendancy) {{
    "#);
    for (name, info) in tree.ascendancies.iter() {
        w!(
            "({}, {}) => Some(\"{}\"),",
            info.class,
            info.ascendancy,
            name.as_ref()
        );
    }
    w!(r#"
                    _ => None,
                }}
            }}

            #[allow(clippy::match_single_binding)]
            pub(crate) fn alternate_ascendancy_name(class: u8, ascendancy: u8) -> Option<&'static str> {{
                match (class, ascendancy) {{
    "#);
    for (name, info) in tree.alternate_ascendancies.iter() {
        w!(
            "({}, {}) => Some(\"{}\"),",
            info.class,
            info.ascendancy,
            name.as_ref()
        );
    }
    w!(r#"
                    _ => None,
                }}
            }}


        }}
    "#);

    Ok(())
}
