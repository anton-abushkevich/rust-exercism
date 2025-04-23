use time::{Duration, PrimitiveDateTime as DateTime};

pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(Duration::seconds(1_000_000_000))
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;
    use super::*;

    #[test]
    fn check() {
        println!("{}", after(datetime!(2000-01-01 0:00)))
    }
}