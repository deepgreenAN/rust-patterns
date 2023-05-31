pub mod commands;

use commands::Command;
use cursive::Cursive;

#[derive(Default)]
pub struct AppContext {
    /// クリップボードの文字列
    clipboard: String,
    /// コマンド履歴
    history: Vec<Box<dyn Command<Cursive>>>,
}

// コマンドを実行するための関数
pub fn app_execute<Cmd>(app: &mut Cursive, mut cmd: Cmd)
where
    Cmd: Command<Cursive> + 'static,
{
    // コマンドの状態を変更する場合のみ履歴に追加
    if cmd.execute(app).0 {
        app.with_user_data(|context: &mut AppContext| {
            context.history.push(Box::new(cmd));
        });
    }
}

// コマンドをアンドゥするための関数
pub fn app_undo(app: &mut Cursive) {
    let mut context = app.take_user_data::<AppContext>().unwrap();
    if let Some(mut cmd) = context.history.pop() {
        cmd.undo(app)
    }
    app.set_user_data(context);
}
