pub mod middlewares;
pub mod structs;
pub mod traits;

// gRPC packages
pub mod grpc_api {
    tonic::include_proto!("trading");
    tonic::include_proto!("wallets");
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