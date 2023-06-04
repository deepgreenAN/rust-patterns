fn main() {
    use bridge::database::Postgres;
    use bridge::pool::Pool;

    let pool = Pool::<Postgres>::connect("postgres://bridge");

    let transaction = pool.begin();

    transaction.rollback();
}
