fn main() {
    use composite::{folder, text_file};

    let parent_folder = folder!(
        "parent",
        folder!(
            "a",
            text_file("d.txt"),
            folder!("e", text_file("i.txt"), text_file("j.txt")),
            text_file("f.txt")
        ),
        folder!(
            "b",
            folder!("g", text_file("k.txt"), text_file("l.txt")),
            text_file("h.txt")
        ),
        text_file("c.txt")
    );

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
