use crate::data;
use std::f32::consts::PI;

const TWO_PI: f32 = 2.0 * PI;

#[derive(Copy, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub struct Node {
    pub id: u16,
    pub position: Coord,
    pub kind: NodeKind,
}

#[derive(Copy, Clone)]
pub enum NodeKind {
    Normal,
    Mastery,
    Keystone,
}

#[derive(Copy, Clone)]
pub struct NodeRef {
    pub id: u16,
    pub position: Coord,
}

pub struct Connection {
    pub a: NodeRef,
    pub b: NodeRef,
    pub path: Path,
}

pub enum Path {
    Arc { sweep: Sweep, radius: u32 },
    Line {},
}

pub enum Sweep {
    Clockwise,
    CounterClockwise,
}

pub struct ViewBox {
    pub x: i32,
    pub y: i32,
    pub dx: u32,
    pub dy: u32,
}

pub struct Tree {
    pub view_box: ViewBox,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}

pub fn build(tree: &data::Tree) -> Tree {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    let mut nodes = Vec::new();
    let mut connections = Vec::new();

    for group in tree.groups().filter(filter_group) {
        for node in group.nodes().filter(filter_node) {
            let (angle, x, y) = node.position();

            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);

            let mut kind = NodeKind::Normal;
            if node.is_keystone {
                kind = NodeKind::Keystone;
            } else if node.is_mastery {
                kind = NodeKind::Mastery;
            }

            nodes.push(Node {
                id: node.id(),
                position: Coord { x, y },
                kind,
            });

            for out_node in node
                .out()
                .filter(|out_node| filter_connection(&node, out_node))
            {
                let (out_angle, out_x, out_y) = out_node.position();

                let path = if node.group == out_node.group && node.orbit == out_node.orbit {
                    let radius = tree.data.constants.orbit_radii[node.orbit.unwrap() as usize];

                    let rot = (angle - out_angle + TWO_PI) % TWO_PI;
                    let sweep = if rot > PI {
                        Sweep::Clockwise
                    } else {
                        Sweep::CounterClockwise
                    };

                    Path::Arc { sweep, radius }
                } else {
                    Path::Line {}
                };

                connections.push(Connection {
                    a: NodeRef {
                        id: node.id(),
                        position: Coord { x, y },
                    },
                    b: NodeRef {
                        id: out_node.id(),
                        position: Coord { x: out_x, y: out_y },
                    },
                    path,
                })
            }
        }
    }

    let dx = (max_x - min_x) as u32;
    let dy = (max_y - min_y) as u32;

    Tree {
        view_box: ViewBox {
            x: min_x,
            y: min_y,
            dx,
            dy,
        },
        nodes,
        connections,
    }
}

fn filter_group(group: &data::Group) -> bool {
    !group.is_proxy
}

fn filter_node(node: &data::Node) -> bool {
    node.ascendancy_name.is_none() && node.class_start_index.is_none()
}
fn filter_connection(a: &data::Node, b: &data::Node) -> bool {
    filter_node(b) && !a.is_mastery && !b.is_mastery
}
