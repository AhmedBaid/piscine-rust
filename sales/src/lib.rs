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
    items: Vec<(String, f32)>,
    receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let proc: Vec<(String, f32)> = s
            .products
            .clone()
            .into_iter()
            .filter(|prod| prod.0 == ele)
            .collect();
        self.items.push(proc[0].clone());
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut res: Vec<f32> = Vec::new();

        let total: f32 = self.items.iter().map(|(_, p)| *p).sum();

        let free_count = self.items.len() / 3;

        let mut prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let discount_total: f32 = prices.iter().take(free_count).sum();

        let pourc = discount_total / total;

        for (_, price) in self.items.iter() {
            let x = price * (1.0 - pourc);
            res.push((x * 100.0).round() / 100.0);
        }

        res.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = res.clone();
        res
    }
}
