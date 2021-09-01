use crate::comp_models;

/// Make the request to the Compound Finance API for accounts
pub async fn consume_api() -> Result<Vec<String>, reqwest::Error> {
	let mut liq_accs: Vec<String> = Vec::new();
	// Request underwater accounts from Compound
	let json_res = reqwest::get("https://api.compound.finance/api/v2/account?max_health[value]=1.0&page_number=1&page_size=100&block_number=0")
		.await?
		.json::<comp_models::CompAccounts>()
		.await?;

	// Separate accounts with 1.0 > Health > 0.0
	for account in json_res.accounts {
		let float_health = account.health.value.parse::<f64>()
			.unwrap();

		if float_health < 1.0 && float_health > 0.0 {
            // Profit calculations
            for token in account.tokens {
                borrowed = token
                    .borrow_balance_underlying
                    .value
                    .parse::<i64>()
                    .unwrap();

                collat = token
                    .supply_balance_underlying
                    .value
                    .parse::<i64>()
                    .unwrap();

                borrowed_in_eth = borrowed * oracle::get_borrowed_value();
                collat_in_eth = collat * oracle::get_collat_value();

            }             

			liq_accs.push(account.address);
		}
	}

	Ok(liq_accs)
}


