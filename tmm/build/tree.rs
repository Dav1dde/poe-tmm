use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;

use crate::data;

const TWO_PI: f32 = 2.0 * PI;

#[derive(Debug, Default, Copy, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub id: u16,
    pub position: Coord,
    pub kind: NodeKind,
}

#[derive(Debug, Copy, Clone)]
pub enum NodeKind {
    Normal,
    Mastery,
    Keystone,
    Ascendancy {
        kind: AscendancyNodeKind,
        ascendancy: Ascendancy,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum AscendancyNodeKind {
    Start,
    Normal,
    Notable,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, strum::EnumString, strum::AsRefStr)]
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
    // 3.23 Ascendancies
    Warden,
    Warlock,
    Primalist,
}

impl Ascendancy {
    pub fn is_alternate(self) -> bool {
        matches!(self, Self::Warden | Self::Warlock | Self::Primalist)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NodeRef {
    pub id: u16,
    pub position: Coord,
}

#[derive(Debug)]
pub struct Connection {
    pub a: Node,
    pub b: Node,
    pub path: Path,
}

#[derive(Debug)]
pub enum Path {
    Arc { sweep: Sweep, radius: u32 },
    Line {},
}

#[derive(Debug)]
pub enum Sweep {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug)]
pub struct ViewBox {
    pub x: i32,
    pub y: i32,
    pub dx: u32,
    pub dy: u32,
}

#[derive(Debug)]
pub struct Tree {
    pub view_box: ViewBox,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub ascendancies: HashMap<Ascendancy, AscendancyInfo>,
    pub alternate_ascendancies: HashSet<(Ascendancy, AscendancyInfo)>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

    let mut update_min_max = |x, y| {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    };

    let mut nodes = Vec::new();
    let mut connections = Vec::new();

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
                    // Only update on normal tree nodes, ascendancies will be moved
                    update_min_max(x, y);

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

    let mut ascendancies = HashMap::new();
    let mut alternate_ascendancies = HashSet::new();

    for (asc_name, asc) in tmp_ascendancies.into_iter() {
        let pos_x_offset = if asc_name.is_alternate() {
            -ASCENDANCY_POS_X
        } else {
            ASCENDANCY_POS_X
        };
        let diff_x = pos_x_offset - asc.start_position.x;
        let diff_y = ASCENDANCY_POS_Y - asc.start_position.y;

        let update_node = |node: &mut Node| {
            node.position = Coord {
                x: diff_x + node.position.x,
                y: diff_y + node.position.y,
            };
        };

        for mut node in asc.nodes {
            update_node(&mut node);
            update_min_max(node.position.x, node.position.y);
            nodes.push(node);
        }

        for mut connection in asc.connections {
            update_node(&mut connection.a);
            update_node(&mut connection.b);
            connections.push(connection);
        }

        if asc_name.is_alternate() {
            let ascendancy = tree.data.alternate_ascendancies.iter().enumerate().find_map(|(i, asc)| (asc.id == asc_name.as_ref()).then_some(i as u8))
                .unwrap_or_else(|| panic!("expected to find alternate ascendancy {asc_name:?} in the alternate ascendancy array"));

            for (class, _) in tree.data.classes.iter().enumerate() {
                alternate_ascendancies.insert((
                    asc_name,
                    AscendancyInfo {
                        class: class as u8,
                        ascendancy,
                        start_node: asc.start_node,
                    },
                ));
            }
            continue;
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
            .unwrap_or((0, 0));

        ascendancies.insert(
            asc_name,
            AscendancyInfo {
                class: class as u8,
                ascendancy: ascendancy as u8,
                start_node: asc.start_node,
            },
        );
    }

    // Small border to make positioning the image easier
    min_x -= 75;
    min_y -= 75;
    max_x += 75;
    max_y += 75;

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
        alternate_ascendancies,
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
