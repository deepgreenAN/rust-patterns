use crate::pizza::{Pizza, PizzaName};

pub struct PizzaStore<T: PizzaName> {
    _pizza_type: std::marker::PhantomData<T>,
}

impl<T: PizzaName> PizzaStore<T> {
    pub fn order_pizza<F>(factory: F) -> Pizza<T>
    where
        F: Fn() -> Pizza<T>,
    {
        let pizza = factory();

        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.box_();

        pizza
    }
}
