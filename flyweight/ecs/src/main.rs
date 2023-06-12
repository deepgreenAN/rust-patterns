use ecs::tree_context::{tree_type_factory, Tree};
use ecs::tree_flyweight::{Color, TreeKind, TreeType};

use bevy_ecs::prelude::*;

/// アプリケーションの開始時にコンポーネントをスポーン
pub fn start_up(mut commands: Commands) {
    let tree_type_entity_1 = commands
        .spawn(TreeType {
            kind: TreeKind::Conifer,
            color: Color {
                red: 0,
                green: 255,
                blue: 0,
            },
        })
        .id();
    commands.spawn(Tree {
        x: 0,
        y: 100,
        tree_type_entity: tree_type_entity_1,
    });

    let tree_type_entity_2 = commands
        .spawn(TreeType {
            kind: TreeKind::Hardwood,
            color: Color {
                red: 255,
                green: 0,
                blue: 0,
            },
        })
        .id();
    commands.spawn(Tree {
        x: 150,
        y: 150,
        tree_type_entity: tree_type_entity_2,
    });

    commands.spawn(Tree {
        x: 200,
        y: 50,
        tree_type_entity: tree_type_entity_1,
    });
}

/// 全てのTreeとそれに結びついたTreeTypeコンポーネントを表示
pub fn all_tree_print(tree_query: Query<&Tree>, tree_type_query: Query<&TreeType>) {
    for tree in tree_query.iter() {
        println!("Tree: {tree:?}");
        let tree_type = tree.tree_type_from_query(&tree_type_query).unwrap();
        println!("TreeType: {tree_type:?}");
    }
}

/// Treeを追加でスポーン
pub fn spawn_trees(mut commands: Commands, tree_type_query: Query<(&TreeType, Entity)>) {
    {
        let tree_type_entity = tree_type_factory(
            TreeType {
                kind: TreeKind::Conifer,
                color: Color {
                    red: 10,
                    green: 180,
                    blue: 40,
                },
            },
            &tree_type_query,
            &mut commands,
        );

        commands.spawn(Tree {
            x: 100,
            y: 100,
            tree_type_entity,
        });
    }
    {
        let tree_type_entity = tree_type_factory(
            TreeType {
                kind: TreeKind::Hardwood,
                color: Color {
                    red: 255,
                    green: 0,
                    blue: 0,
                },
            },
            &tree_type_query,
            &mut commands,
        );

        commands.spawn(Tree {
            x: 300,
            y: 300,
            tree_type_entity,
        });
    }
}

fn main() {
    let mut world = World::new();

    Schedule::default().add_system(start_up).run(&mut world);

    Schedule::default().add_system(spawn_trees).run(&mut world);

    Schedule::default()
        .add_system(all_tree_print)
        .run(&mut world);
}
