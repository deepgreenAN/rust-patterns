fn main() {
    use factory_method::{pizza, pizza_store::PizzaStore};

    let pizza_store = PizzaStore;

    let pizza = pizza_store.order_pizza(pizza::new_york_cheese_factory);

    println!("イーサンの注文は: {}", pizza.name());

    let pizza = pizza_store.order_pizza(pizza::chicago_cheese_factory);

    println!("ジョエルの注文は: {}", pizza.name());
}
