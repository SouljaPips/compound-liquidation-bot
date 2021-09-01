use ethers::{
	abi::{ Abi, FunctionExt },
	contract::{ abigen, BaseContract, ContractError },
	types::{Address, Call, CallType, U256},
};

use std::{
	collections::HashMap,
	io::{ stdout, Write },
};

abigen!(
	Comptroller,
	"abi/comptroller.json",
	methods {
        // TODO: Fix bug in ethers-rs so that we can rename them properly
        borrowGuardianPaused(address) as borrow_guardian_paused2;
        mintGuardianPaused(address) as mint_guardian_paused2;
        actionGuardianPaused(address) as action_paused2;
    },
);

abigen!(CToken, "abi/ctoken.json",);

#[derive (Debug, Clone)]
/// Compound Object
pub struct Compound {
	ctoken: BaseContract,
	cether: BaseContract,
	comptroller: BaseContract,
	ctoken_to_ctoken: HashMap<Address, Address>
}

impl Compound {
	pub fn new<T: IntoIterator<Item = (Address, Address)>>(ctoken_to_ctoken: T) -> Self {
		Self {
			ctoken: BaseContract::from({
				serde_json::from_str::<Abi>(include_str!("../../abi/ctoken.json"))
					.expect("ctoken abi parse failure")
			}),
			cether: BaseContract::from({
				serde_json::from_str::<Abi>(include_str!("../../abi/cether.json"))
					.expect("cether abi parse failure")
			}),
			comptroller: BaseContract::from({
				serde_json::from_str::<Abi>(include_str!("../../abi/comptroller.json"))
					.expect("comptroller abi parse failure")
			}),
			ctoken_to_ctoken: ctoken_to_ctoken.into_iter().collect(),
		}
	}
}


