use crate::factory::Factory;
use crate::pizza::Pizza;

pub struct PizzaStore;

impl PizzaStore {
    pub fn order_pizza<F: Factory, T: Pizza>(&self, factory: F) -> T {
        let pizza = T::create_from_factory(factory);

        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.box_();

        pizza
    }
}
