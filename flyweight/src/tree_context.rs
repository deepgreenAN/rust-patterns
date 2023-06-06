use crate::tree_flyweight::TreeType;
use bevy_ecs::{change_detection::Mut, prelude::*, query::QueryEntityError};

/// 外因的状態．内因的状態をエンティティとして保持．
#[derive(Component, Debug)]
pub struct Tree {
    pub x: u32,
    pub y: u32,
    pub tree_type_entity: Entity,
}

impl Tree {
    /// クエリから内因的状態を取得
    pub fn tree_type_from_query<'w>(
        &self,
        query: &'w Query<&TreeType>,
    ) -> Result<&'w TreeType, QueryEntityError> {
        query.get(self.tree_type_entity)
    }

    /// クエリから内因的状態を可変参照で取得
    pub fn tree_type_mut_from_query<'w>(
        &self,
        query: &'w mut Query<&mut TreeType>,
    ) -> Result<Mut<'w, TreeType>, QueryEntityError> {
        query.get_mut(self.tree_type_entity)
    }
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
