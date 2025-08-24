// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(p_name: String, q: u32, unit_price: u32) -> Self {
        Self::check_product_name(&p_name);
        Self::check_product_price(&unit_price);
        Self::check_quantity(&q);
        Order {
            product_name: p_name,
            quantity: q,
            unit_price: unit_price,
        }
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, p: String) {
        Self::check_product_name(&p);
        self.product_name = p;
    }

    pub fn set_quantity(&mut self, q: u32) {
        Self::check_quantity(&q);
        self.quantity = q;
    }

    fn check_product_name(p: &String) {
        if p.is_empty() || p.len() > 300 {
            panic!("Invalid product name");
        }
    }

    fn check_quantity(q: &u32) {
        if *q <= 0 {
            panic!("Invalid quantity");
        }
    }

    fn check_product_price(p: &u32) {
        if *p <= 0 {
            panic!("Invalid product price");
        }
    }

    pub fn set_unit_price(&mut self, p: u32) {
        Self::check_product_price(&p);
        self.unit_price = p;
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}