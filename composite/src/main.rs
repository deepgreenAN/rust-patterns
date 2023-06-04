fn main() {
    use composite::{File, FileComponent};

    let a_folder = {
        let e_folder = {
            let mut e_folder = FileComponent::new_folder("e".to_string());
            e_folder
                .add(FileComponent::File(File::TextFile("i.txt".to_string())))
                .unwrap();
            e_folder
                .add(FileComponent::File(File::TextFile("j.txt".to_string())))
                .unwrap();
            e_folder
        };

        let mut a_folder = FileComponent::new_folder("a".to_string());
        a_folder
            .add(FileComponent::File(File::TextFile("d.txt".to_string())))
            .unwrap();
        a_folder.add(e_folder).unwrap();
        a_folder
            .add(FileComponent::File(File::TextFile("f.txt".to_string())))
            .unwrap();

        a_folder
    };

    let b_folder = {
        let g_folder = {
            let mut g_folder = FileComponent::new_folder("g".to_string());
            g_folder
                .add(FileComponent::File(File::TextFile("k.txt".to_string())))
                .unwrap();
            g_folder
                .add(FileComponent::File(File::TextFile("l.txt".to_string())))
                .unwrap();
            g_folder
        };

        let mut b_folder = FileComponent::new_folder("b".to_string());
        b_folder.add(g_folder).unwrap();
        b_folder
            .add(FileComponent::File(File::TextFile("h.txt".to_string())))
            .unwrap();

        b_folder
    };

    let parent_folder = {
        let mut parent_folder = FileComponent::new_folder("parent".to_string());
        parent_folder.add(a_folder).unwrap();
        parent_folder.add(b_folder).unwrap();
        parent_folder
            .add(FileComponent::File(File::TextFile("c.txt".to_string())))
            .unwrap();

        parent_folder
    };

    parent_folder.print_recursive();

    println!("以下はiter_file");
    for file in parent_folder.iter_file() {
        println!("イテレーターから取得したファイル: {}", file.name());
    }

    println!("以下はiter_file_component");
    for file_component in parent_folder.iter_file_component() {
        println!(
            "イテレーターから取得したコンポーネント: {}",
            file_component.name()
        );
    }

    println!("以下はinto_iter");
    for file in parent_folder.into_iter() {
        println!("イテレーターから取得したファイル: {}", file.name());
    }
}
