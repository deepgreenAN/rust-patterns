use crate::tree_flyweight::TreeType;

use std::rc::Rc;

/// 外因的状態．内因的状態をuuidとして保持．
#[derive(Debug)]
pub struct Tree {
    pub x: u32,
    pub y: u32,
    pub tree_type: Rc<TreeType>,
}
