pub mod middlewares;
pub mod structs;
pub mod traits;

// gRPC packages
pub mod api {
    tonic::include_proto!("trading");
}

/* 
pub fn test_proto() {
    let _order = trading::Order {
        id: "".into(),
        owner: "".into(),
        quantity: 0,
        price: 0,
        executed: 0,
        timestamp: Some(prost_types::Timestamp { seconds: 0, nanos: 0 }),
        side: trading::Side::Ask,
        mode: trading::Mode::Limit,
    };
} */