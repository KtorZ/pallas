// TODO: this should move to pallas::ledger crate at some point

use pallas_crypto::hash::Hash;
use std::hash::Hash as StdHash;
// required for derive attrs to work
use pallas_codec::minicbor::{self};

use pallas_codec::utils::{AnyUInt, Bytes, KeyValuePairs, TagWrap};
use pallas_codec::{
    minicbor::{Decode, Encode},
    utils::AnyCbor,
};

use crate::miniprotocols::Point;

use super::{Client, ClientError};

mod codec;

// https://github.com/input-output-hk/ouroboros-consensus/blob/main/ouroboros-consensus-cardano/src/shelley/Ouroboros/Consensus/Shelley/Ledger/Query.hs
#[derive(Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum BlockQuery {
    GetLedgerTip,
    GetEpochNo,
    GetNonMyopicMemberRewards(AnyCbor),
    GetCurrentPParams,
    GetProposedPParamsUpdates,
    GetStakeDistribution,
    GetUTxOByAddress(Addrs),
    GetUTxOWhole,
    DebugEpochState,
    GetCBOR(AnyCbor),
    GetFilteredDelegationsAndRewardAccounts(AnyCbor),
    GetGenesisConfig,
    DebugNewEpochState,
    DebugChainDepState,
    GetRewardProvenance,
    GetUTxOByTxIn(AnyCbor),
    GetStakePools,
    GetStakePoolParams(AnyCbor),
    GetRewardInfoPools,
    GetPoolState(AnyCbor),
    GetStakeSnapshots(AnyCbor),
    GetPoolDistr(AnyCbor),
    GetStakeDelegDeposits(AnyCbor),
    GetConstitutionHash,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum HardForkQuery {
    GetInterpreter,
    GetCurrentEra,
}

pub type Epoch = u64;
pub type Proto = u16;
pub type Era = u16;

#[derive(Debug, Clone, PartialEq)]
pub enum LedgerQuery {
    BlockQuery(Era, BlockQuery),
    HardForkQuery(HardForkQuery),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    LedgerQuery(LedgerQuery),
    GetSystemStart,
    GetChainBlockNo,
    GetChainPoint,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Coin(Coin),
    Multiasset(Coin, Multiasset<Coin>),
}

#[derive(Debug, Encode, Decode, PartialEq)]
pub struct SystemStart {
    #[n(0)]
    pub year: u32,

    #[n(1)]
    pub day_of_year: u32,

    #[n(2)]
    pub picoseconds_of_day: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RationalNumber {
    pub numerator: u64,
    pub denominator: u64,
}

pub type UnitInterval = RationalNumber;
pub type PositiveInterval = RationalNumber;

pub type ProtocolVersionMajor = u64;
pub type ProtocolVersionMinor = u64;

pub type CostModel = Vec<i64>;

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
#[cbor(map)]
pub struct CostMdls {
    #[n(0)]
    pub plutus_v1: Option<CostModel>,

    #[n(1)]
    pub plutus_v2: Option<CostModel>,
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct ExUnitPrices {
    #[n(0)]
    mem_price: PositiveInterval,

    #[n(1)]
    step_price: PositiveInterval,
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct ExUnits {
    #[n(0)]
    pub mem: u32,
    #[n(1)]
    pub steps: u64,
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
#[cbor(array)]
pub struct ProtocolParam {
    #[n(0)]
    pub minfee_a: Option<u32>,
    #[n(1)]
    pub minfee_b: Option<u32>,
    #[n(2)]
    pub max_block_body_size: Option<u32>,
    #[n(3)]
    pub max_transaction_size: Option<u32>,
    #[n(4)]
    pub max_block_header_size: Option<u32>,
    #[n(5)]
    pub key_deposit: Option<Coin>,
    #[n(6)]
    pub pool_deposit: Option<Coin>,
    #[n(7)]
    pub maximum_epoch: Option<Epoch>,
    #[n(8)]
    pub desired_number_of_stake_pools: Option<u32>,
    #[n(9)]
    pub pool_pledge_influence: Option<RationalNumber>,
    #[n(10)]
    pub expansion_rate: Option<UnitInterval>,
    #[n(11)]
    pub treasury_growth_rate: Option<UnitInterval>,
    #[n(12)]
    pub protocol_version_major: Option<ProtocolVersionMajor>,
    #[n(13)]
    pub protocol_version_minor: Option<ProtocolVersionMinor>,
    #[n(14)]
    pub min_pool_cost: Option<Coin>,
    #[n(15)]
    pub ada_per_utxo_byte: Option<Coin>,
    #[n(16)]
    pub cost_models_for_script_languages: Option<CostMdls>,
    #[n(17)]
    pub execution_costs: Option<ExUnitPrices>,
    #[n(18)]
    pub max_tx_ex_units: Option<ExUnits>,
    #[n(19)]
    pub max_block_ex_units: Option<ExUnits>,
    #[n(20)]
    pub max_value_size: Option<u32>,
    #[n(21)]
    pub collateral_percentage: Option<u32>,
    #[n(22)]
    pub max_collateral_inputs: Option<u32>,
}

#[derive(Debug, Encode, Decode, PartialEq)]
pub struct StakeDistribution {
    #[n(0)]
    pub pools: KeyValuePairs<Bytes, Pool>,
}

#[derive(Debug, Encode, Decode, PartialEq, Clone)]
pub struct Pool {
    #[n(0)]
    pub stakes: Fraction,

    #[n(1)]
    pub hashes: Bytes,
}

#[derive(Debug, Encode, Decode, PartialEq, Clone)]
pub struct Fraction {
    #[n(0)]
    pub num: u64,

    #[n(1)]
    pub dem: u64,
}

pub type Addr = Bytes;

pub type Addrs = Vec<Addr>;

pub type Coin = AnyUInt;

pub type PolicyId = Hash<28>;

pub type AssetName = Bytes;

pub type Multiasset<A> = KeyValuePairs<PolicyId, KeyValuePairs<AssetName, A>>;

#[derive(Debug, Encode, Decode, PartialEq, Clone)]
pub struct UTxOByAddress {
    #[n(0)]
    pub utxo: KeyValuePairs<UTxO, Values>,
}

// Bytes CDDL ->  #6.121([ * #6.121([ *datum ]) ])
pub type Datum = (Era, TagWrap<Bytes, 24>);

#[derive(Debug, Encode, Decode, PartialEq, Clone)]
#[cbor(map)]
pub struct Values {
    #[n(0)]
    pub address: Bytes,

    #[n(1)]
    pub amount: Value,

    #[n(2)]
    pub inline_datum: Option<Datum>,
}

#[derive(Debug, Encode, Decode, PartialEq, Clone, StdHash, Eq)]
pub struct UTxO {
    #[n(0)]
    pub transaction_id: Hash<32>,

    #[n(1)]
    pub index: AnyUInt,
}

pub async fn get_chain_point(client: &mut Client) -> Result<Point, ClientError> {
    let query = Request::GetChainPoint;
    let result = client.query(query).await?;

    Ok(result)
}

pub async fn get_current_era(client: &mut Client) -> Result<Era, ClientError> {
    let query = HardForkQuery::GetCurrentEra;
    let query = LedgerQuery::HardForkQuery(query);
    let query = Request::LedgerQuery(query);
    let result = client.query(query).await?;

    Ok(result)
}

pub async fn get_system_start(client: &mut Client) -> Result<SystemStart, ClientError> {
    let query = Request::GetSystemStart;
    let result = client.query(query).await?;

    Ok(result)
}

pub async fn get_current_pparams(
    client: &mut Client,
    era: u16,
) -> Result<Vec<ProtocolParam>, ClientError> {
    let query = BlockQuery::GetCurrentPParams;
    let query = LedgerQuery::BlockQuery(era, query);
    let query = Request::LedgerQuery(query);
    let result = client.query(query).await?;

    Ok(result)
}

pub async fn get_block_epoch_number(client: &mut Client, era: u16) -> Result<u32, ClientError> {
    let query = BlockQuery::GetEpochNo;
    let query = LedgerQuery::BlockQuery(era, query);
    let query = Request::LedgerQuery(query);
    let (result,): (_,) = client.query(query).await?;

    Ok(result)
}

pub async fn get_stake_distribution(
    client: &mut Client,
    era: u16,
) -> Result<StakeDistribution, ClientError> {
    let query = BlockQuery::GetStakeDistribution;
    let query = LedgerQuery::BlockQuery(era, query);
    let query = Request::LedgerQuery(query);
    let result = client.query(query).await?;

    Ok(result)
}

pub async fn get_utxo_by_address(
    client: &mut Client,
    era: u16,
    addrs: Addrs,
) -> Result<UTxOByAddress, ClientError> {
    let query = BlockQuery::GetUTxOByAddress(addrs);
    let query = LedgerQuery::BlockQuery(era, query);
    let query = Request::LedgerQuery(query);
    let result = client.query(query).await?;

    Ok(result)
}
