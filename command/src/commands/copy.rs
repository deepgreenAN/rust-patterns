use super::{Command, IsChangeState};
use crate::AppContext;

use cursive::{views::EditView, Cursive};

#[derive(Default)]
pub struct CopyCommand;

impl Command<Cursive> for CopyCommand {
    /// コピーを実行
    fn execute(&mut self, app: &mut Cursive) -> IsChangeState {
        let editor = app.find_name::<EditView>("Editor").unwrap();
        let mut context = app.take_user_data::<AppContext>().unwrap();

        // エディターの文字列をクリップボードにコピー．
        context.clipboard = editor.get_content().to_string();

        // コンテキストを更新
        app.set_user_data(context);

        IsChangeState(false)
    }
    /// `execute`で自身を変更しないためなにもしない
    fn undo(&mut self, _app: &mut Cursive) {}
}
