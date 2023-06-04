use crate::connection::Connection;

/// 型パラメーターを汎用的なものにするテクニック
/// コネクションの関連型とプールの型パラメーターとして利用する
/// 関連型としてコネクショントレイトを実装した型を要求し，コネクショントレイトの関連型を自身とする．
pub trait Database {
    type Connection: Connection<Database = Self>;
}

pub struct MySql;

impl Database for MySql {
    type Connection = crate::connection::MySqlConnection;
}

pub struct Postgres;

impl Database for Postgres {
    type Connection = crate::connection::PgConnection;
}

pub struct Sqlite;

impl Database for Sqlite {
    type Connection = crate::connection::SqliteConnection;
}
