use std::rc::Rc;

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
#[derive(Debug)]
pub struct TreeTypeFactory {
    tree_types: Vec<Rc<TreeType>>,
}

impl TreeTypeFactory {
    pub fn new() -> Self {
        Self {
            tree_types: Vec::new(),
        }
    }

    pub fn register_tree_type(&mut self, new_tree_type: TreeType) -> Rc<TreeType> {
        let found_tree_type = self
            .tree_types
            .iter()
            .find(|tree_type| ***tree_type == new_tree_type);

        match found_tree_type {
            Some(found_tree_type) => Rc::clone(found_tree_type),
            None => {
                let registered_tree_type = Rc::new(new_tree_type);
                self.tree_types.push(Rc::clone(&registered_tree_type));
                registered_tree_type
            }
        }
    }
}
