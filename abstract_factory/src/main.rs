use abstract_factory::pizza::Pizza;

fn main() {
    use abstract_factory::factory::{ChicagoFactory, NewYorkFactory};
    use abstract_factory::pizza::{CheesePizza, ClamPizza};
    use abstract_factory::pizza_store::PizzaStore;

    let pizza_store = PizzaStore;

    let pizza = pizza_store.order_pizza::<_, CheesePizza>(ChicagoFactory);

    println!("イーサンの注文は{}です", pizza.name());

    let pizza = pizza_store.order_pizza::<_, ClamPizza>(NewYorkFactory);

    println!("ジョエルの注文は{}です", pizza.name());
}
