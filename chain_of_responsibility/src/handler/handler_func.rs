use crate::{Patient, ProcessError};

use dyn_clone::DynClone;

/// クローンするために必要なハンドラーに設定する関数が実装すべきトレイト
pub trait HandlerFunc: Fn(Patient) -> Result<Patient, ProcessError> + DynClone {}

impl<T: Fn(Patient) -> Result<Patient, ProcessError> + Clone> HandlerFunc for T {}
