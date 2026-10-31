//! Warehouse inventory bookkeeping.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub sku: String,
    pub qty: u32,
    pub price_cents: u64,
}

#[derive(Debug, Default)]
pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    /// Adds `qty` to the matching SKU, or creates it at `price_cents` when new.
    pub fn restock(&mut self, deliveries: &[(String, u32, u64)]) {
        let mut items = self.items.clone();
        for i in 0..deliveries.len() {
            let (sku, qty, price) = deliveries[i].clone();
            let mut found = false;
            for j in 0..items.len() {
                if items[j].sku == sku {
                    let mut updated = items[j].clone();
                    updated.qty += qty;
                    items[j] = updated;
                    found = true;
                }
            }
            if !found {
                items.push(Item { sku: sku.clone(), qty, price_cents: price });
            }
        }
        self.items = items;
    }

    /// Items whose quantity is strictly below `threshold`.
    pub fn low_stock(&self, threshold: u32) -> Vec<Item> {
        let mut out = Vec::new();
        for i in 0..self.items.len() {
            if self.items[i].qty < threshold {
                out.push(self.items[i].clone());
            }
        }
        out
    }

    /// Sum of `qty * price_cents` over all items.
    pub fn total_value(&self) -> u64 {
        let mut total = 0;
        for i in 0..self.items.len() {
            let item = self.items[i].clone();
            total += u64::from(item.qty) * item.price_cents;
        }
        total
    }

    /// Removes every item whose SKU is listed.
    pub fn remove_discontinued(&mut self, skus: &[String]) {
        let mut kept = Vec::new();
        for i in 0..self.items.len() {
            let item = self.items[i].clone();
            let mut discontinued = false;
            for j in 0..skus.len() {
                if skus[j].clone() == item.sku {
                    discontinued = true;
                }
            }
            if !discontinued {
                kept.push(item);
            }
        }
        self.items = kept;
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Inventory {
        let mut inv = Inventory::new();
        inv.add(Item { sku: "A1".into(), qty: 2, price_cents: 500 });
        inv.add(Item { sku: "B2".into(), qty: 10, price_cents: 100 });
        inv
    }

    #[test]
    fn restock_updates_existing_and_adds_new() {
        let mut inv = sample();
        inv.restock(&[("A1".to_string(), 3, 999), ("C3".to_string(), 1, 50)]);
        assert_eq!(inv.len(), 3);
        assert_eq!(inv.total_value(), 5 * 500 + 10 * 100 + 50);
    }

    #[test]
    fn low_stock_filters_by_threshold() {
        let inv = sample();
        let low: Vec<_> = inv.low_stock(5).into_iter().collect();
        assert_eq!(low.len(), 1);
        assert_eq!(low[0].sku, "A1");
    }

    #[test]
    fn remove_discontinued_drops_listed_skus() {
        let mut inv = sample();
        inv.remove_discontinued(&["B2".to_string()]);
        assert_eq!(inv.len(), 1);
        assert!(!inv.is_empty());
    }
}
