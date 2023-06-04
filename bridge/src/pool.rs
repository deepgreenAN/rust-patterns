use crate::connection::Connection;
use crate::database::Database;

// -------------------------------------------------------------------------------------------------

/// "抽象"側のコネクションプール
/// Connectionトレイと実装した型をクライアントから隠し，より分かりやすいデータベース型のみ知っていればよい．
pub struct Pool<DB: Database> {
    connection: DB::Connection,
}

impl<DB: Database> Pool<DB> {
    pub fn connect(_url: &str) -> Self {
        Self {
            connection: DB::Connection::new(),
        }
    }
    pub fn acquire(&self) -> DB::Connection {
        self.connection.clone()
    }
    pub fn begin(&self) -> crate::connection::Transaction<DB> {
        self.acquire().begin()
    }
}
