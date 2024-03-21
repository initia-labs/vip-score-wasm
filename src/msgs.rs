use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InstantiateMsg {
  pub allow_list: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  PrepareStage {
    stage: u64,
  },
  FinalizeStage {
    stage: u64,
  },
  IncreaseScore {
    addr: Addr,
    stage: u64,
    amount: u64,
  },
  DecreaseScore {
    addr: Addr,
    stage: u64,
    amount: u64,
  },
  UpdateScore {
    addr: Addr,
    stage: u64,
    amount: u64,
  },
  UpdateScores {
    stage: u64,
    scores: Vec<(Addr, u64)>,
  },
  AddAllowList {
    addr: Addr,
  },
  RemoveAllowList {
    addr: Addr,
  },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  GetScore {
    addr: Addr,
    stage: u64,
  },
  GetScores {
    stage: u64,
    limit: u8,
    start_after: Option<Addr>
  },
  GetStageInfo {
    stage: u64,
  },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct UserScore {
  pub stage: u64,
  pub addr: Addr,
  pub score: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct UserScores {
  pub scores: Vec<UserScore>,
}
