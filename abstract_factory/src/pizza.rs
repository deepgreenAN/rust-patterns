use crate::factory::Factory;

pub trait Pizza {
    fn name(&self) -> &str;
    fn create_from_factory<F: Factory>(factory: F) -> Self;
    fn prepare(&self);
    fn bake(&self) {
        println!("350度で25分間焼く");
    }
    fn cut(&self) {
        println!("扇形に切り分ける");
    }
    fn box_(&self) {
        println!("箱に入れる");
    }
}

pub struct CheesePizza {
    name: String,
    dough: String,
    sauce: String,
    toppings: Vec<String>,
}

impl Pizza for CheesePizza {
    fn create_from_factory<F: Factory>(factory: F) -> Self {
        let mut toppings = Vec::new();
        toppings.push(factory.cheese_from_factory());

        CheesePizza {
            name: format!("{}チーズピザ", factory.factory_name()),
            dough: factory.dough_from_factory(),
            sauce: factory.sauce_from_factory(),
            toppings,
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn prepare(&self) {
        println!("{}を下処理", self.name);
        println!("{}を作成", self.dough);
        println!("{}を追加", self.sauce);
        println!("トッピングを追加");

        for topping in &self.toppings {
            println!("{}を追加", topping);
        }
    }
}

pub struct ClamPizza {
    name: String,
    dough: String,
    sauce: String,
    clam: String,
    toppings: Vec<String>,
}

impl Pizza for ClamPizza {
    fn create_from_factory<F: Factory>(factory: F) -> Self {
        Self {
            name: format!("{}クラムピザ", factory.factory_name()),
            dough: factory.dough_from_factory(),
            sauce: factory.sauce_from_factory(),
            clam: factory.clam_from_factory(),
            toppings: Vec::new(),
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn prepare(&self) {
        println!("{}を下処理", self.name);
        println!("{}を作成", self.dough);
        println!("{}を追加", self.sauce);
        println!("{}を追加", self.clam);
        println!("トッピングを追加");

        for topping in &self.toppings {
            println!("{}を追加", topping);
        }
    }
}
