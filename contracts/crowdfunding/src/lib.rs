#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, token, Address, Env, Map, Symbol};


const CAMPAIGN_GOAL: Symbol = symbol_short!("goal");
const CAMPAIGN_DEADLINE: Symbol = symbol_short!("deadline");
const TOTAL_RAISED: Symbol = symbol_short!("raised");
const DONATIONS: Symbol = symbol_short!("donations");
const CAMPAIGN_OWNER: Symbol = symbol_short!("owner");
const XLM_TOKEN_ADDRESS: Symbol = symbol_short!("xlm_addr");
const IS_ALREADY_INIT: Symbol = symbol_short!("is_init");

#[contract]
pub struct CrowdfundingContract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl CrowdfundingContract {
     pub fn initialize(
        env: Env,
        owner: Address,    // Address creator campaign
        goal: i128,        // Target amount (stroops: 1 XLM = 10,000,000 stroops)
        deadline: u64,     // Unix timestamp kapan campaign berakhir
        xlm_token: Address, // XLM token contract address (native token di testnet)
    ) {
       // Verify owner adalah yang claim
        owner.require_auth();

        // Store campaign settings ke blockchain
        env.storage().instance().set(&CAMPAIGN_OWNER, &owner);
        env.storage().instance().set(&CAMPAIGN_GOAL, &goal);
        env.storage().instance().set(&CAMPAIGN_DEADLINE, &deadline);
        env.storage().instance().set(&TOTAL_RAISED, &0i128);
        env.storage().instance().set(&XLM_TOKEN_ADDRESS, &xlm_token);

        // Set initialization flag to true
        env.storage().instance().set(&IS_ALREADY_INIT, &true);

        // Initialize empty donations map
        // Map<Address, i128> = tracking siapa donate berapa
        let donations: Map<Address, i128> = Map::new(&env);
        env.storage().instance().set(&DONATIONS, &donations);
    }

    pub fn donate(env: Env, donor: Address, amount: i128) {
        // Verify donor authorization
        donor.require_auth();

        // Check apakah campaign masih aktif
        let deadline: u64 = env.storage().instance().get(&CAMPAIGN_DEADLINE).unwrap();
        if env.ledger().timestamp() > deadline {
            panic!("Campaign has ended");
        }

        // Validate amount harus positif
        if amount <= 0 {
            panic!("Donation amount must be positive");
        }

        // Get XLM token contract dan contract address
        let xlm_token_address: Address = env.storage().instance().get(&XLM_TOKEN_ADDRESS).unwrap();
        let xlm_token = token::Client::new(&env, &xlm_token_address);
        let contract_address = env.current_contract_address();

        // Transfer XLM dari donor ke contract ini
        xlm_token.transfer(&donor, &contract_address, &amount);

        // Update total raised
        let mut total: i128 = env.storage().instance().get(&TOTAL_RAISED).unwrap();
        total += amount;
        env.storage().instance().set(&TOTAL_RAISED, &total);

        // Track donasi individual donor
        let mut donations: Map<Address, i128> = env.storage().instance().get(&DONATIONS).unwrap();
        let current_donation = donations.get(donor.clone()).unwrap_or(0);
        donations.set(donor, current_donation + amount);
        env.storage().instance().set(&DONATIONS, &donations);
    }

    pub fn get_total_raised(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL_RAISED).unwrap_or(0)
    }

    /// Get berapa banyak specific donor sudah donate
    /// Frontend perlu pass: donor address
    pub fn get_donation(env: Env, donor: Address) -> i128 {
        let donations: Map<Address, i128> = env.storage().instance().get(&DONATIONS).unwrap();
        donations.get(donor).unwrap_or(0)
    }

    // Get initialization status - for frontend to check if contract is initialized
    pub fn get_is_already_init(env: Env) -> bool {
        env.storage().instance().get(&IS_ALREADY_INIT).unwrap_or(false)
    }
}

mod test;
