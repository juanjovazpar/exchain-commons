fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running build.rs...");
    tonic_prost_build::compile_protos("protos/order.proto")?;
    tonic_prost_build::compile_protos("protos/wallet.proto")?;
    println!("Proto compilation finished");
    
    Ok(())
}