use poe_api::api::*;
use std::cell::Cell;
use std::f32::consts::PI;
use std::ops::Deref;
use std::path::Path;

const TWO_PI: f32 = 2.0 * PI;

const ANGLES_16: [u32; 16] = [
    0, 30, 45, 60, 90, 120, 135, 150, 180, 210, 225, 240, 270, 300, 315, 330,
];
const ANGLES_40: [u32; 40] = [
    0, 10, 20, 30, 40, 45, 50, 60, 70, 80, 90, 100, 110, 120, 130, 135, 140, 150, 160, 170, 180,
    190, 200, 210, 220, 225, 230, 240, 250, 260, 270, 280, 290, 300, 310, 315, 320, 330, 340, 350,
];

pub struct Tree {
    pub data: SkillTreeData,
}

impl Tree {
    pub fn read(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let f = std::fs::read_to_string(path)?;
        let data: poe_api::api::SkillTreeData = serde_json::from_str(&f)?;
        Ok(Self { data })
    }

    pub fn groups(&self) -> impl Iterator<Item = Group<'_>> {
        self.data.groups.values().map(|group| Group {
            inner: group,
            parent: self,
        })
    }

    fn get_position(&self, node: &SkillTreeNode, group: &SkillTreeGroup) -> (f32, i32, i32) {
        let radius = self.data.constants.orbit_radii[node.orbit.unwrap() as usize] as f32;
        let skills_on_orbit = self.data.constants.skills_per_orbit[node.orbit.unwrap() as usize];
        let orbit_index = node.orbit_index.unwrap_or(0);

        let angle = match skills_on_orbit {
            16 => (ANGLES_16[orbit_index as usize] as f32).to_radians(),
            40 => (ANGLES_40[orbit_index as usize] as f32).to_radians(),
            soo => TWO_PI / soo as f32 * orbit_index as f32,
        };

        let x = group.x + radius * angle.sin();
        let y = group.y - radius * angle.cos();

        (angle % TWO_PI, x as i32, y as i32)
    }
}

pub struct Group<'a> {
    inner: &'a SkillTreeGroup,
    parent: &'a Tree,
}

impl<'a> Group<'a> {
    pub fn nodes(&self) -> impl Iterator<Item = Node<'_>> {
        self.inner
            .nodes
            .iter()
            .map(|node_id| {
                (
                    node_id,
                    self.parent
                        .data
                        .nodes
                        .get(node_id)
                        .expect("group contains unknown node"),
                )
            })
            .map(|(id, node)| Node {
                id: id.parse().unwrap(),
                inner: node,
                groupx: Cell::new(Some(self.inner)),
                parent: self.parent,
            })
    }
}

impl<'a> Deref for Group<'a> {
    type Target = SkillTreeGroup;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

pub struct Node<'a> {
    id: u16,
    inner: &'a SkillTreeNode,
    groupx: Cell<Option<&'a SkillTreeGroup>>,
    parent: &'a Tree,
}

impl<'a> Node<'a> {
    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn position(&self) -> (f32, i32, i32) {
        self.parent.get_position(self.inner, self.group())
    }

    pub fn out(&self) -> impl Iterator<Item = Node<'_>> {
        self.inner
            .out
            .iter()
            .map(|id| (id, self.parent.data.nodes.get(id).unwrap()))
            .map(|(id, node)| Node {
                id: id.parse().unwrap(),
                inner: node,
                parent: self.parent,
                groupx: Cell::new(None),
            })
    }

    fn group(&self) -> &SkillTreeGroup {
        if let Some(group) = self.groupx.get() {
            return group;
        }

        let group_id = self.inner.group.unwrap().to_string();
        let group = self.parent.data.groups.get(&group_id).unwrap();
        self.groupx.set(Some(group));
        group
    }
}

impl<'a> Deref for Node<'a> {
    type Target = SkillTreeNode;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
