use super::{ArrivalInfo, DepartureInfo, Station};
use crate::train::Train;
use crate::StationError;

use std::collections::{HashMap, VecDeque};

pub struct SingleTrackStation {
    /// コンボーネントのコレクション
    trains: HashMap<String, Box<dyn Train>>,
    /// 駅に入りたい列車のキュー
    train_queue: VecDeque<String>,
    /// プラットフォームにある列車名
    train_on_platform: Option<String>,
}

impl Station for SingleTrackStation {
    fn notify_arrival(&mut self, train_name: &str) -> ArrivalInfo {
        // 既にプラットフォームに列車が存在している場合
        if self.train_on_platform.is_some() {
            // 待っているキューに加える
            self.train_queue.push_back(train_name.to_string());

            ArrivalInfo {
                is_entered: false,
                train_on_platform: self.train_on_platform.clone(),
            }
        } else {
            // プラットフォームに移動
            self.train_on_platform = Some(train_name.to_string());

            ArrivalInfo {
                is_entered: true,
                train_on_platform: self.train_on_platform.clone(),
            }
        }
    }
    fn notify_departure(&mut self, train_name: &str) -> DepartureInfo {
        // 与えた名前の列車がプラットフォームに存在している場合
        if Some(train_name.to_string()) == self.train_on_platform {
            // プラットフォームから無くなる
            self.train_on_platform = None;

            // 次に駅に列車が入る．
            if let Some(next_train_name) = self.train_queue.pop_front() {
                // 可変参照のまま行えないため，一度取り出してからTrainのメソッドを実行
                let mut next_train = self.trains.remove(&next_train_name).unwrap();
                next_train.arrive(self);

                self.trains.insert(next_train_name.clone(), next_train);

                self.train_on_platform = Some(next_train_name);
            }
        }

        DepartureInfo
    }
}

/// SingleTrackStationの外部インターフェース．Trainを利用しているだけ．
impl SingleTrackStation {
    pub fn new() -> Self {
        Self {
            trains: HashMap::new(),
            train_queue: VecDeque::new(),
            train_on_platform: None,
        }
    }
    pub fn accept_train(
        &mut self,
        mut new_train: impl Train + 'static,
    ) -> Result<(), StationError> {
        if self.trains.contains_key(new_train.name()) {
            return Err(StationError("既に受け入れた列車です".to_string()));
        }

        new_train.arrive(self);
        self.trains
            .insert(new_train.name().to_string(), Box::new(new_train));

        Ok(())
    }
    pub fn depart_train(&mut self, train_name: &str) -> Result<(), StationError> {
        let train_opt = self.trains.remove(train_name);
        if let Some(mut train) = train_opt {
            train.depart(self);
            Ok(())
        } else {
            Err(StationError(
                "まだ受け入れてないか・既に出発した列車です".to_string(),
            ))
        }
    }
}
