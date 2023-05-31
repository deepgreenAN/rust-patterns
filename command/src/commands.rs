mod copy;
mod cut;
mod paste;

pub use copy::CopyCommand;
pub use cut::CutCommand;
pub use paste::PasteCommand;

/// 状態を変更したかどうかを示すフラッグ
pub struct IsChangeState(pub bool);

/// 状態Tに対するコマンドが実装すべきトレイト
pub trait Command<T> {
    /// `app`に対してコマンドを実行
    fn execute(&mut self, app: &mut T) -> IsChangeState;
    /// `app`に対してコマンドのアンドゥ
    fn undo(&mut self, app: &mut T);
}
