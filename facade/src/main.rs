fn main() {
    use facade::{WalletFacade, WalletOptions};

    let wallet_options = WalletOptions::new()
        .account_id("abc")
        .security_code(1234)
        .build_options();

    let mut wallet = WalletFacade::new(wallet_options);

    wallet.add_money_to_wallet("abc", 1234, 10).unwrap();

    wallet.deduct_money_from_wallet("abc", 1234, 5).unwrap()
}
