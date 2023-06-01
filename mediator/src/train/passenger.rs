use super::Train;

pub struct PassengerTrain {
    name: String,
}

impl PassengerTrain {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Train for PassengerTrain {
    fn name(&self) -> &str {
        &self.name
    }
    fn arrive(&mut self, mediator: &mut dyn crate::station::Station) {
        let arrival_info = mediator.notify_arrival(&self.name);
        // 駅に入れた場合とそうでない場合
        if arrival_info.is_entered {
            println!("旅客列車{}が到着しました", self.name);
        } else {
            println!(
                "現在{}が停まっているため，旅客列車{}は待たされています．",
                arrival_info.train_on_platform.unwrap(),
                self.name
            );
        }
    }
    fn depart(&mut self, mediator: &mut dyn crate::station::Station) {
        println!("旅客列車{}が出発しました", self.name);
        mediator.notify_departure(&self.name);
    }
}
