use super::Train;

pub struct FreightTrain {
    name: String,
}

impl FreightTrain {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Train for FreightTrain {
    fn name(&self) -> &str {
        &self.name
    }
    fn arrive(&mut self, mediator: &mut dyn crate::station::Station) {
        let arrival_info = mediator.notify_arrival(&self.name);
        // 駅に入れた場合とそうでない場合
        if arrival_info.is_entered {
            println!("貨物列車{}が到着しました", self.name);
        } else {
            println!("貨物列車{}は待たされています．", self.name);
        }
    }
    fn depart(&mut self, mediator: &mut dyn crate::station::Station) {
        println!("貨物列車{}が出発しました", self.name);
        mediator.notify_departure(&self.name);
    }
}
