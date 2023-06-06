fn main() {
    use proxy::server_proxy::ProxySqlServer;
    use proxy::server_subject::{QueryServer, SqlServer};

    println!("real subjectのサーバー");

    {
        let server = SqlServer::new();

        println!("SqlServerのコンストラクタをよびました");
        server.migrate("Some migration sql");

        let _ = server.query::<u32>("Some query sql");
    }

    println!("以下はプロキシサーバー");

    {
        let server = ProxySqlServer::new();

        println!("ProxySqlServerのコンストラクタをよびました");

        server.migrate("Some migration sql");

        let _ = server.query::<u32>("Some query sql");
    }
}
