use cosmwasm_std::{
  Addr, DepsMut, Empty, Env, Event, MessageInfo, Response, StdError, StdResult, Storage
};
use cw_storage_plus::IntKey;
use crate::msgs::{InstantiateMsg, ExecuteMsg};
use crate::state::{Contract, StageInfo};

impl<'a> Contract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg
    ) -> StdResult<Response> {
        let events = msg.allow_list.iter().map(|addr| {
            self.add_allow_list_internal(deps.storage, addr.clone()).unwrap()
        });

        Ok(Response::new().add_events(events))
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg
    ) -> StdResult<Response> {
        match msg {
            ExecuteMsg::PrepareStage { stage }
                => self.prepare_stage(deps, env, info, stage),
            ExecuteMsg::FinalizeStage { stage }
                => self.finailze_stage(deps, env, info, stage),
            ExecuteMsg::IncreaseScore { addr, stage, amount }
                => self.increase_score(deps, env, info, addr, stage, amount),
            ExecuteMsg::DecreaseScore { addr, stage, amount }
                => self.decrease_score(deps, env, info, addr, stage, amount),
            ExecuteMsg::UpdateScore { addr, stage, amount }
                => self.update_score(deps, env, info, addr, stage, amount),
            ExecuteMsg::UpdateScores { stage, scores }
                => self.update_scores(deps, env, info, stage, scores),
            ExecuteMsg::AddAllowList { addr }
                => self.add_allow_list(deps, env, info, addr),
            ExecuteMsg::RemoveAllowList { addr }
                => self.remove_allow_list(deps, env, info, addr),
        }
    }
}

impl<'a> Contract<'a> {
    fn prepare_stage(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        stage: u64,
    ) -> StdResult<Response> {
        self.check_permission(deps.storage, &info.sender)?;

        let mut res = Response::new();

        let prepare_event = self.prepare_stage_internal(stage, deps.storage)?;
        if prepare_event.is_some() {
            res = res.add_event(prepare_event.unwrap());
        }

        Ok(res)
    }

    fn finailze_stage(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        stage: u64,
    ) -> StdResult<Response> {
        self.check_permission(deps.storage, &info.sender)?;

        self.stages.update(deps.storage, stage, |stage_info| -> StdResult<_> {
            match stage_info {
                Some(mut stage_info) => {
                    stage_info.is_finalized = true;
                    Ok(stage_info)
                },
                None => Err(StdError::generic_err("Stage not found"))
            }
        })?;

        Ok(Response::new().add_event(Event::new("finalize-stage")
        .add_attributes(vec![
            ("stage", &stage.to_string()),
        ]
    )))
    }

    fn increase_score(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        addr: Addr,
        stage: u64,
        amount: u64,
    ) -> StdResult<Response> {
        self.check_permission(deps.storage, &info.sender)?;
        let mut res = Response::new();

        let prepare_event = self.prepare_stage_internal(stage, deps.storage)?;
        if prepare_event.is_some() {
            res = res.add_event(prepare_event.unwrap());
        }


        if !self.stages.has(deps.storage, stage) {
            return Err(StdError::generic_err("Stage not found"));
        };

        if self.stages.load(deps.storage, stage).unwrap().is_finalized {
            return Err(StdError::generic_err("Stage finalized"));
        };

        let user_key = user_score_key(addr.clone(), stage);

        // update user score
        let new_score = self.scores.update(deps.storage, user_key, |score| -> StdResult<_> {
            match score {
                Some(score) => Ok(score + amount),
                None => Ok(amount)
            }
        });

        // update total score
        let stage_info = self.stages.update(deps.storage, stage, |stage_info| -> StdResult<_> {
            match stage_info {
                Some(mut stage_info) => {
                    stage_info.total_score = stage_info.total_score + amount;
                    Ok(stage_info)
                },
                None => Err(StdError::generic_err("Stage not found")) // can not reach
            }
        });

        let event = generate_update_score_event(&addr, stage, new_score.unwrap(), stage_info.unwrap().total_score);

        Ok(res.add_event(event))
    }

    fn decrease_score(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        addr: Addr,
        stage: u64,
        amount: u64,
    ) -> StdResult<Response> {
        self.check_permission(deps.storage, &info.sender)?;
        let mut res = Response::new();

        let prepare_event = self.prepare_stage_internal(stage, deps.storage)?;
        if prepare_event.is_some() {
            res = res.add_event(prepare_event.unwrap());
        }

        if !self.stages.has(deps.storage, stage) {
            return Err(StdError::generic_err("Stage not found"));
        };

        if self.stages.load(deps.storage, stage).unwrap().is_finalized {
            return Err(StdError::generic_err("Stage finalized"));
        };

        let user_key = user_score_key(addr.clone(), stage);

        // update user score
        let new_score = self.scores.update(deps.storage, user_key, |score| -> StdResult<_> {
            match score {
                Some(score) => {
                    if amount > score {
                        return Err(StdError::generic_err("Insufficient score"))
                    };
                    Ok(score - amount)
                },
                None => Err(StdError::generic_err("User score not found"))
            }
        });

        // update total score
        let stage_info: Result<StageInfo, StdError> = self.stages.update(deps.storage, stage, |stage_info| -> StdResult<_> {
            match stage_info {
                Some(mut stage_info) => {
                    if amount > stage_info.total_score {
                        return Err(StdError::generic_err("Insufficient score"))
                    };
                    stage_info.total_score = stage_info.total_score - amount;
                    Ok(stage_info)
                },
                None => Err(StdError::generic_err("Stage not found"))
            }
        });

        let event = generate_update_score_event(&addr, stage, new_score.unwrap(), stage_info.unwrap().total_score);
        Ok(res.add_event(event))
    }

    fn update_score(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        addr: Addr,
        stage: u64,
        amount: u64,
    ) -> StdResult<Response> {
        self.check_permission(deps.storage, &info.sender)?;
        let mut res = Response::new();

        let prepare_event = self.prepare_stage_internal(stage, deps.storage)?;
        if prepare_event.is_some() {
            res = res.add_event(prepare_event.unwrap());
        }

        let event = self.update_score_internal(deps.storage, stage, addr, amount)?;
        Ok(res.add_event(event))
    }

    fn update_scores(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        stage: u64,
        scores: Vec<(Addr, u64)>,
    ) -> StdResult<Response> {
        self.check_permission(deps.storage, &info.sender)?;
        let mut res = Response::new();

        let prepare_event = self.prepare_stage_internal(stage, deps.storage)?;
        if prepare_event.is_some() {
            res = res.add_event(prepare_event.unwrap());
        }

        let scores_events = scores.iter().map(|(addr, amount)| {
            self.update_score_internal(deps.storage, stage, addr.clone(), *amount).unwrap()
        });

        Ok(res.add_events(scores_events))
    }

    fn add_allow_list(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        addr: Addr,
    ) -> StdResult<Response> {
        self.check_permission(deps.storage, &info.sender)?;
        let event = self.add_allow_list_internal(deps.storage, addr)?;
        Ok(Response::new().add_event(event))
    }

    fn remove_allow_list(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        addr: Addr,
    ) -> StdResult<Response> {
        self.check_permission(deps.storage, &info.sender)?;
        self.allow_list.remove(deps.storage, addr.clone());
        Ok(Response::new().add_event(Event::new("remove-allow-list").add_attributes(vec![
            ("addr", &addr.to_string()),
        ])))
    }

    fn add_allow_list_internal(
        &self,
        storage: &mut dyn Storage,
        addr: Addr,
    ) -> StdResult<Event> { 
        self.allow_list.save(storage, addr.clone(), &Empty {  })?;

        Ok(Event::new("add-allow-list").add_attributes(vec![
            ("addr", &addr.to_string()),
        ]))
    }

    fn update_score_internal(
        &self,
        storage: &mut dyn Storage,
        stage: u64,
        addr: Addr,
        amount: u64,
    ) -> StdResult<Event> {
        if !self.stages.has(storage, stage) {
            return Err(StdError::generic_err("Stage not found"));
        };

        if self.stages.load(storage, stage).unwrap().is_finalized {
            return Err(StdError::generic_err("Stage finalized"));
        };


        let user_key = user_score_key(addr.clone(), stage);

        let mut score_diff: i128 = amount as i128;

        // update user score
        let new_score = self.scores.update(storage, user_key, |score| -> StdResult<_> {
            match score {
                Some(score) => {
                    score_diff = (amount as i128) - (score as i128)
                },
                None => {},   
            }
            Ok(amount)
        });

        // update total score
        let stage_info = self.stages.update(storage, stage, |stage_info| -> StdResult<_> {
            match stage_info {
                Some(mut stage_info) => {
                    stage_info.total_score = ((stage_info.total_score as i128) + score_diff) as u64;
                    Ok(stage_info)
                },
                None => Err(StdError::generic_err("Stage not found"))
            }
        });

        Ok(generate_update_score_event(&addr, stage, new_score.unwrap(), stage_info.unwrap().total_score))
    }
}

//helper
impl<'a> Contract<'a> {
    pub fn check_permission(
        &self,
        store: &dyn Storage,
        account_addr: &Addr,
    ) -> StdResult<()> {
        let is_allowed = self.allow_list.has(store, account_addr.clone());
        if !is_allowed {
            return Err(StdError::generic_err("Address is not allowed"));
        }
        
        Ok(())
    }

    pub fn prepare_stage_internal(
        &self,
        stage: u64,
        storage: &mut dyn Storage,
    ) -> StdResult<Option<Event>> {
        if stage == 0 {
            return Err(StdError::generic_err("Stage can not be zero"))
        };

        if self.stages.has(storage, stage) {
            return Ok(Option::None);
        };

        self.stages.save(storage, stage, &StageInfo { stage, total_score: 0, is_finalized: false })?;

        Ok(Some(Event::new("prepare-stage")
            .add_attributes(vec![
                ("stage", &stage.to_string()),
            ])
        ))
    }
}

pub fn user_score_key(user_addr: Addr, stage: u64) -> Vec<u8> {
    [stage.to_cw_bytes().to_vec(), user_addr.as_bytes().to_vec()].concat()
}

pub fn generate_update_score_event(addr: &Addr, stage: u64, new_score: u64, total_score: u64) -> Event {
    Event::new("update-score").add_attributes(vec![
        ("addr", &addr.to_string()),
        ("stage", &stage.to_string()),
        ("score", &new_score.to_string()),
        ("total_score", &total_score.to_string()),
    ])
}