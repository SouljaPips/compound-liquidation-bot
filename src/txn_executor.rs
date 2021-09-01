use ethers::prelude::*;
use std::sync::Arc;

//use crate::platforms::*;

pub async fn process(
	address: String,
	client: Arc<NonceManagerMiddleware<SignerMiddleware<Provider<Ws>,Wallet<ethers::core::k256::ecdsa::SigningKey>>>>
) -> Result<(), Box<dyn std::error::Error>> {
	println!("Found new account");
	Ok(())
}