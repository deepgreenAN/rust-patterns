use flyweight::tree_context::Tree;
use flyweight::tree_flyweight::TreeType;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

static TREE_TYPE_MAP: Lazy<Mutex<HashMap<Uuid, TreeType>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn print_forest(forest: &[Tree], tree_type_map: &HashMap<Uuid, TreeType>) {
    for tree in forest.iter() {
        println!("Tree: {tree:?}");
        let tree_type = tree.tree_type(tree_type_map).unwrap();
        println!("TreeType: {tree_type:?}");
    }
}

fn main() {
    use flyweight::tree_flyweight::{tree_type_factory, Color, TreeKind};

    let tree_type_1 = tree_type_factory(
        TreeType {
            kind: TreeKind::Conifer,
            color: Color {
                red: 10,
                green: 180,
                blue: 40,
            },
        },
        &mut TREE_TYPE_MAP.lock().unwrap(),
    );

    let tree_1 = Tree {
        x: 0,
        y: 100,
        tree_type_id: tree_type_1,
    };

    let tree_type_2 = tree_type_factory(
        TreeType {
            kind: TreeKind::Hardwood,
            color: Color {
                red: 255,
                green: 0,
                blue: 0,
            },
        },
        &mut TREE_TYPE_MAP.lock().unwrap(),
    );

    let tree_2 = Tree {
        x: 150,
        y: 150,
        tree_type_id: tree_type_2,
    };

    let tree_3 = Tree {
        x: 200,
        y: 50,
        tree_type_id: tree_type_1,
    };

    let tree_type_4 = tree_type_factory(
        TreeType {
            kind: TreeKind::Hardwood,
            color: Color {
                red: 255,
                green: 0,
                blue: 0,
            },
        },
        &mut TREE_TYPE_MAP.lock().unwrap(),
    );

    let tree_4 = Tree {
        x: 500,
        y: 500,
        tree_type_id: tree_type_4,
    };

    let forest = vec![tree_1, tree_2, tree_3, tree_4];

    print_forest(&forest, &TREE_TYPE_MAP.lock().unwrap());
}
