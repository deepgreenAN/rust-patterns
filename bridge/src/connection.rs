use crate::database::Database;

// -------------------------------------------------------------------------------------------------
// Transaction

/// パターンには関係無い，データベースのトランザクション
pub struct Transaction<DB: Database> {
    _db: std::marker::PhantomData<DB>,
}

impl<DB: Database> Transaction<DB> {
    pub fn new() -> Self {
        Self {
            _db: std::marker::PhantomData,
        }
    }
    pub fn commit(self) {
        println!("トランザクションをコミットしました");
    }
    pub fn rollback(self) {
        println!("トランザクションをロールバックしました");
    }
}

// -------------------------------------------------------------------------------------------------
// Connection

/// "実装"側のインターフェースとなるトレイト
pub trait Connection: Clone {
    type Database: Database;

    fn new() -> Self;
    fn close(self);
    fn begin(&mut self) -> Transaction<Self::Database>;
    fn transaction<F, T>(&mut self, callback: F) -> Result<T, String>
    where
        F: FnOnce() -> Result<T, String>,
    {
        let transaction = self.begin();
        let res = callback();
        match res {
            Ok(res_value) => {
                transaction.commit();
                Ok(res_value)
            }
            Err(e) => {
                transaction.rollback();
                Err(e)
            }
        }
    }
}

/// MySqlの実装
#[derive(Clone)]
pub struct MySqlConnection;

impl Connection for MySqlConnection {
    type Database = crate::database::MySql;

    fn new() -> Self {
        MySqlConnection
    }
    fn close(self) {
        println!("MySqlデータベースコネクションをクローズしました");
    }
    fn begin(&mut self) -> Transaction<Self::Database> {
        println!("MySqlデータベースのトランザクションを開始しました");
        Transaction::new()
    }
}

/// Postgresの実装
#[derive(Clone)]
pub struct PgConnection;

impl Connection for PgConnection {
    type Database = crate::database::Postgres;

    fn new() -> Self {
        PgConnection
    }
    fn close(self) {
        println!("Postgresデータベースコネクションをクローズしました");
    }
    fn begin(&mut self) -> Transaction<Self::Database> {
        println!("Postgresデータベースのトランザクションを開始しました");
        Transaction::new()
    }
}

/// Sqliteの実装
#[derive(Clone)]
pub struct SqliteConnection;

impl Connection for SqliteConnection {
    type Database = crate::database::Sqlite;

    fn new() -> Self {
        SqliteConnection
    }
    fn close(self) {
        println!("Sqliteデータベースコネクションをクローズしました");
    }
    fn begin(&mut self) -> Transaction<Self::Database> {
        println!("Sqliteデータベースのトランザクションを開始しました");
        Transaction::new()
    }
}
