use reqwest;
use dotenv::dotenv;
use tokio::sync::watch;
use tokio::time::{
	sleep,
	Duration,
};
use ethers::prelude::*;
use std::sync::{
	Arc,
	Mutex,
};

mod config;
mod comp_models;
mod comp;
mod txn_executor;
mod platforms;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Config struct
	dotenv().ok();
	let cfg = crate::config::config::Config::config_from_env()
		.unwrap();

	// Channel for pushing liquidatable accounts
	let (liq_tx, mut liq_rx) = watch::channel("hotAddr".to_string());

	// Eth Client 
	let ws = Ws::connect(cfg.node_addr.clone()).await?;
	let provider = Provider::new(ws);
	let provider = provider.interval(Duration::from_millis(500));
	let wallet: LocalWallet = std::fs::read_to_string(cfg.priv_key)?
		.parse()?;
	let address = wallet.address();
	let client = SignerMiddleware::new(provider, wallet);
	let client = NonceManagerMiddleware::new(client, address);
	let client = Arc::new(client);

	// Make vec for tasks
	let mut sub_tasks = vec![];

	// Parse accounts from Compound Finance API
	let liq_parse = tokio::task::spawn(async move{
		loop { 
			// Get hot addrs from this cycle
			let addrs = comp::consume_api()
				.await
				.unwrap();

			// Send addrs over to other process
			for addr in addrs {
				liq_tx.send(addr)
					.unwrap();
			}

			sleep(Duration::from_millis(500)).await;
		}
	});

	// Liquidation Execution
	let txn_exec = tokio::task::spawn(async move {
		while liq_rx.changed().await.is_ok() {
			let hot_addr = liq_rx.borrow().clone();
			txn_executor::process(hot_addr, client.clone())
			.await
				.unwrap();
		}
	});

	sub_tasks.push(liq_parse);
	sub_tasks.push(txn_exec);

	for handle in sub_tasks {
		handle.await.unwrap();
	}
	
	Ok(())
}
