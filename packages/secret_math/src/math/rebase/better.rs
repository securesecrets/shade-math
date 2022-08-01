use std::ops::{Add, Div};
use cosmwasm_std::StdResult;
use ethnum::U256;

use crate::core::muldiv;

use super::Rebase;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
/// Rebase that uses U256 for math, resulting in better performance.
pub struct BtrRebase {
    pub elastic: U256,
    pub base: U256,
}

impl From<Rebase> for BtrRebase {
    fn from(r: Rebase) -> Self {
        BtrRebase { elastic: U256::from_be_bytes(r.base.to_be_bytes()), base: U256::from_be_bytes(r.elastic.to_be_bytes()) }
    }
}

impl Default for BtrRebase {
    fn default() -> Self {
        Self::new()
    }
}

impl BtrRebase {
    pub fn new() -> Self {
        BtrRebase { elastic: U256::ZERO, base: U256::ZERO }
    }

    /// Calculates the base value in relationship to `elastic` and self
    pub fn to_base(&self, elastic: U256, round_up: bool) -> StdResult<U256> {
        let mut base: U256;
        if self.elastic == 0 {
            base = elastic;
        } else {
            base = muldiv(elastic, self.base, self.elastic)?;
            if round_up && muldiv(base, self.elastic, self.base)? < elastic {
                base += U256::ONE;
            }
        }
        Ok(base)
    }

    /// Calculates the elastic value in relationship to `base` and self
    pub fn to_elastic(&self, base: U256, round_up: bool) -> StdResult<U256> {
        let mut elastic: U256;
        if self.base == 0 {
            elastic = base;
        } else {
            elastic = muldiv(base, self.elastic, self.base)?;
            if round_up && muldiv(elastic, self.base, self.elastic)? < base {
                elastic += U256::ONE;
            }
        }
        Ok(elastic)
    }

    /// Add `elastic` to `self` and update `total.base`
    pub fn add_elastic(&mut self, elastic: U256, round_up: bool) -> StdResult<(& mut Self, U256)> {
        let base = self.to_base(elastic, round_up)?;
        self.elastic += elastic;
        self.base += base;
        Ok((self, base))
    }

    /// Sub `elastic` from `self` and update `total.base`
    pub fn sub_elastic(&mut self, elastic: U256, round_up: bool) -> StdResult<(& mut Self, U256)> {
        let base = self.to_base(elastic, round_up)?;
        self.elastic -= elastic;
        self.base -= base;
        Ok((self, base))
    }

    /// Add `base` to `total` and update `self.elastic`
    pub fn add_base(&mut self, base: U256, round_up: bool) -> StdResult<(& mut Self, U256)> {
        let elastic = self.to_elastic(base, round_up)?;
        self.elastic += elastic;
        self.base += base;
        Ok((self, elastic))
    }

    /// Sub `base` from `total` and update `self.elastic`
    pub fn sub_base(&mut self, base: U256, round_up: bool) -> StdResult<(& mut Self, U256)> {
        let elastic = self.to_elastic(base, round_up)?;
        self.elastic -= elastic;
        self.base -= base;
        Ok((self, elastic))
    }

    /// Add `elastic` and `base` to self.
    pub fn add_self(&mut self, elastic: U256, base: U256) -> StdResult<& mut Self> {
        self.elastic += elastic;
        self.base += base;
        Ok(self)
    }

    /// Subtract `elastic` and `base` from self.
    pub fn sub_self(&mut self, elastic: U256, base: U256) -> StdResult<& mut Self> {
        self.elastic -= elastic;
        self.base -= base;
        Ok(self)
    }
}
