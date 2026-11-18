//! Contracts, the `CONTRACT` nodes of the `ContractSystem` scenario.

use crate::career::scenario;
use crate::field::{ExtractError, Finite, invalid, non_empty, one_of, optional_text};
use crate::node::Node;

/// A contract's state, from `state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractState {
    /// Generated but not yet offered.
    Generated,
    /// Offered and awaiting acceptance.
    Offered,
    /// The offer lapsed.
    OfferExpired,
    /// Declined.
    Declined,
    /// Cancelled after acceptance.
    Cancelled,
    /// Accepted and in progress.
    Active,
    /// Completed.
    Completed,
    /// The deadline passed.
    DeadlineExpired,
    /// Failed.
    Failed,
    /// Withdrawn by the game.
    Withdrawn,
}

const CONTRACT_STATES: [(&str, ContractState); 10] = [
    ("Generated", ContractState::Generated),
    ("Offered", ContractState::Offered),
    ("OfferExpired", ContractState::OfferExpired),
    ("Declined", ContractState::Declined),
    ("Cancelled", ContractState::Cancelled),
    ("Active", ContractState::Active),
    ("Completed", ContractState::Completed),
    ("DeadlineExpired", ContractState::DeadlineExpired),
    ("Failed", ContractState::Failed),
    ("Withdrawn", ContractState::Withdrawn),
];

/// Which list of the `ContractSystem` scenario a contract was read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractList {
    /// A `CONTRACT` node of `CONTRACTS`, an offered or active contract.
    Current,
    /// A `CONTRACT_FINISHED` node of `CONTRACTS`, a contract that has ended. Observed on
    /// 2026-11-18 in a career save of game version 1.12.5, which held 13 `CONTRACT` and
    /// 94 `CONTRACT_FINISHED` nodes in a single `CONTRACTS` node.
    Finished,
}

/// The twelve numbers of a contract's `values` field.
///
/// The order is an *inference* from the game's modding documentation, consistent with
/// the magnitudes observed in a save of game version 1.12.5, and not verified against
/// the game's code. Times are in seconds, and dates are universal times with zero for a
/// date that has not occurred.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContractValues {
    /// How long the offer stands.
    pub time_expiry: Finite,
    /// How long the contract runs once accepted.
    pub time_deadline: Finite,
    /// Funds paid on acceptance.
    pub funds_advance: Finite,
    /// Funds paid on completion.
    pub funds_completion: Finite,
    /// Funds taken on failure.
    pub funds_failure: Finite,
    /// Reputation gained on completion.
    pub reputation_completion: Finite,
    /// Reputation lost on failure.
    pub reputation_failure: Finite,
    /// Science gained on completion.
    pub science_completion: Finite,
    /// When the offer expires.
    pub date_expire: Finite,
    /// When the contract was accepted.
    pub date_accepted: Finite,
    /// When the contract's deadline falls.
    pub date_deadline: Finite,
    /// When the contract ended.
    pub date_finished: Finite,
}

/// The number of fields in a contract's `values`.
const CONTRACT_VALUE_COUNT: usize = 12;

/// One contract.
#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    /// The contract's unique identifier, from `guid`.
    pub guid: String,
    /// The contract's class name, such as `ExplorationContract`, from `type`. It is an
    /// open set, since modifications add contract types.
    pub contract_type: String,
    /// The contract's state.
    pub state: ContractState,
    /// The list it was read from.
    pub list: ContractList,
    /// The offering agency, from `agent`, where present.
    pub agent: Option<String>,
    /// The rewards, penalties and dates, from `values`, where present. The rewards of
    /// the contract's parameters, in its `PARAM` nodes, are not included.
    pub values: Option<ContractValues>,
}

/// Reads the contracts of the `ContractSystem` scenario's `CONTRACTS` node, the current
/// ones from its `CONTRACT` children and the finished ones from its `CONTRACT_FINISHED`
/// children, in the order the save lists them. An absent `CONTRACTS` node is an empty
/// list, and a repeated one is read in full.
///
/// # Errors
///
/// A missing scenario, or a contract without a valid `guid`, `type` or `state`, or with
/// a `values` field that is not twelve finite numbers separated by commas.
pub fn contracts(game: &Node) -> Result<Vec<Contract>, ExtractError> {
    let system = scenario(game, "ContractSystem")?;
    let mut out = Vec::new();
    for holder in system.children_named("CONTRACTS") {
        for node in holder.children() {
            let list = match node.name() {
                "CONTRACT" => ContractList::Current,
                "CONTRACT_FINISHED" => ContractList::Finished,
                _ => continue,
            };
            out.push(contract(node, list)?);
        }
    }
    Ok(out)
}

fn contract(node: &Node, list: ContractList) -> Result<Contract, ExtractError> {
    Ok(Contract {
        guid: non_empty(node, "guid")?,
        contract_type: non_empty(node, "type")?,
        state: one_of(node, "state", &CONTRACT_STATES, "a contract state")?,
        list,
        agent: optional_text(node, "agent"),
        values: contract_values(node)?,
    })
}

fn contract_values(node: &Node) -> Result<Option<ContractValues>, ExtractError> {
    let Some(value) = node.value("values") else {
        return Ok(None);
    };
    // One field past the count is enough to refuse a list that is too long, so no more
    // are collected, however many commas the value holds.
    let fields: Vec<&str> = value
        .value()
        .split(',')
        .map(str::trim)
        .take(CONTRACT_VALUE_COUNT + 1)
        .collect();
    let [
        time_expiry,
        time_deadline,
        funds_advance,
        funds_completion,
        funds_failure,
        reputation_completion,
        reputation_failure,
        science_completion,
        date_expire,
        date_accepted,
        date_deadline,
        date_finished,
    ] = <[&str; CONTRACT_VALUE_COUNT]>::try_from(fields)
        .map_err(|_| invalid(node, "values", value, "twelve numbers separated by commas"))?;
    let number = |text: &str| -> Result<Finite, ExtractError> {
        text.parse::<f64>()
            .ok()
            .and_then(Finite::new)
            .ok_or_else(|| invalid(node, "values", value, "twelve finite numbers"))
    };
    Ok(Some(ContractValues {
        time_expiry: number(time_expiry)?,
        time_deadline: number(time_deadline)?,
        funds_advance: number(funds_advance)?,
        funds_completion: number(funds_completion)?,
        funds_failure: number(funds_failure)?,
        reputation_completion: number(reputation_completion)?,
        reputation_failure: number(reputation_failure)?,
        science_completion: number(science_completion)?,
        date_expire: number(date_expire)?,
        date_accepted: number(date_accepted)?,
        date_deadline: number(date_deadline)?,
        date_finished: number(date_finished)?,
    }))
}
