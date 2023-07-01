use flyweight::tree_context::Tree;
use flyweight::tree_flyweight::TreeType;

fn print_forest(forest: &[Tree]) {
    for tree in forest.iter() {
        println!("Tree: {tree:?}");
        let tree_type = &tree.tree_type;
        println!("TreeType: {tree_type:?}");
    }
}

fn main() {
    use flyweight::tree_flyweight::{Color, TreeKind, TreeTypeFactory};

    use std::rc::Rc;

    let mut tree_type_factory = TreeTypeFactory::new();

    let tree_type_1 = tree_type_factory.register_tree_type(TreeType {
        kind: TreeKind::Conifer,
        color: Color {
            red: 10,
            green: 180,
            blue: 40,
        },
    });

    let tree_1 = Tree {
        x: 0,
        y: 100,
        tree_type: Rc::clone(&tree_type_1),
    };

    let tree_type_2 = tree_type_factory.register_tree_type(TreeType {
        kind: TreeKind::Hardwood,
        color: Color {
            red: 255,
            green: 0,
            blue: 0,
        },
    });

    let tree_2 = Tree {
        x: 150,
        y: 150,
        tree_type: tree_type_2,
    };

    let tree_3 = Tree {
        x: 200,
        y: 50,
        tree_type: tree_type_1,
    };

    let tree_type_4 = tree_type_factory.register_tree_type(TreeType {
        kind: TreeKind::Hardwood,
        color: Color {
            red: 255,
            green: 0,
            blue: 0,
        },
    });

    let tree_4 = Tree {
        x: 500,
        y: 500,
        tree_type: tree_type_4,
    };

    let forest = vec![tree_1, tree_2, tree_3, tree_4];

    print_forest(&forest);

    println!("{tree_type_factory:?}");
}
