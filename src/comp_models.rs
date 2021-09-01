use serde::{
	Serialize,
	Deserialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompAccounts {
    pub accounts: Vec<Account>,
    pub error: Option<serde_json::Value>,
    pub pagination_summary: PaginationSummary,
    pub request: Request,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub block_updated: Option<serde_json::Value>,
    pub health: MaxHealth,
    pub tokens: Vec<Token>,
    pub total_borrow_value_in_eth: MaxHealth,
    pub total_collateral_value_in_eth: MaxHealth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxHealth {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub address: String,
    pub borrow_balance_underlying: MaxHealth,
    pub lifetime_borrow_interest_accrued: MaxHealth,
    pub lifetime_supply_interest_accrued: MaxHealth,
    pub safe_withdraw_amount_underlying: MaxHealth,
    pub supply_balance_underlying: MaxHealth,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationSummary {
    pub page_number: i64,
    pub page_size: i64,
    pub total_entries: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub addresses: Vec<Option<serde_json::Value>>,
    pub block_number: i64,
    pub block_timestamp: i64,
    pub max_health: MaxHealth,
    pub min_borrow_value_in_eth: Option<serde_json::Value>,
    pub network: String,
    pub page_number: i64,
    pub page_size: i64,
}
