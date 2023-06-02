use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PublishError(String);

#[derive(Default)]
pub struct Publisher<T> {
    pub observer_map: HashMap<String, Box<dyn FnMut(&T)>>,
}

impl<T> Publisher<T> {
    pub fn new() -> Self {
        Self {
            observer_map: HashMap::new(),
        }
    }
    /// オブザーバーをサブスクライブするためのメソッド．引数として直接関数を渡すことができる．
    pub fn subscribe<F>(&mut self, observer_name: &str, observer: F) -> Result<(), PublishError>
    where
        F: FnMut(&T) + 'static,
    {
        let observer = Box::new(observer) as Box<dyn FnMut(&T)>;
        self.subscribe_box(observer_name, observer)
    }
    /// オブザーバーをサブスクライブするためのメソッド．引数としてBox<dyn>関数をとる．
    pub fn subscribe_box(
        &mut self,
        observer_name: &str,
        observer: Box<dyn FnMut(&T)>,
    ) -> Result<(), PublishError> {
        match self.observer_map.entry(observer_name.to_string()) {
            Entry::Occupied(_) => Err(PublishError("オブザーバー名が重複しています".to_string())),
            Entry::Vacant(v) => {
                v.insert(observer);
                Ok(())
            }
        }
    }
    /// オブザーバーをリストから外すためのメソッド．外したオブザーバーを返す．
    pub fn unsubscribe(&mut self, observer_name: &str) -> Result<Box<dyn FnMut(&T)>, PublishError> {
        match self.observer_map.remove(observer_name) {
            Some(observer) => Ok(observer),
            None => Err(PublishError(
                "その名前を持つオブザーバーは存在しません．".to_string(),
            )),
        }
    }
    /// オブザーバーにコンテキストを通知するためのメソッド．FnMutだから`&mut self`
    pub fn notify_observers(&mut self, context: &T) {
        for observer in self.observer_map.values_mut() {
            observer(context);
        }
    }
}
