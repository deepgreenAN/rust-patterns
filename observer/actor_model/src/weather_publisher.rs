use actix::prelude::*;

/// オブザーバーのアクターに渡すメッセージ．返り値は()型となる．
#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct WeatherInfo {
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
}

/// パブリッシャーにオブザーバーを追加するときのメッセージ．
#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct SubscribeMsg(pub Recipient<WeatherInfo>);

/// パブリッシャーからオブザーバーを削除するときのメッセージ．
#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct UnSubscribeMsg(pub Recipient<WeatherInfo>);

pub struct WeatherPublisher {
    observers: Vec<Recipient<WeatherInfo>>,
}

impl WeatherPublisher {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
}

impl Actor for WeatherPublisher {
    type Context = Context<Self>;
}

/// SubscribeMsgに関する処理を実装
impl Handler<SubscribeMsg> for WeatherPublisher {
    type Result = ();
    fn handle(&mut self, msg: SubscribeMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.observers.push(msg.0);
    }
}

/// UnSubscribeMsgに関する処理を実装
impl Handler<UnSubscribeMsg> for WeatherPublisher {
    type Result = ();
    fn handle(&mut self, msg: UnSubscribeMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.observers.retain(|item| item != &msg.0)
    }
}

/// WetherInfoに関する処理を実装
impl Handler<WeatherInfo> for WeatherPublisher {
    type Result = ();
    fn handle(&mut self, msg: WeatherInfo, _ctx: &mut Self::Context) -> Self::Result {
        for observer in self.observers.iter() {
            observer.do_send(msg.clone());
        }

        // System::current().stop();  // あったほうがいいかもしれない
    }
}
