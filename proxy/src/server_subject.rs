pub trait QueryServer {
    fn query<T: Default>(&self, sql: &str) -> Vec<T>;
    fn migrate(&self, sql: &str);
}

pub struct SqlServer;

impl SqlServer {
    pub fn new() -> Self {
        println!("SQLサーバーインスタンスを起動しています...");
        SqlServer
    }
}

impl QueryServer for SqlServer {
    fn query<T: Default>(&self, _sql: &str) -> Vec<T> {
        (0..10).map(|_| T::default()).collect()
    }
    fn migrate(&self, _sql: &str) {}
}
