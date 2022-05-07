use std::io::Write;

use crate::tree::{Path, Sweep, Tree};

const STYLES_TEMPLATE: &str = r#"
{% for node in nodes -%}
#n{{ node }}{% if !loop.last %}, {% endif -%}
{%- endfor %} {
    color: red;
}

{% for (a, b) in nodes|connections -%}
#c{{ a }}-{{ b }}{% if !loop.last %}, {% endif -%}
{%- endfor %} {
    color: red;
}
"#;

const OFFSET: u32 = 100;

pub fn render(tree: &Tree, output: &mut dyn Write) -> anyhow::Result<()> {
    macro_rules! w {
        ($($tts:tt)*) => {
            writeln!(output, $($tts)*)?
        }
    }

    w!(
        r#"<svg style="background-color: #aaa" nodes="44202 29353" viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
        tree.view_box.x - OFFSET as i32,
        tree.view_box.y - OFFSET as i32,
        tree.view_box.dx + OFFSET * 2,
        tree.view_box.dy + OFFSET * 2,
    );

    w!(r#"<style>"#);
    w!(
        r#"
        #nodes circle {{
            r: 50;
            fill: currentColor;
        }}
        #connections {{
            fill: none;
            stroke: currentColor;
            stroke-width: 20;
        }}

        {}
    "#,
        STYLES_TEMPLATE
    );
    w!(r#"</style>"#);

    w!(r#"<g id="connections">"#);
    for connection in &tree.connections {
        let x1 = connection.a.position.x;
        let y1 = connection.a.position.y;
        let x2 = connection.b.position.x;
        let y2 = connection.b.position.y;

        let a = connection.a.id.min(connection.b.id);
        let b = connection.a.id.max(connection.b.id);

        match &connection.path {
            Path::Arc { sweep, radius: r } => {
                let sweep = match sweep {
                    Sweep::Clockwise => 1,
                    Sweep::CounterClockwise => 0,
                };
                w!(r#"<path d="M {x1} {y1} A {r} {r} 0 0 {sweep} {x2} {y2}" id="c{a}-{b}" />"#);
            }
            Path::Line {} => {
                w!(r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" id="c{a}-{b}" />"#);
            }
        }
    }
    w!("</g>");

    w!(r#"<g id="nodes">"#);
    for node in &tree.nodes {
        w!(
            r#"<circle cx="{}" cy="{}" id="n{}" />"#,
            node.position.x,
            node.position.y,
            node.id
        );
    }
    w!("</g>");

    w!("</svg>");

    Ok(())
}
