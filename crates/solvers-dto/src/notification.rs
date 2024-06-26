use {
    super::serialize,
    number::serialization::HexOrDecimalU256,
    serde::Deserialize,
    serde_with::{serde_as, DisplayFromStr},
    std::collections::BTreeSet,
    web3::types::{AccessList, H160, H256, U256},
};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub auction_id: Option<i64>,
    pub solution_id: Option<u64>,
    #[serde(flatten)]
    pub kind: Kind,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Kind {
    Timeout,
    EmptySolution,
    DuplicatedSolutionId,
    #[serde(rename_all = "camelCase")]
    SimulationFailed {
        block: BlockNo,
        tx: Tx,
        succeeded_once: bool,
    },
    ZeroScore,
    ScoreHigherThanQuality {
        #[serde_as(as = "HexOrDecimalU256")]
        score: U256,
        #[serde_as(as = "HexOrDecimalU256")]
        quality: U256,
    },
    SuccessProbabilityOutOfRange {
        probability: f64,
    },
    #[serde(rename_all = "camelCase")]
    ObjectiveValueNonPositive {
        #[serde_as(as = "HexOrDecimalU256")]
        quality: U256,
        #[serde_as(as = "HexOrDecimalU256")]
        gas_cost: U256,
    },
    NonBufferableTokensUsed {
        tokens: BTreeSet<H160>,
    },
    SolverAccountInsufficientBalance {
        #[serde_as(as = "HexOrDecimalU256")]
        required: U256,
    },
    Success {
        transaction: H256,
    },
    Revert {
        transaction: H256,
    },
    DriverError {
        reason: String,
    },
    Cancelled,
    Fail,
    PostprocessingTimedOut,
}

type BlockNo = u64;

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Tx {
    pub from: H160,
    pub to: H160,
    #[serde_as(as = "serialize::Hex")]
    pub input: Vec<u8>,
    #[serde_as(as = "HexOrDecimalU256")]
    pub value: U256,
    pub access_list: AccessList,
}
