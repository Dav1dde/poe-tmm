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
       #[derive(::askama::Template)]
       #[template(path = "{path}", escape = "html")]
        pub struct Tree {{
            pub nodes: Vec<u16>,
        }}
    "#);

    w!("{}", FILTERS);

    Ok(())
}
