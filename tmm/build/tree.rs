use crate::data;
use std::collections::HashMap;
use std::f32::consts::PI;

const TWO_PI: f32 = 2.0 * PI;

#[derive(Default, Copy, Clone)]
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
    Ascendancy {
        kind: AscendancyNodeKind,
        ascendancy: Ascendancy,
    },
}

#[derive(Copy, Clone)]
pub enum AscendancyNodeKind {
    Start,
    Normal,
    Notable,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, strum::EnumString, strum::AsRefStr)]
pub enum Ascendancy {
    Ascendant,
    Juggernaut,
    Berserker,
    Chieftain,
    Raider,
    Deadeye,
    Pathfinder,
    Occultist,
    Elementalist,
    Necromancer,
    Slayer,
    Gladiator,
    Champion,
    Inquisitor,
    Hierophant,
    Guardian,
    Assassin,
    Trickster,
    Saboteur,
}

#[derive(Copy, Clone)]
pub struct NodeRef {
    pub id: u16,
    pub position: Coord,
}

pub struct Connection {
    pub a: Node,
    pub b: Node,
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
    pub ascendancies: HashMap<Ascendancy, AscendancyInfo>,
}

pub struct AscendancyInfo {
    pub class: u8,
    pub ascendancy: u8,
    pub start_node: u16,
}

pub fn build(tree: &data::Tree) -> Tree {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    let mut nodes = Vec::new();
    let mut connections = Vec::new();
    let mut ascendancies = HashMap::new();

    let mut tmp_ascendancies = HashMap::new();
    #[derive(Default)]
    struct TmpAsc {
        start_node: u16,
        start_position: Coord,
        nodes: Vec<Node>,
        connections: Vec<Connection>,
    }

    for group in tree.groups().filter(filter_group) {
        for node in group.nodes().filter(filter_node) {
            let (angle, x, y) = node.position();

            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);

            let tree_node = Node {
                id: node.id(),
                position: Coord { x, y },
                kind: node_kind(&node),
            };

            let (nodes, connections) =
                if let NodeKind::Ascendancy { ascendancy, .. } = tree_node.kind {
                    let asc = tmp_ascendancies
                        .entry(ascendancy)
                        .or_insert_with(TmpAsc::default);
                    if node.is_ascendancy_start {
                        asc.start_node = node.id();
                        asc.start_position = Coord { x, y };
                    }
                    (&mut asc.nodes, &mut asc.connections)
                } else {
                    (&mut nodes, &mut connections)
                };
            nodes.push(tree_node);

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

                let connection = Connection {
                    a: tree_node,
                    b: Node {
                        id: out_node.id(),
                        position: Coord { x: out_x, y: out_y },
                        kind: node_kind(&out_node),
                    },
                    path,
                };

                connections.push(connection);
            }
        }
    }

    const ASCENDANCY_POS_X: i32 = 7000;
    const ASCENDANCY_POS_Y: i32 = -7700;

    for (asc_name, asc) in tmp_ascendancies.into_iter() {
        let diff_x = ASCENDANCY_POS_X - asc.start_position.x;
        let diff_y = ASCENDANCY_POS_Y - asc.start_position.y;

        let update_node = |mut node: Node| {
            node.position = Coord {
                x: diff_x + node.position.x,
                y: diff_y + node.position.y,
            };
            node
        };

        for node in asc.nodes {
            nodes.push(update_node(node));
        }

        for mut connection in asc.connections {
            connection.a = update_node(connection.a);
            connection.b = update_node(connection.b);
            connections.push(connection);
        }

        let (class, ascendancy) = tree
            .data
            .classes
            .iter()
            .enumerate()
            .find_map(|(class_i, class)| {
                class
                    .ascendancies
                    .iter()
                    .enumerate()
                    .find(|(_, asc)| asc.name == asc_name.as_ref())
                    .map(|(asc_i, _)| (class_i, asc_i + 1))
            })
            .expect("ascendancy {asc_name} not found in classes array");

        ascendancies.insert(
            asc_name,
            AscendancyInfo {
                class: class as u8,
                ascendancy: ascendancy as u8,
                start_node: asc.start_node,
            },
        );
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
        ascendancies,
    }
}

fn node_kind(node: &data::Node) -> NodeKind {
    let mut kind = NodeKind::Normal;
    if node.is_keystone {
        kind = NodeKind::Keystone;
    } else if node.is_mastery {
        kind = NodeKind::Mastery;
    } else if node.ascendancy_name.is_some() {
        kind = NodeKind::Ascendancy {
            kind: match (node.is_ascendancy_start, node.is_notable) {
                (true, _) => AscendancyNodeKind::Start,
                (_, true) => AscendancyNodeKind::Notable,
                (_, false) => AscendancyNodeKind::Normal,
            },
            ascendancy: node
                .ascendancy_name
                .as_ref()
                .expect("ascendancy node should have an ascendancy name")
                .parse()
                .expect("invalid/unknown ascendancy name"),
        }
    }

    kind
}

fn filter_group(group: &data::Group) -> bool {
    !group.is_proxy
}

fn filter_node(node: &data::Node) -> bool {
    node.class_start_index.is_none()
}
fn filter_connection(a: &data::Node, b: &data::Node) -> bool {
    filter_node(b)
        && !a.is_mastery
        && !b.is_mastery
        // make sure there are no connections between ascendancy and non ascendancy nodes
        && (a.ascendancy_name.is_some() == b.ascendancy_name.is_some())
}
