use crate::tree_flyweight::TreeType;
use bevy_ecs::prelude::*;

/// 外因的状態．内因的状態をエンティティとして保持．
#[derive(Component, Debug)]
pub struct Tree {
    pub x: u32,
    pub y: u32,
    pub tree_type_entity: Entity,
}

/// 内因的状態のファクトリ．渡されたクエリ時点でしか調べないことに注意
pub fn tree_type_factory<'w>(
    new_tree_type: TreeType,
    query: &'w Query<(&TreeType, Entity)>,
    commands: &'w mut Commands,
) -> Entity {
    let found_entity = query
        .iter()
        .find(|(tree_type, _)| *tree_type == &new_tree_type)
        .map(|(_, entity)| entity);

    match found_entity {
        Some(found_entity) => found_entity,
        None => commands.spawn(new_tree_type).id(),
    }
}
