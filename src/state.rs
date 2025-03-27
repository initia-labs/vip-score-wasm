use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr, Empty};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub struct Contract<'a> {
  pub init_stage: Item<'a, u64>,
  pub scores: Map<'a, Vec<u8>, u64>, // key (stage::addr)
  pub stages: Map<'a, u64, StageInfo>,
  pub allow_list: Map<'a, Addr, Empty>, // user or contract address
}

impl Default for Contract<'static> {
  fn default() -> Self {
    Self::new(
      "init_stage",
      "scores",
      "stages",
      "allow_list",
    )
  }
}

impl<'a> Contract<'a> {
  fn new(
    initi_stage_key: &'a str,
    scores_key: &'a str,
    stages_key: &'a str,
    allow_list_key: &'a str,
  ) -> Self {
    Self {
      init_stage: Item::<'a, u64>::new(initi_stage_key),
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