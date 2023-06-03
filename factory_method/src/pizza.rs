pub trait PizzaName {
    fn name(&self) -> String;
}

pub struct Pizza<T: PizzaName> {
    name: T,
    dough: String,
    sauce: String,
    toppings: Vec<String>,
}

impl<T: PizzaName> Pizza<T> {
    pub fn prepare(&self) {
        println!("{}を下処理", self.name.name());
        println!("{}を作成", self.dough);
        println!("{}を追加", self.sauce);
        println!("トッピングを追加");

        for topping in &self.toppings {
            println!("{}を追加", topping);
        }
    }
    pub fn bake(&self) {
        println!("350度で25分間焼く");
    }
    pub fn cut(&self) {
        println!("扇形に切り分ける");
    }
    pub fn box_(&self) {
        println!("箱に入れる");
    }
    pub fn name(&self) -> String {
        self.name.name()
    }
}

// -------------------------------------------------------------------------------------------------
// ピザと対応するファクトリ．ここが変更・作成される．

pub struct NYStyleCheesePizza;

impl PizzaName for NYStyleCheesePizza {
    fn name(&self) -> String {
        "ニューヨークスタイルのソース＆チーズピザ".to_string()
    }
}

pub fn new_york_cheese_factory() -> Pizza<NYStyleCheesePizza> {
    Pizza {
        name: NYStyleCheesePizza,
        dough: "薄いクラスト生地".to_string(),
        sauce: "マリナラソース".to_string(),
        toppings: vec!["粉レッジャーノチーズ".to_string()],
    }
}

pub struct ChicagoStyleCheesePizza;

impl PizzaName for ChicagoStyleCheesePizza {
    fn name(&self) -> String {
        "シカゴスタイルのディープデッシュチーズピザ".to_string()
    }
}

pub fn chicago_cheese_factory() -> Pizza<ChicagoStyleCheesePizza> {
    Pizza {
        name: ChicagoStyleCheesePizza,
        dough: "極厚クラスト生地".to_string(),
        sauce: "プラムトマトソース".to_string(),
        toppings: vec!["刻んだモッツァレラチーズ".to_string()],
    }
}
