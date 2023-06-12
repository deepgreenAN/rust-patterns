use crate::tree_flyweight::TreeType;

use std::collections::HashMap;
use uuid::Uuid;

/// 外因的状態．内因的状態をuuidとして保持．
#[derive(Debug)]
pub struct Tree {
    pub x: u32,
    pub y: u32,
    pub tree_type_id: Uuid,
}

impl Tree {
    // tree_typeを参照で参照で取得
    pub fn tree_type<'a>(
        &self,
        tree_type_map: &'a HashMap<Uuid, TreeType>,
    ) -> Option<&'a TreeType> {
        tree_type_map.get(&self.tree_type_id)
    }
    /// tree_typeを可変参照で取得
    pub fn tree_type_mut<'a>(
        &self,
        tree_type_map: &'a mut HashMap<Uuid, TreeType>,
    ) -> Option<&'a mut TreeType> {
        tree_type_map.get_mut(&self.tree_type_id)
    }
}
