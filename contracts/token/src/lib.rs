#![no_std]
use core::panic;

use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec, Address, contracttype, symbol_short};

const BALANCE : soroban_sdk::Symbol = symbol_short!("BALANCE");
const NAME: soroban_sdk::Symbol = symbol_short!("NAME");
const SYMBOL: soroban_sdk::Symbol = symbol_short!("SYMBOL");
const TOTAL: soroban_sdk::Symbol = symbol_short!("TOTAL");



#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub total_supply: i128,
}

// Token data structure
#[contractimpl]
impl Contract {
    pub fn initialize(
        env: Env,
        name: String,
        symbol: String,
        total_supply: i128,
        admin: Address,
    ) {
        admin.require_auth();

        if total_supply <= 0 {
            panic!("Total supply harus lebih dari 0");
        }

        env.storage().instance().set(&NAME, &name);
        env.storage().instance().set(&SYMBOL, &symbol);
        env.storage().instance().set(&TOTAL, &total_supply);

        env.storage().instance().set(&BALANCE, &total_supply);
    }

    pub fn get_name(env: Env) -> String {
        env.storage().instance().get(&NAME).unwrap()
    }

    pub fn get_symbol(env: Env) -> String {
        env.storage().instance().get(&SYMBOL).unwrap()
    }

    pub fn get_total_supply(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL).unwrap()
    }

    // Get balance
    pub fn get_balance(env: Env) -> i128 {
        env.storage().instance().get(&BALANCE).unwrap_or(0)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // Verify authorization
        from.require_auth();

        // Validasi amount
        if amount <= 0 {
            panic!("Amount harus lebih dari 0");
        }

        // Get current balances (simplified)
        let balance: i128 = env.storage().instance().get(&BALANCE).unwrap_or(0);

        // Check sufficient balance
        if balance < amount {
            panic!("Balance tidak cukup");
        }

        // Update balance (simplified version)
        let new_balance = balance - amount;
        env.storage().instance().set(&BALANCE, &new_balance);
    }

}

mod test;
