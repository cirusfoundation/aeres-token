#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]
extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloc::string::String;
use core::marker::PhantomData;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::sol,
    evm, msg,
    prelude::*,
};
use alloy_sol_types::Result;

#[macro_export]
macro_rules! bail {
    ($error:expr) => {
        return Result::Err($error.into())
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $error:expr) => {
        if !$cond {
            $crate::bail!($error);
        }
    };
}

pub trait Erc20Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
    const DECIMALS: u8;
}

sol_storage! {
    /// Erc20 implements all ERC-20 methods.
    pub struct Erc20<T> {
        /// Maps users to balances
        mapping(address => uint256) balances;
        /// Maps users to a mapping of each spender's allowance
        mapping(address => mapping(address => uint256)) allowances;
        /// The available supply of the token
        uint256 available_supply;
        /// The total supply of the token
        uint256 total_supply;
        /// The contract owner address
        address contract_owner;
        /// Used to allow [`Erc20Params`]
        PhantomData<T> phantom;
    }
}

// Declare events and Solidity error types
sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    error InsufficientBalance(address from, uint256 have, uint256 want);
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);

    error PermissionDenied();
    error AvailableSupplyAmountExceed();
}

#[derive(SolidityError)]
pub enum Erc20Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),

    PermissionDenied(PermissionDenied),
    AvailableSupplyAmountExceed(AvailableSupplyAmountExceed),
}

// These methods aren't exposed to other contracts
impl<T: Erc20Params> Erc20<T> {
    pub fn init(
        &mut self,
        address: Address,
        total_supply: U256,
        available_supply: U256,
    ) -> Result<(), Erc20Error> {
        ensure!(
            address == self.contract_owner.get(),
            Erc20Error::PermissionDenied(PermissionDenied {})
        );
        self.total_supply.set(total_supply);
        self.available_supply.set(available_supply);
        self.mint(address, available_supply)?;
        Ok(())
    }

    pub fn transfer_impl(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<(), Erc20Error> {
        let mut sender_balance = self.balances.setter(from);
        let old_sender_balance = sender_balance.get();
        if old_sender_balance < value {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from,
                have: old_sender_balance,
                want: value,
            }));
        }
        sender_balance.set(old_sender_balance - value);
        let mut to_balance = self.balances.setter(to);
        let new_to_balance = to_balance.get() + value;
        to_balance.set(new_to_balance);
        evm::log(Transfer { from, to, value });
        Ok(())
    }

    pub fn mint(&mut self, address: Address, value: U256) -> Result<(), Erc20Error> {
        ensure!(
            address == self.contract_owner.get(),
            Erc20Error::PermissionDenied(PermissionDenied {})
        );
        let available_supply_updated = self.available_supply.get() + value;
        ensure!(
            available_supply_updated <= self.total_supply.get(),
            Erc20Error::AvailableSupplyAmountExceed(AvailableSupplyAmountExceed {})
        );
        self.available_supply.set(available_supply_updated);
        let mut owner_balance = self.balances.setter(address);
        let new_balance = owner_balance.get() + value;
        owner_balance.set(new_balance);
        evm::log(Transfer {
            from: Address::ZERO,
            to: address,
            value,
        });
        Ok(())
    }

    pub fn burn(&mut self, address: Address, value: U256) -> Result<(), Erc20Error> {
        ensure!(
            address == self.contract_owner.get(),
            Erc20Error::PermissionDenied(PermissionDenied {})
        );
        let mut owner_balance = self.balances.setter(address);
        let old_balance = owner_balance.get();
        if old_balance < value {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from: address,
                have: old_balance,
                want: value,
            }));
        }
        owner_balance.set(old_balance - value);
        self.total_supply.set(self.total_supply.get() - value);
        evm::log(Transfer {
            from: address,
            to: Address::ZERO,
            value,
        });
        Ok(())
    }
}

#[external]
impl<T: Erc20Params> Erc20<T> {
    pub fn name() -> Result<String, Erc20Error> {
        Ok(T::NAME.into())
    }

    pub fn symbol() -> Result<String, Erc20Error> {
        Ok(T::SYMBOL.into())
    }

    pub fn decimals() -> Result<u8, Erc20Error> {
        Ok(T::DECIMALS)
    }

    pub fn balance_of(&self, address: Address) -> Result<U256, Erc20Error> {
        Ok(self.balances.get(address))
    }

    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error> {
        self.transfer_impl(msg::sender(), to, value)?;
        Ok(true)
    }

    pub fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Erc20Error> {
        self.allowances.setter(msg::sender()).insert(spender, value);
        evm::log(Approval {
            owner: msg::sender(),
            spender,
            value,
        });
        Ok(true)
    }

    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Erc20Error> {
        let mut sender_allowances = self.allowances.setter(from);
        let mut allowance = sender_allowances.setter(msg::sender());
        let old_allowance = allowance.get();
        if old_allowance < value {
            return Err(Erc20Error::InsufficientAllowance(InsufficientAllowance {
                owner: from,
                spender: msg::sender(),
                have: old_allowance,
                want: value,
            }));
        }
        allowance.set(old_allowance - value);
        self.transfer_impl(from, to, value)?;
        Ok(true)
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Erc20Error> {
        Ok(self.allowances.getter(owner).get(spender))
    }
}
