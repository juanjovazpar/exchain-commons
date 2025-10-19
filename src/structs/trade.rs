use chrono::Utc;
use uuid::Uuid;

use crate::structs::order::{Price, Quantity, Timestamp};

/* 
    Represents a completed trade between a bid and an ask order.
    Fields:
    - `id`: Unique identifier of the trade.
    - `ask`: UUID of the ask order involved.
    - `bid`: UUID of the bid order involved.
    - `quantity`: Quantity of the asset traded.
    - `price`: Execution price of the trade.
    - `timestamp`: Time when the trade was executed (milliseconds since epoch).
*/
pub struct Trade {
    pub id: Uuid,
    pub ask: Uuid,
    pub bid: Uuid,
    pub quantity: Quantity,
    pub price: Price,
    pub timestamp: Timestamp,
}
impl Trade {
    pub fn new(ask: Uuid, bid: Uuid, quantity: Quantity, price: Price) -> Self {
        Self {
            ask,
            bid,
            quantity,
            price,
            id: Uuid::new_v4(),
            timestamp: Utc::now().timestamp_millis()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_creation() {
        let ask = Uuid::new_v4();
        let bid = Uuid::new_v4();
        let amount: Quantity = 100;
        let price: Price = 50;

        let trade = Trade::new(ask, bid, amount, price);

        assert_eq!(trade.ask, ask);
        assert_eq!(trade.bid, bid);
        assert_eq!(trade.quantity, amount);
        assert_eq!(trade.price, price);
        assert_ne!(trade.timestamp, 1);

        assert!(Uuid::parse_str(&trade.id.to_string()).is_ok());
    }
}