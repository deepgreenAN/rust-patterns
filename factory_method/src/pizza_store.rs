use crate::pizza::{Pizza, PizzaName};

pub struct PizzaStore;

impl PizzaStore {
    pub fn order_pizza<F, T>(&self, factory: F) -> Pizza<T>
    where
        F: Fn() -> Pizza<T>,
        T: PizzaName,
    {
        let pizza = factory();

        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.box_();

        pizza
    }
}
