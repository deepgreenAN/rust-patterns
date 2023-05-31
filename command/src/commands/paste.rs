use super::{Command, IsChangeState};
use crate::AppContext;

use cursive::{views::EditView, Cursive};

#[derive(Default)]
pub struct PasteCommand {
    backup: String,
}

impl Command<Cursive> for PasteCommand {
    fn execute(&mut self, app: &mut Cursive) -> IsChangeState {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();

        app.with_user_data(|context: &mut AppContext| {
            // エディターの内容をバックアップ
            self.backup = editor.get_content().to_string();
            // クリップボードの内容をエディターにペースト．
            editor.set_content(context.clipboard.clone());
        });

        IsChangeState(true)
    }

    fn undo(&mut self, app: &mut Cursive) {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();
        // エディターをバックアップと同じ内容にする．
        editor.set_content(&self.backup);
    }
}
