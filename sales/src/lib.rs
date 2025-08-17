#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Self {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for (product_name, product_price) in &s.products {
            if product_name == &ele {
                self.items.push((product_name.clone(), *product_price));
                break;
            }
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let prices: Vec<f32> = self.items.iter().map(|item| item.1).collect();
        let num_free_items = prices.len() / 3;
        if num_free_items == 0 {
            self.receipt = prices;
            return self.receipt.clone();
        }
        let original_total: f32 = prices.iter().sum();
        let mut sorted_prices = prices.clone();
        sorted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let total_discount: f32 = sorted_prices.iter().take(num_free_items).sum();
        let discount_ratio = total_discount / original_total;
        let mut final_receipt: Vec<f32> = prices.iter().map(|price| {
                let new_price = price * (1.0 - discount_ratio);
                (new_price * 100.0).round() / 100.0
            }).collect();

        final_receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = final_receipt.clone();
        self.receipt.clone()
    }
}