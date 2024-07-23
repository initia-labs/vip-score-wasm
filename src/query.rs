use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, Order, StdResult};
use cw_storage_plus::{IntKey, KeyDeserialize};

use crate::execute::user_score_key;
use crate::state::{Contract, StageInfo};
use crate::msgs::{QueryMsg, UserScore, UserScores};

impl<'a> Contract<'a> {
    fn get_score(&self, deps: Deps, _env: Env, addr: Addr, stage: u64) -> StdResult<UserScore> {
        let user_key = user_score_key(addr.clone(), stage);
        let score = self.scores.load(deps.storage, user_key)?;
        Ok(UserScore { score, stage, addr })
    }

    fn get_scores(&self, deps: Deps, _env: Env, stage: u64, limit: u64, start_after: Option<Addr>) -> StdResult<UserScores> {
        let min = if start_after.is_some() {
            Some(cw_storage_plus::Bound::ExclusiveRaw(user_score_key(start_after.unwrap(), stage)))
        } else {
            Some(cw_storage_plus::Bound::ExclusiveRaw(stage.to_cw_bytes().to_vec()))
        };
        let max = Some(cw_storage_plus::Bound::ExclusiveRaw((stage + 1).to_cw_bytes().to_vec()));
        let mut iter = self.scores.range(deps.storage, min, max, Order::Ascending).into_iter();
        let mut scores = vec![];
        let mut score_res = iter.next();
        while scores.len() < (limit as usize) && score_res.is_some() {
            let (key, score) = score_res.unwrap().unwrap();
            let mut stage_bytes:[u8; 8] = [0; 8];
            stage_bytes.copy_from_slice(&key[0..8]);
            let stage: u64 = IntKey::from_cw_bytes(stage_bytes);

            let addr = Addr::from_vec(key[8..].to_vec())?;
            scores.push(UserScore { score, stage, addr });
            score_res = iter.next();
        };
        Ok(UserScores { scores })
    }

    fn get_stage_info(&self, deps: Deps, _env: Env, stage: u64) -> StdResult<StageInfo> {
        let stage_info = self.stages.load(deps.storage, stage)?;
        Ok(stage_info)
    }
}


impl<'a> Contract<'a> {
pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetScore { addr, stage } => to_json_binary(&self.get_score(deps, env, addr, stage)?),
            QueryMsg::GetScores { stage, limit, start_after }
                => to_json_binary(&self.get_scores(deps, env, stage, limit, start_after)?),
            QueryMsg::GetStageInfo { stage } => to_json_binary(&self.get_stage_info(deps, env, stage)?)
        }
    }
}
