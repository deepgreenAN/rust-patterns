use crate::server_subject::{QueryServer, SqlServer};

use once_cell::sync::OnceCell;
use std::sync::Mutex;

pub struct ProxySqlServer {
    real_subject: Mutex<OnceCell<SqlServer>>,
}

impl ProxySqlServer {
    pub fn new() -> Self {
        // lazyに初期化
        Self {
            real_subject: Mutex::new(OnceCell::new()),
        }
    }
}

impl QueryServer for ProxySqlServer {
    fn query<T: Default>(&self, sql: &str) -> Vec<T> {
        let mut real_subject = self.real_subject.lock().unwrap();

        let res = real_subject
            .get_or_init(|| SqlServer::new())
            .query::<T>(sql);

        println!("クエリ: {sql} が実行されました");

        // 初期化
        *real_subject = OnceCell::new();

        res
    }
    fn migrate(&self, sql: &str) {
        let mut real_subject = self.real_subject.lock().unwrap();

        real_subject.get_or_init(|| SqlServer::new()).migrate(sql);

        println!("SQLサーバーがマイグレーションされました");

        // 初期化
        *real_subject = OnceCell::new();
    }
}
