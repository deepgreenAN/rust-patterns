use std::collections::HashMap;
use uuid::Uuid;

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
#[derive(Default, Debug, PartialEq)]
pub struct TreeType {
    pub kind: TreeKind,
    pub color: Color,
}

/// 内因的状態のファクトリ
pub fn tree_type_factory(tree_type: TreeType, tree_type_map: &mut HashMap<Uuid, TreeType>) -> Uuid {
    let found_id = tree_type_map
        .iter()
        .find_map(|(k, v)| if *v == tree_type { Some(*k) } else { None });

    match found_id {
        Some(found_id) => found_id,
        None => {
            let new_id = Uuid::new_v4();
            tree_type_map.insert(new_id, tree_type);
            new_id
        }
    }
}
