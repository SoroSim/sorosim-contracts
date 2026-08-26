#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

const TOKEN_A: Symbol = symbol_short!("TOKEN_A");
const TOKEN_B: Symbol = symbol_short!("TOKEN_B");
const RESERVE_A: Symbol = symbol_short!("RSRV_A");
const RESERVE_B: Symbol = symbol_short!("RSRV_B");
const TOTAL_SHARES: Symbol = symbol_short!("SHARES");

/// Storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Share(Address), // LP token shares per address
}

#[contract]
pub struct AmmContract;

#[contractimpl]
impl AmmContract {
    /// Initialize the AMM pool with two token addresses
    pub fn initialize(env: Env, token_a: Address, token_b: Address) {
        if env.storage().instance().has(&TOKEN_A) {
            panic!("already initialized");
        }

        env.storage().instance().set(&TOKEN_A, &token_a);
        env.storage().instance().set(&TOKEN_B, &token_b);
        env.storage().instance().set(&RESERVE_A, &0i128);
        env.storage().instance().set(&RESERVE_B, &0i128);
        env.storage().instance().set(&TOTAL_SHARES, &0i128);
    }

    /// Get the reserve amounts for both tokens
    pub fn get_reserves(env: Env) -> (i128, i128) {
        let reserve_a: i128 = env.storage().instance().get(&RESERVE_A).unwrap_or(0);
        let reserve_b: i128 = env.storage().instance().get(&RESERVE_B).unwrap_or(0);

        (reserve_a, reserve_b)
    }

    /// Get token addresses
    pub fn get_tokens(env: Env) -> (Address, Address) {
        let token_a: Address = env
            .storage()
            .instance()
            .get(&TOKEN_A)
            .unwrap_or_else(|| panic!("not initialized"));
        let token_b: Address = env
            .storage()
            .instance()
            .get(&TOKEN_B)
            .unwrap_or_else(|| panic!("not initialized"));

        (token_a, token_b)
    }

    /// Get LP token shares for an address
    pub fn get_shares(env: Env, address: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Share(address))
            .unwrap_or(0)
    }

    /// Get total LP token shares
    pub fn get_total_shares(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL_SHARES).unwrap_or(0)
    }

    /// Calculate the constant product (k = x * y)
    pub fn get_k(env: Env) -> i128 {
        let (reserve_a, reserve_b) = Self::get_reserves(env);
        reserve_a.checked_mul(reserve_b).unwrap_or(0)
    }

    /// Get the price of token A in terms of token B
    pub fn get_price_a(env: Env) -> i128 {
        let (reserve_a, reserve_b) = Self::get_reserves(env);
        if reserve_a == 0 {
            panic!("no liquidity");
        }
        reserve_b / reserve_a
    }

    /// Get the price of token B in terms of token A
    pub fn get_price_b(env: Env) -> i128 {
        let (reserve_a, reserve_b) = Self::get_reserves(env);
        if reserve_b == 0 {
            panic!("no liquidity");
        }
        reserve_a / reserve_b
    }

    /// Add liquidity to the pool
    pub fn add_liquidity(env: Env, provider: Address, amount_a: i128, amount_b: i128) -> i128 {
        provider.require_auth();

        if amount_a <= 0 || amount_b <= 0 {
            panic!("amounts must be positive");
        }

        let (reserve_a, reserve_b) = Self::get_reserves(env.clone());
        let total_shares = Self::get_total_shares(env.clone());

        let shares_to_mint = if total_shares == 0 {
            // First liquidity provider gets sqrt(amount_a * amount_b) shares
            let product = amount_a
                .checked_mul(amount_b)
                .unwrap_or_else(|| panic!("overflow"));
            Self::sqrt(product)
        } else {
            // Calculate shares proportional to existing pool
            let share_a = (amount_a * total_shares) / reserve_a;
            let share_b = (amount_b * total_shares) / reserve_b;
            // Use minimum to maintain ratio
            if share_a < share_b {
                share_a
            } else {
                share_b
            }
        };

        if shares_to_mint <= 0 {
            panic!("insufficient liquidity");
        }

        // Update reserves
        env.storage()
            .instance()
            .set(&RESERVE_A, &(reserve_a + amount_a));
        env.storage()
            .instance()
            .set(&RESERVE_B, &(reserve_b + amount_b));

        // Update shares
        let provider_shares = Self::get_shares(env.clone(), provider.clone());
        env.storage().persistent().set(
            &DataKey::Share(provider.clone()),
            &(provider_shares + shares_to_mint),
        );

        env.storage()
            .instance()
            .set(&TOTAL_SHARES, &(total_shares + shares_to_mint));

        shares_to_mint
    }

    /// Remove liquidity from the pool
    pub fn remove_liquidity(env: Env, provider: Address, shares: i128) -> (i128, i128) {
        provider.require_auth();

        if shares <= 0 {
            panic!("shares must be positive");
        }

        let provider_shares = Self::get_shares(env.clone(), provider.clone());
        if provider_shares < shares {
            panic!("insufficient shares");
        }

        let (reserve_a, reserve_b) = Self::get_reserves(env.clone());
        let total_shares = Self::get_total_shares(env.clone());

        // Calculate proportional amounts to return
        let amount_a = (shares * reserve_a) / total_shares;
        let amount_b = (shares * reserve_b) / total_shares;

        if amount_a <= 0 || amount_b <= 0 {
            panic!("insufficient liquidity");
        }

        // Update reserves
        env.storage()
            .instance()
            .set(&RESERVE_A, &(reserve_a - amount_a));
        env.storage()
            .instance()
            .set(&RESERVE_B, &(reserve_b - amount_b));

        // Update shares
        env.storage().persistent().set(
            &DataKey::Share(provider.clone()),
            &(provider_shares - shares),
        );

        env.storage()
            .instance()
            .set(&TOTAL_SHARES, &(total_shares - shares));

        (amount_a, amount_b)
    }

    /// Swap token A for token B
    pub fn swap_a_for_b(
        env: Env,
        user: Address,
        amount_a_in: i128,
        min_amount_b_out: i128,
    ) -> i128 {
        user.require_auth();

        if amount_a_in <= 0 {
            panic!("amount must be positive");
        }

        let (reserve_a, reserve_b) = Self::get_reserves(env.clone());

        if reserve_a == 0 || reserve_b == 0 {
            panic!("no liquidity");
        }

        // Calculate output using constant product formula: x * y = k
        // amount_out = (amount_in * reserve_out) / (reserve_in + amount_in)
        let amount_b_out = (amount_a_in * reserve_b) / (reserve_a + amount_a_in);

        if amount_b_out < min_amount_b_out {
            panic!("slippage exceeded");
        }

        // Update reserves
        env.storage()
            .instance()
            .set(&RESERVE_A, &(reserve_a + amount_a_in));
        env.storage()
            .instance()
            .set(&RESERVE_B, &(reserve_b - amount_b_out));

        amount_b_out
    }

    /// Swap token B for token A
    pub fn swap_b_for_a(
        env: Env,
        user: Address,
        amount_b_in: i128,
        min_amount_a_out: i128,
    ) -> i128 {
        user.require_auth();

        if amount_b_in <= 0 {
            panic!("amount must be positive");
        }

        let (reserve_a, reserve_b) = Self::get_reserves(env.clone());

        if reserve_a == 0 || reserve_b == 0 {
            panic!("no liquidity");
        }

        // Calculate output using constant product formula
        let amount_a_out = (amount_b_in * reserve_a) / (reserve_b + amount_b_in);

        if amount_a_out < min_amount_a_out {
            panic!("slippage exceeded");
        }

        // Update reserves
        env.storage()
            .instance()
            .set(&RESERVE_A, &(reserve_a - amount_a_out));
        env.storage()
            .instance()
            .set(&RESERVE_B, &(reserve_b + amount_b_in));

        amount_a_out
    }

    /// Helper function to calculate integer square root
    fn sqrt(x: i128) -> i128 {
        if x == 0 {
            return 0;
        }

        let mut z = (x + 1) / 2;
        let mut y = x;

        while z < y {
            y = z;
            z = (x / z + z) / 2;
        }

        y
    }
}
