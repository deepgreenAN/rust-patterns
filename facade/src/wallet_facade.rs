mod subsystems;

use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new), build_method(name = build_options))]
pub struct WalletOptions {
    #[builder(setter(into))]
    account_id: String,
    security_code: u32,
    #[builder(default, setter(strip_option))]
    balance: Option<u32>,
}

pub struct WalletFacade {
    account: subsystems::Account,
    wallet: subsystems::Wallet,
    code: subsystems::SecurityCode,
    notification: subsystems::Notification,
    ledger: subsystems::Ledger,
}

impl WalletFacade {
    pub fn new(options: WalletOptions) -> Self {
        let WalletOptions {
            account_id,
            security_code,
            balance,
        } = options;

        let this = Self {
            account: subsystems::Account::new(account_id),
            wallet: subsystems::Wallet::new(balance.unwrap_or(0)),
            code: subsystems::SecurityCode::new(security_code),
            notification: subsystems::Notification,
            ledger: subsystems::Ledger,
        };

        println!("アカウントを作成しました");
        this
    }

    pub fn add_money_to_wallet(
        &mut self,
        account_id: &str,
        security_code: u32,
        amount: u32,
    ) -> Result<(), String> {
        println!("お金を入れています");
        self.account.check(account_id)?;
        self.code.check(security_code)?;
        self.wallet.credit_balance(amount);
        self.notification.send_wallet_credit_notification();
        self.ledger.make_entry(account_id, "credit".into(), amount);
        Ok(())
    }

    pub fn deduct_money_from_wallet(
        &mut self,
        account_id: &str,
        security_code: u32,
        amount: u32,
    ) -> Result<(), String> {
        println!("お金を取り出しています");
        self.account.check(account_id)?;
        self.code.check(security_code)?;
        self.wallet.debit_balance(amount);
        self.notification.send_wallet_debit_notification();
        self.ledger.make_entry(account_id, "debit".into(), amount);
        Ok(())
    }
}
