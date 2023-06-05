pub struct Wallet {
    /// 残高
    balance: u32,
}

impl Wallet {
    pub fn new(balance: u32) -> Self {
        Self { balance }
    }
    pub fn credit_balance(&mut self, amount: u32) {
        self.balance += amount;
    }
    pub fn debit_balance(&mut self, amount: u32) {
        self.balance.checked_sub(amount).expect("残高が足りません");
    }
}

pub struct Account {
    name: String,
}

impl Account {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn check(&self, name: &str) -> Result<(), String> {
        if &self.name != name {
            return Err("アカウント名が違います".to_string());
        }

        println!("アカウントが認証されました");
        Ok(())
    }
}

pub struct Ledger;

impl Ledger {
    pub fn make_entry(&mut self, account_id: &str, txn_type: &str, amount: u32) {
        println!(
            "台帳エントリー: アカウントID: {} トランザクションタイプ: {} 取引量: {}",
            account_id, txn_type, amount
        );
    }
}

pub struct Notification;

impl Notification {
    pub fn send_wallet_credit_notification(&self) {
        println!("クレジットの通知を送信しました");
    }

    pub fn send_wallet_debit_notification(&self) {
        println!("デビットの通知を送信しました");
    }
}

pub struct SecurityCode {
    code: u32,
}

impl SecurityCode {
    pub fn new(code: u32) -> Self {
        Self { code }
    }

    pub fn check(&self, code: u32) -> Result<(), String> {
        if self.code != code {
            return Err("セキュリティコードが間違っています".to_string());
        }

        println!("セキュリティコードが認証されました");
        Ok(())
    }
}
