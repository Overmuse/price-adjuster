use chrono::{Date, Utc};

trait Adjust {
    fn adjustment_factor(&self, raw_price: f64) -> f64;
}

pub struct Dividend {
    date: Date<Utc>,
    amount: f64,
}

impl Dividend {
    fn new(date: Date<Utc>, amount: f64) -> Self {
        Self { date, amount }
    }
}

impl Adjust for Dividend {
    fn adjustment_factor(&self, raw_price: f64) -> f64 {
        (raw_price + self.amount) / raw_price
    }
}

pub struct Split {
    date: Date<Utc>,
    amount: f64,
}

impl Split {
    fn new(date: Date<Utc>, amount: f64) -> Self {
        Self { date, amount }
    }
}

impl Adjust for Split {
    fn adjustment_factor(&self, _raw_price: f64) -> f64 {
        self.amount
    }
}

fn cumulative_adjustment<T>(events: &[T]) -> Vec<f64>
where
    T: Adjust,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn dividends_work() {
        let div = Dividend::new(Utc.ymd(2014, 8, 7), 0.47);
        assert_eq!((div.adjustment_factor(94.48) * 1e5).round() / 1e5, 1.00497);
    }

    #[test]
    fn splits_work() {
        let split = Split::new(Utc.ymd(2014, 8, 7), 3.0 / 2.0);
        assert_eq!(split.adjustment_factor(94.48), 1.5);
    }
}
