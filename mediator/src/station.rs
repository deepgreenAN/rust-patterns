mod single_track;

pub use single_track::SingleTrackStation;

// メディエイターから返される情報(機能に応じて変更される)
pub struct ArrivalInfo {
    pub is_entered: bool,
    pub train_on_platform: Option<String>,
}

pub struct DepartureInfo;

/// メディエイターのトレイト．外部のAPIではなくコンボーネントを結び付けるロジックであり，機能に応じて変更されることがある．
/// このインターフェースを各種コンポーネント(Trainを実装した型)は知っている必要がある．(逆にこれだを知っていればよい)
pub trait Station {
    /// メディエイターとメディエイターに結びついているコンポーネント(列車)に対して新たな列車の到着を通知．情報を返答．
    fn notify_arrival(&mut self, train_name: &str) -> ArrivalInfo;
    /// メディエイターとメディエイターに結びついているコンポーネント(列車)に対して指定した列車の出発を通知．情報を返答．
    fn notify_departure(&mut self, train_name: &str) -> DepartureInfo;
}
