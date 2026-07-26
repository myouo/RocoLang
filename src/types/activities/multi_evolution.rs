use serde::{Deserialize, Serialize};

use super::super::{RocoRequestContext, RocoRewardKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionCandidate {
    pub candidate_index: i64,
    pub spirit_id: i64,
    pub catch_time: i64,
    pub level: i64,
    pub stats: MultiEvolutionCandidateStats,
    pub innate_stats: MultiEvolutionCandidateStats,
    pub postnatal_stats: MultiEvolutionCandidateStats,
    pub condition_code: i64,
    pub condition_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionCandidateStats {
    pub full_hp: i64,
    pub near_attack: i64,
    pub near_armor: i64,
    pub far_attack: i64,
    pub far_armor: i64,
    pub speed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionRewardItem {
    pub reward_id: i64,
    pub reward_kind: RocoRewardKind,
    pub raw_reward_type: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionCandidatesInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub candidates: Vec<MultiEvolutionCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionElementEvolveResult {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub pet_id: i64,
    pub evolution_result: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionGrassStageResult {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionGrassEvolveResult {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub pet_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionBoosterItemInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub item_id: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionRewardsInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub rewards: Vec<MultiEvolutionRewardItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEvolutionRewardAvailabilityInfo {
    pub result_code: i64,
    pub message: String,
    pub request_context: RocoRequestContext,
    pub available: bool,
}
