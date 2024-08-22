#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]
extern crate alloc;

use crate::erc20::{Erc20, Erc20Params};
use alloc::vec::Vec;
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod erc20;

struct TokenParams;

macro_rules! env_or {
    ($name:expr, $default:expr) => {
        match option_env!($name) {
            Some(v) => v,
            None => $default,
        }
    };
}

/// Immutable definitions
impl Erc20Params for TokenParams {
    const NAME: &'static str = env_or!("NAME", "AERES");
    const SYMBOL: &'static str = env_or!("SYMBOL", "ARS");
    const DECIMALS: u8 = 18;
}

// The contract
sol_storage! {
    #[entrypoint]
    struct Token {
        #[borrow] // Allows erc20 to access Token's storage and make calls
        Erc20<TokenParams> erc20;
    }
}

#[external]
#[inherit(Erc20<TokenParams>)]
impl Token {
    pub fn mint(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        self.erc20.mint(msg::sender(), amount);
        Ok(())
    }

    pub fn burn(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        self.erc20.burn(msg::sender(), amount)?;
        Ok(())
    }
}

#[cfg(feature = "export-abi")]
fn main() {
    print_abi("UNLICENSED", "pragma solidity ^0.8.20;")
}
