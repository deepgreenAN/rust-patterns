use bevy_ecs::prelude::*;

#[derive(Default, Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Default, Debug, PartialEq)]
pub enum TreeKind {
    #[default]
    Conifer,
    Hardwood,
}

/// 内因的状態
#[derive(Default, Debug, PartialEq, Component)]
pub struct TreeType {
    pub kind: TreeKind,
    pub color: Color,
}
