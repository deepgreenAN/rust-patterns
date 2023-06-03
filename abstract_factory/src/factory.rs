pub trait Factory {
    fn dough_from_factory(&self) -> String;
    fn sauce_from_factory(&self) -> String;
    fn cheese_from_factory(&self) -> String;
    fn clam_from_factory(&self) -> String;
    fn veggies(&self) -> Vec<String>;
    fn factory_name(&self) -> String;
}

pub struct NewYorkFactory;

impl Factory for NewYorkFactory {
    fn cheese_from_factory(&self) -> String {
        "粉レッジャーノ".to_string()
    }
    fn sauce_from_factory(&self) -> String {
        "マリナラソース".to_string()
    }
    fn clam_from_factory(&self) -> String {
        "新鮮なクラム".to_string()
    }
    fn dough_from_factory(&self) -> String {
        "薄いクラスト生地".to_string()
    }
    fn veggies(&self) -> Vec<String> {
        vec!["きのこ".to_string(), "たまねぎ".to_string()]
    }
    fn factory_name(&self) -> String {
        "ニューヨークスタイル".to_string()
    }
}
pub struct ChicagoFactory;

impl Factory for ChicagoFactory {
    fn cheese_from_factory(&self) -> String {
        "モッツァレラ".to_string()
    }
    fn sauce_from_factory(&self) -> String {
        "プラムトマトソース".to_string()
    }
    fn clam_from_factory(&self) -> String {
        "冷凍クラム".to_string()
    }
    fn dough_from_factory(&self) -> String {
        "極厚クラスト生地".to_string()
    }
    fn veggies(&self) -> Vec<String> {
        vec![
            "ほうれん草".to_string(),
            "ナス".to_string(),
            "ブラックオリーブ".to_string(),
        ]
    }
    fn factory_name(&self) -> String {
        "シカゴスタイル".to_string()
    }
}
