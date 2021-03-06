//! Cell types

use crate::gw_common::sparse_merkle_tree::H256;
use crate::gw_types::packed::{
    ChallengeLockArgs, CustodianLockArgs, DepositLockArgs, Script, StakeLockArgs,
    WithdrawalLockArgs,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CellValue {
    pub sudt_script_hash: H256,
    pub amount: u128,
    pub capacity: u64,
}

#[derive(Debug)]
pub struct WithdrawalCell {
    pub index: usize,
    pub args: WithdrawalLockArgs,
    pub value: CellValue,
}

#[derive(Clone)]
pub struct DepositRequestCell {
    pub index: usize,
    pub args: DepositLockArgs,
    pub value: CellValue,
    pub account_script: Script,
    pub account_script_hash: H256,
}

#[derive(Debug)]
pub struct CustodianCell {
    pub index: usize,
    pub args: CustodianLockArgs,
    pub value: CellValue,
}

pub struct StakeCell {
    pub index: usize,
    pub args: StakeLockArgs,
    pub capacity: u64,
}

pub struct ChallengeCell {
    pub index: usize,
    pub args: ChallengeLockArgs,
    pub value: CellValue,
}

pub struct BurnCell {
    pub index: usize,
    pub value: CellValue,
}
