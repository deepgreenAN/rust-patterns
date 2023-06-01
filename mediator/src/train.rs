mod freight;
mod passenger;

pub use freight::FreightTrain;
pub use passenger::PassengerTrain;

use crate::station::Station;

/// コンボーネントが実装するトレイト．他のコンボ―ネントとのやり取りをメディエイターに委譲する．
/// 多くのコンポーネントが実装すべきトレイトであり，Mediatorパターンの文脈ではあまり変更を想定されていないはず．
pub trait Train {
    fn name(&self) -> &str;
    fn arrive(&mut self, mediator: &mut dyn Station);
    fn depart(&mut self, mediator: &mut dyn Station);
}
