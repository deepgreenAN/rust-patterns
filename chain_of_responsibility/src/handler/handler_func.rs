use super::ProcessError;

use dyn_clone::DynClone;

/// クローンするために必要なハンドラーに設定する関数が実装すべきトレイト
pub trait HandlerFunc<T>: Fn(T) -> Result<T, ProcessError<T>> + DynClone {}

// 任意のFn(T) -> Result<T, ProcessError<T>>にHandlerFuncを実装
impl<F, T> HandlerFunc<T> for F where F: Fn(T) -> Result<T, ProcessError<T>> + Clone {}
