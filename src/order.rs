fn main() {
    let order = Order::new("Pizza", "Local Pizzeria", "Tech District");

    let confirmation = order.place();
    println!("{}", confirmation);
}

struct Order {
    item: String,
    restaurant: String,
    location: String,
}

impl Order {
    fn new(item: &str, restaurant: &str, location: &str) -> Self {
        Self {
            item: item.to_string(),
            restaurant: restaurant.to_string(),
            location: location.to_string(),
        }
    }

    fn place(&self) -> String {
        format!(
            "Order confirmed: {} from {} at {}.",
            self.item, self.restaurant, self.location
        )
    }
}