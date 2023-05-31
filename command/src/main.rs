fn main() {
    use command::{app_execute, app_undo, commands, AppContext};

    use cursive::{
        traits::Nameable,
        views::{Dialog, EditView},
    };

    let mut app = cursive::crossterm();

    app.set_user_data(AppContext::default());

    app.add_layer(
        Dialog::around(EditView::default().with_name("Editor"))
            .title("Type and use buttons")
            .button("Copy", |s| app_execute(s, commands::CopyCommand::default()))
            .button("Cut", |s| app_execute(s, commands::CutCommand::default()))
            .button("Paste", |s| {
                app_execute(s, commands::PasteCommand::default())
            })
            .button("Undo", app_undo)
            .button("Quit", |s| s.quit()),
    );

    println!("App start");
    app.run();
}
