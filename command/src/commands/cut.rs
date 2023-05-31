use super::{Command, IsChangeState};
use crate::AppContext;

use cursive::{views::EditView, Cursive};

#[derive(Default)]
pub struct CutCommand {
    backup: String,
}

impl Command<Cursive> for CutCommand {
    fn execute(&mut self, app: &mut Cursive) -> IsChangeState {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();

        app.with_user_data(|context: &mut AppContext| {
            // 現代の内容をバックアップ
            self.backup = editor.get_content().to_string();
            // クリップボードにコピー
            context.clipboard = self.backup.clone();
            // エディターの内容を空に
            editor.set_content("".to_string());
        });

        IsChangeState(true)
    }

    fn undo(&mut self, app: &mut Cursive) {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();
        // エディターの内容をバックアップと同じにする
        editor.set_content(&self.backup);
    }
}
