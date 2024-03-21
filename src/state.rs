use cw_storage_plus::Map;
use cosmwasm_std::{Addr, Empty};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub struct Contract<'a> {
  pub scores: Map<'a, Vec<u8>, u64>, // key (stage::addr)
  pub stages: Map<'a, u64, StageInfo>,
  pub allow_list: Map<'a, Addr, Empty>, // user or contract address
}

impl Default for Contract<'static> {
  fn default() -> Self {
    Self::new(
      "scores",
      "stages",
      "allow_list",
    )
  }
}

impl<'a> Contract<'a> {
  fn new(
    scores_key: &'a str,
    stages_key: &'a str,
    allow_list_key: &'a str,
  ) -> Self {
    Self {
      scores: Map::<'a, Vec<u8>, u64>::new(scores_key),
      stages: Map::<'a, u64, StageInfo>::new(stages_key),
      allow_list: Map::<'a, Addr, Empty>::new(allow_list_key), 
    }
  }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct StageInfo {
  pub stage: u64,
  pub total_score: u64,
  pub is_finalized: bool,
}