mod handler_func;

pub use handler_func::HandlerFunc;

/// リクエストに対する関数の失敗を通知するためのエラー
#[derive(Debug, Clone)]
pub struct ProcessError<T> {
    pub request: T,
}

/// ハンドラ
pub struct Handler<T> {
    func: Box<dyn HandlerFunc<T>>,
    next: Option<Box<Handler<T>>>,
    failed_next: Option<Box<Handler<T>>>,
}

impl<T> Handler<T> {
    /// コンストラクタ．関数を引数にとってビルダーを返す．
    pub fn new<F: HandlerFunc<T> + 'static>(func: F) -> HandlerBuilder<T> {
        HandlerBuilder::new(func)
    }
    /// 再帰して自身をクローン．
    pub fn recur_clone(&self) -> Self {
        let func = dyn_clone::clone_box(&*self.func);
        let next = match self.next.as_ref() {
            Some(next_handler) => Some(Box::new(next_handler.recur_clone())),
            None => None,
        };
        let failed_next = match self.failed_next.as_ref() {
            Some(failed_next_handler) => Some(Box::new(failed_next_handler.recur_clone())),
            None => None,
        };

        Handler {
            func,
            next,
            failed_next,
        }
    }
    /// ハンドラーを再帰的に実行．
    pub fn execute(&self, request: T) -> T {
        // ハンドラ自体の関数を実行
        let request_res = (self.func)(request);

        match request_res {
            Ok(request) => {
                // 次のハンドラを持っている場合
                if let Some(next) = self.next.as_ref() {
                    next.execute(request)
                } else {
                    request
                }
            }
            Err(e) => {
                let ProcessError { request } = e;

                // 失敗したときの次のハンドラを持っている場合
                if let Some(failed_next) = self.failed_next.as_ref() {
                    failed_next.execute(request)
                } else {
                    request
                }
            }
        }
    }
}

impl<T> Clone for Handler<T> {
    fn clone(&self) -> Self {
        self.recur_clone()
    }
}

pub struct HandlerBuilder<T> {
    func: Box<dyn HandlerFunc<T>>,
    next: Option<Box<Handler<T>>>,
    failed_next: Option<Box<Handler<T>>>,
}

impl<T> Default for HandlerBuilder<T> {
    fn default() -> Self {
        Self {
            func: Box::new(|request: T| Ok(request)),
            next: None,
            failed_next: None,
        }
    }
}

impl<T> HandlerBuilder<T> {
    pub fn new<F: HandlerFunc<T> + 'static>(func: F) -> Self {
        let func = Box::new(func) as Box<dyn HandlerFunc<T>>;

        HandlerBuilder {
            func,
            next: None,
            failed_next: None,
        }
    }
    pub fn next(&mut self, handler: Handler<T>) -> &mut Self {
        self.next = Some(Box::new(handler));
        self
    }
    pub fn failed_next(&mut self, handler: Handler<T>) -> &mut Self {
        self.failed_next = Some(Box::new(handler));
        self
    }
    pub fn build(&mut self) -> Handler<T> {
        let HandlerBuilder {
            func,
            next,
            failed_next,
        } = std::mem::take(self);

        Handler {
            func,
            next,
            failed_next,
        }
    }
}
