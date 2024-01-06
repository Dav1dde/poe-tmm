use std::borrow::Cow;
use std::io::Write;

use crate::tree::{AscendancyNodeKind, NodeKind, Path, Sweep, Tree};

const STYLES: &str = r#"
svg {
    background-color: {{ background_color }};
}

.nodes {
    color: red;
}

.nodes circle {
}
.nodes circle.keystone {
}
.nodes circle.mastery {
    color: transparent;
    stroke-width: 40;
}

.ascendancy:not(.active) {
    display: none;
}

.connections {
    color: red;
}

.active {
    color: green !important;
}
"#;
const SCRIPT: &'static str = include_str!("svg.js");
const SCRIPT_MOUSE: &'static str = include_str!("svg-mouse.js");

const OFFSET: u32 = 100;

pub fn render(tree: &Tree, output: &mut dyn Write) -> anyhow::Result<()> {
    macro_rules! w {
        ($($tts:tt)*) => {
            writeln!(output, $($tts)*)?
        }
    }

    w!(
        r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
        tree.view_box.x - OFFSET as i32,
        tree.view_box.y - OFFSET as i32,
        tree.view_box.dx + OFFSET * 2,
        tree.view_box.dy + OFFSET * 2,
    );

    w!(r#"<style>{STYLES}</style>"#);

    w!(r#"<g class="connections" fill="none" stroke-width="20" stroke="currentColor">"#);
    for connection in &tree.connections {
        let x1 = connection.a.position.x;
        let y1 = connection.a.position.y;
        let x2 = connection.b.position.x;
        let y2 = connection.b.position.y;

        let a = connection.a.id.min(connection.b.id);
        let b = connection.a.id.max(connection.b.id);

        let class = match connection.a.kind {
            NodeKind::Ascendancy { ascendancy, .. } => {
                format!(r#"class="ascendancy {}""#, ascendancy.as_ref())
            }
            _ => String::new(),
        };

        match &connection.path {
            Path::Arc { sweep, radius: r } => {
                let sweep = match sweep {
                    Sweep::Clockwise => 1,
                    Sweep::CounterClockwise => 0,
                };
                w!(
                    r#"<path d="M {x1} {y1} A {r} {r} 0 0 {sweep} {x2} {y2}" id="c{a}-{b}" {class} />"#
                );
            }
            Path::Line {} => {
                w!(r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" id="c{a}-{b}" {class} />"#);
            }
        }
    }
    w!("</g>");

    w!(r#"<g class="nodes" stroke="currentColor" fill="currentColor">"#);
    for node in &tree.nodes {
        let attrs: Cow<'static, str> = match node.kind {
            NodeKind::Mastery => r#"r="35" class="mastery""#.into(),
            NodeKind::Keystone => r#"r="80" class="keystone""#.into(),
            NodeKind::Ascendancy { kind, ascendancy } => {
                let name = ascendancy.as_ref();
                use AscendancyNodeKind::*;
                match kind {
                    Start => "".into(),
                    Notable => format!(r#"r="65" class="ascendancy {name}""#).into(),
                    Normal => format!(r#"r="45" class="ascendancy {name}""#).into(),
                }
            }
            _ => r#"r="50""#.into(),
        };
        w!(
            r#"<circle cx="{}" cy="{}" id="n{}" {attrs} />"#,
            node.position.x,
            node.position.y,
            node.id
        );
    }
    w!("</g>");

    w!(r#"<script><![CDATA[(function() {{"#);
    w!(r#"window._ACTIVE_COLOR = "green";"#);
    w!(r#"window._ascendancy_name = function(classId, ascendancyId) {{"#);
    for (name, info) in tree.ascendancies.iter() {
        w!(
            r#"if (classId === {} && ascendancyId === {}) {{ return "{}" }}"#,
            info.class,
            info.ascendancy,
            name.as_ref()
        );
    }
    w!(r#"}}"#);
    w!(r#"window._alternate_ascendancy_name = function(classId, ascendancyId) {{"#);
    for (name, info) in tree.alternate_ascendancies.iter() {
        w!(
            r#"if (classId === {} && ascendancyId === {}) {{ return "{}" }}"#,
            info.class,
            info.ascendancy,
            name.as_ref()
        );
    }
    w!(r#"}}"#);
    w!(r#"}})()]]></script>"#);

    w!(r#"<script><![CDATA[(function() {{ {SCRIPT_MOUSE} }})()]]></script>"#);
    w!(r#"<script><![CDATA[addEventListener('load', function() {{ {SCRIPT} }})]]></script>"#);

    w!("</svg>");

    Ok(())
}
