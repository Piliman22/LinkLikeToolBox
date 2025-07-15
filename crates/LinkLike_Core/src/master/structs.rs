use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvAlbums {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "AdvSeriesId")]
    pub adv_series_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "ScriptId")]
    pub script_id: i32,
    #[serde(rename = "OpenSeasonFanLevel")]
    pub open_season_fan_level: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: String,
    #[serde(rename = "WatchRewardId")]
    pub watch_reward_id: String,
    #[serde(rename = "WatchRewardNum")]
    pub watch_reward_num: String,
    #[serde(rename = "RewardTextId")]
    pub reward_text_id: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "SubTitleName")]
    pub sub_title_name: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "IsPeriod")]
    pub is_period: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "SeasonsId")]
    pub seasons_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "FiscalYearDisplay")]
    pub fiscal_year_display: i32,
    #[serde(rename = "AdvAlbumId")]
    pub adv_album_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvStoryDigestMovies {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "DigestFiscalYearDisplay")]
    pub digest_fiscal_year_display: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeginnerMissionBannerRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MissionType")]
    pub mission_type: i32,
    #[serde(rename = "ItemType")]
    pub item_type: i32,
    #[serde(rename = "ItemId")]
    pub item_id: i32,
    #[serde(rename = "Num")]
    pub num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeginnerMissionsHintImages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "BeginnerMissionsHintId")]
    pub beginner_missions_hint_id: i32,
    #[serde(rename = "ImageId")]
    pub image_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeginnerMissionsHint {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthdayRareBonuses {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CardSeriesId")]
    pub card_series_id: i32,
    #[serde(rename = "SkillName")]
    pub skill_name: String,
    #[serde(rename = "LimitBreakTimes")]
    pub limit_break_times: i32,
    #[serde(rename = "MentalBonus")]
    pub mental_bonus: i32,
    #[serde(rename = "VoltageBonus")]
    pub voltage_bonus: i32,
    #[serde(rename = "HeartBonus")]
    pub heart_bonus: i32,
    #[serde(rename = "LoveBonus")]
    pub love_bonus: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignAddRewardSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "AddRewardOddsId_1")]
    pub add_reward_odds_id_1: i64,
    #[serde(rename = "AddRewardOddsId_2")]
    pub add_reward_odds_id_2: i64,
    #[serde(rename = "AddRewardOddsId_3")]
    pub add_reward_odds_id_3: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignAddRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "AddRewardOddsId")]
    pub add_reward_odds_id: i64,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "AddRewardItemId")]
    pub add_reward_item_id: i32,
    #[serde(rename = "AddRewardItemQuantity")]
    pub add_reward_item_quantity: i32,
    #[serde(rename = "AddRewardItemOdds")]
    pub add_reward_item_odds: i32,
    #[serde(rename = "AddRewardItemOddsSum")]
    pub add_reward_item_odds_sum: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CampaignType")]
    pub campaign_type: i32,
    #[serde(rename = "TargetContents")]
    pub target_contents: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "EffectValue")]
    pub effect_value: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCoordinates {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "CardDatasId")]
    pub card_datas_id: i32,
    #[serde(rename = "CardCoordSceneType")]
    pub card_coord_scene_type: i32,
    #[serde(rename = "XCoord")]
    pub x_coord: i32,
    #[serde(rename = "YCoord")]
    pub y_coord: i32,
    #[serde(rename = "Scale")]
    pub scale: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CardSeriesId")]
    pub card_series_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
    #[serde(rename = "Rarity")]
    pub rarity: i32,
    #[serde(rename = "EvolveTimes")]
    pub evolve_times: i32,
    #[serde(rename = "CardLevelLimitAddition")]
    pub card_level_limit_addition: i32,
    #[serde(rename = "Style")]
    pub style: i32,
    #[serde(rename = "Mood")]
    pub mood: i32,
    #[serde(rename = "ExperienceType")]
    pub experience_type: i32,
    #[serde(rename = "InitialSmile")]
    pub initial_smile: i32,
    #[serde(rename = "InitialPure")]
    pub initial_pure: i32,
    #[serde(rename = "InitialCool")]
    pub initial_cool: i32,
    #[serde(rename = "InitialMental")]
    pub initial_mental: i32,
    #[serde(rename = "MaxSmile")]
    pub max_smile: i32,
    #[serde(rename = "MaxPure")]
    pub max_pure: i32,
    #[serde(rename = "MaxCool")]
    pub max_cool: i32,
    #[serde(rename = "MaxMental")]
    pub max_mental: i32,
    #[serde(rename = "BeatPoint")]
    pub beat_point: i32,
    #[serde(rename = "SpecialAppealSeriesId")]
    pub special_appeal_series_id: i32,
    #[serde(rename = "SkillSeriesId")]
    pub skill_series_id: i32,
    #[serde(rename = "AttributeId")]
    pub attribute_id: i32,
    #[serde(rename = "SpineId")]
    pub spine_id: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "CenterSkillSeriesId")]
    pub center_skill_series_id: i32,
    #[serde(rename = "CenterAttributeSeriesId")]
    pub center_attribute_series_id: i32,
    #[serde(rename = "RhythmGameSkillSeriesId")]
    pub rhythm_game_skill_series_id: i32,
    #[serde(rename = "CenterSkillLvUpItemId")]
    pub center_skill_lv_up_item_id: i32,
    #[serde(rename = "RhythmGameSkillLvUpItemId")]
    pub rhythm_game_skill_lv_up_item_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDuetVoice {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CardSeriesId")]
    pub card_series_id: i32,
    #[serde(rename = "CharacterIds")]
    pub character_ids: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardEvolutionMaterials {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CostItemsId1")]
    pub cost_items_id1: i32,
    #[serde(rename = "CostNum1")]
    pub cost_num1: i32,
    #[serde(rename = "CostItemsId2")]
    pub cost_items_id2: i32,
    #[serde(rename = "CostNum2")]
    pub cost_num2: i32,
    #[serde(rename = "CostItemsId3")]
    pub cost_items_id3: i32,
    #[serde(rename = "CostNum3")]
    pub cost_num3: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardGetMovieSettings {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CardInfoPositionType")]
    pub card_info_position_type: i32,
    #[serde(rename = "CardInfoDisplayStartTimeSeconds")]
    pub card_info_display_start_time_seconds: i32,
    #[serde(rename = "UrCardEffectBackgroundId")]
    pub ur_card_effect_background_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardLevels {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ExperienceType")]
    pub experience_type: i32,
    #[serde(rename = "CardLevel")]
    pub card_level: i32,
    #[serde(rename = "Experience")]
    pub experience: i32,
    #[serde(rename = "CumulativeExperience")]
    pub cumulative_experience: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardLimitBreakMaterials {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CardSeriesId")]
    pub card_series_id: i32,
    #[serde(rename = "LimitBreakTimes")]
    pub limit_break_times: i32,
    #[serde(rename = "CostItemsId")]
    pub cost_items_id: i32,
    #[serde(rename = "CostNum")]
    pub cost_num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardRarities {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RarityName")]
    pub rarity_name: String,
    #[serde(rename = "Evolution0_MaxLevel")]
    pub evolution0__max_level: i32,
    #[serde(rename = "Evolution1_MaxLevel")]
    pub evolution1__max_level: i32,
    #[serde(rename = "Evolution2_MaxLevel")]
    pub evolution2__max_level: i32,
    #[serde(rename = "Evolution3_MaxLevel")]
    pub evolution3__max_level: i32,
    #[serde(rename = "Evolution4_MaxLevel")]
    pub evolution4__max_level: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Evolution0Id")]
    pub evolution0_id: i32,
    #[serde(rename = "Evolution1Id")]
    pub evolution1_id: i32,
    #[serde(rename = "Evolution2Id")]
    pub evolution2_id: i32,
    #[serde(rename = "ObtainFanLvPt")]
    pub obtain_fan_lv_pt: i32,
    #[serde(rename = "Evolution1FanLvPt")]
    pub evolution1_fan_lv_pt: i32,
    #[serde(rename = "Evolution2FanLvPt")]
    pub evolution2_fan_lv_pt: i32,
    #[serde(rename = "LimitBreak1FanLvPt")]
    pub limit_break1_fan_lv_pt: i32,
    #[serde(rename = "LimitBreak2FanLvPt")]
    pub limit_break2_fan_lv_pt: i32,
    #[serde(rename = "LimitBreak3FanLvPt")]
    pub limit_break3_fan_lv_pt: i32,
    #[serde(rename = "LimitBreak4FanLvPt")]
    pub limit_break4_fan_lv_pt: i32,
    #[serde(rename = "LimitedType")]
    pub limited_type: i32,
    #[serde(rename = "SideStyleSettingCharacterId")]
    pub side_style_setting_character_id: String,
    #[serde(rename = "Evolution3Id")]
    pub evolution3_id: i32,
    #[serde(rename = "Evolution4Id")]
    pub evolution4_id: i32,
    #[serde(rename = "Evolution3FanLvPt")]
    pub evolution3_fan_lv_pt: i32,
    #[serde(rename = "Evolution4FanLvPt")]
    pub evolution4_fan_lv_pt: i32,
    #[serde(rename = "Evolution3RequiredFanLv")]
    pub evolution3_required_fan_lv: i32,
    #[serde(rename = "Evolution4RequiredFanLv")]
    pub evolution4_required_fan_lv: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSkillEffectDetailParams {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ParamType")]
    pub param_type: String,
    #[serde(rename = "ParamValue")]
    pub param_value: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSkillEffectDetails {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "SkillEffectDetailType")]
    pub skill_effect_detail_type: String,
    #[serde(rename = "TargetMood")]
    pub target_mood: i32,
    #[serde(rename = "EffectValue")]
    pub effect_value: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSkillEffects {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ActionType")]
    pub action_type: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSkillLevelUpMaterials {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "CardSeriesId")]
    pub card_series_id: i32,
    #[serde(rename = "SkillType")]
    pub skill_type: i32,
    #[serde(rename = "SkillLevel")]
    pub skill_level: i32,
    #[serde(rename = "Cost_ItemsId1")]
    pub cost__items_id1: i32,
    #[serde(rename = "CostNum1")]
    pub cost_num1: i32,
    #[serde(rename = "Cost_ItemsId2")]
    pub cost__items_id2: i32,
    #[serde(rename = "CostNum2")]
    pub cost_num2: i32,
    #[serde(rename = "Cost_ItemsId3")]
    pub cost__items_id3: i32,
    #[serde(rename = "CostNum3")]
    pub cost_num3: i32,
    #[serde(rename = "Cost_ItemsIds")]
    pub cost__items_ids: String,
    #[serde(rename = "CostNums")]
    pub cost_nums: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSkillModes {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ModeOffName")]
    pub mode_off_name: String,
    #[serde(rename = "ModeOnName")]
    pub mode_on_name: String,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSkillSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "SkillIcon")]
    pub skill_icon: i32,
    #[serde(rename = "SkillMainEffect")]
    pub skill_main_effect: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSkills {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "CardSkillSeriesId")]
    pub card_skill_series_id: i32,
    #[serde(rename = "SkillLevel")]
    pub skill_level: i32,
    #[serde(rename = "SkillCost")]
    pub skill_cost: i32,
    #[serde(rename = "ApperanceType")]
    pub apperance_type: i32,
    #[serde(rename = "CardSkillEffectId")]
    pub card_skill_effect_id: String,
    #[serde(rename = "Description")]
    pub description: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterAttributeEffects {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CenterAttributeEffectType")]
    pub center_attribute_effect_type: i32,
    #[serde(rename = "CenterAttributeEffectValue")]
    pub center_attribute_effect_value: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterAttributes {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "CenterAttributeSeriesId")]
    pub center_attribute_series_id: i32,
    #[serde(rename = "CenterAttributeName")]
    pub center_attribute_name: String,
    #[serde(rename = "TargetIds")]
    pub target_ids: String,
    #[serde(rename = "CenterAttributeEffectId")]
    pub center_attribute_effect_id: i32,
    #[serde(rename = "Description")]
    pub description: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterSkillConditions {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CenterSkillConditionType")]
    pub center_skill_condition_type: i32,
    #[serde(rename = "CenterSkillConditionValue")]
    pub center_skill_condition_value: i32,
    #[serde(rename = "CenterSkillConditionValue2")]
    pub center_skill_condition_value2: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterSkillEffects {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CenterSkillEffectType")]
    pub center_skill_effect_type: i32,
    #[serde(rename = "CenterSkillEffectValue")]
    pub center_skill_effect_value: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterSkills {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "CenterSkillSeriesId")]
    pub center_skill_series_id: i32,
    #[serde(rename = "CenterSkillName")]
    pub center_skill_name: String,
    #[serde(rename = "SkillLevel")]
    pub skill_level: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "CenterSkillConditionIds")]
    pub center_skill_condition_ids: String,
    #[serde(rename = "CenterSkillEffectId")]
    pub center_skill_effect_id: i32,
    #[serde(rename = "Description")]
    pub description: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeModeEffectDetails {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "EffectDetailType")]
    pub effect_detail_type: String,
    #[serde(rename = "TargetMood")]
    pub target_mood: i32,
    #[serde(rename = "EffectValue")]
    pub effect_value: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeModeEffects {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "StandardQuestStagesId")]
    pub standard_quest_stages_id: i32,
    #[serde(rename = "ActionType")]
    pub action_type: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "Description")]
    pub description: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeModeReleaseCondition {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ReleaseChallengeModeId")]
    pub release_challenge_mode_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeModeStages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ChallengeModeAreasId")]
    pub challenge_mode_areas_id: i32,
    #[serde(rename = "CorrespondedQuestStageId")]
    pub corresponded_quest_stage_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Hint")]
    pub hint: String,
    #[serde(rename = "MapNumber")]
    pub map_number: i32,
    #[serde(rename = "StageType")]
    pub stage_type: i32,
    #[serde(rename = "UseType")]
    pub use_type: i32,
    #[serde(rename = "UseItem")]
    pub use_item: i32,
    #[serde(rename = "UseNum")]
    pub use_num: i32,
    #[serde(rename = "LiveStagesId")]
    pub live_stages_id: i32,
    #[serde(rename = "QuestMusicsType")]
    pub quest_musics_type: i32,
    #[serde(rename = "QuestMusicsDetail")]
    pub quest_musics_detail: i32,
    #[serde(rename = "DeckRestrictedType")]
    pub deck_restricted_type: i32,
    #[serde(rename = "DeckRestrictedDetail")]
    pub deck_restricted_detail: i32,
    #[serde(rename = "ChallengeModeEffectId")]
    pub challenge_mode_effect_id: i64,
    #[serde(rename = "QuestLevel")]
    pub quest_level: i32,
    #[serde(rename = "FirstClearRewardSeriesId")]
    pub first_clear_reward_series_id: i64,
    #[serde(rename = "ChallengeModeScore")]
    pub challenge_mode_score: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterFavoriteGifts {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ItemsId")]
    pub items_id: i32,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
    #[serde(rename = "FavoriteRank")]
    pub favorite_rank: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Characters {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "NameLast")]
    pub name_last: String,
    #[serde(rename = "NameFirst")]
    pub name_first: String,
    #[serde(rename = "LatinAlphabetNameLast")]
    pub latin_alphabet_name_last: String,
    #[serde(rename = "LatinAlphabetNameFirst")]
    pub latin_alphabet_name_first: String,
    #[serde(rename = "GenerationsId")]
    pub generations_id: i32,
    #[serde(rename = "SeriesType")]
    pub series_type: i32,
    #[serde(rename = "IconOrderId")]
    pub icon_order_id: i32,
    #[serde(rename = "CharacterVoice")]
    pub character_voice: String,
    #[serde(rename = "ThemeColor")]
    pub theme_color: String,
    #[serde(rename = "Introduction")]
    pub introduction: String,
    #[serde(rename = "ShowSeasonFanLvStartTime")]
    pub show_season_fan_lv_start_time: DateTime<Utc>,
    #[serde(rename = "ShowSeasonFanLvEndTime")]
    pub show_season_fan_lv_end_time: DateTime<Utc>,
    #[serde(rename = "IsExistFanLv")]
    pub is_exist_fan_lv: i32,
    #[serde(rename = "StyleType")]
    pub style_type: i32,
    #[serde(rename = "PrintFilterType")]
    pub print_filter_type: i32,
    #[serde(rename = "DisplayFullName")]
    pub display_full_name: String,
    #[serde(rename = "LatinAlpabetFullName")]
    pub latin_alpabet_full_name: String,
    #[serde(rename = "NameDisplayType")]
    pub name_display_type: i32,
    #[serde(rename = "DisplayGeneration")]
    pub display_generation: String,
    #[serde(rename = "GraduateIntroduction")]
    pub graduate_introduction: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comics {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ViewType")]
    pub view_type: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "TabListId")]
    pub tab_list_id: i32,
    #[serde(rename = "AppearanceCharacterIds")]
    pub appearance_character_ids: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonMissions {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ClearConditionNum")]
    pub clear_condition_num: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "ItemsId")]
    pub items_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentGuidances {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ContentId")]
    pub content_id: i32,
    #[serde(rename = "Priority")]
    pub priority: i32,
    #[serde(rename = "IsSkip")]
    pub is_skip: i32,
    #[serde(rename = "IsEnable")]
    pub is_enable: i32,
    #[serde(rename = "ConditionValues")]
    pub condition_values: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentsReleaseConditions {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ReleaseContentsId")]
    pub release_contents_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostumeModels {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
    #[serde(rename = "CostumesId")]
    pub costumes_id: i32,
    #[serde(rename = "HairStyleId")]
    pub hair_style_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Costumes {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Label")]
    pub label: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomComplementMaterials {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLiveReleaseConditions {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ReleaseDailyLivesId")]
    pub release_daily_lives_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyQuestSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "IsSunday")]
    pub is_sunday: i32,
    #[serde(rename = "IsMonday")]
    pub is_monday: i32,
    #[serde(rename = "IsTuesday")]
    pub is_tuesday: i32,
    #[serde(rename = "IsWednesday")]
    pub is_wednesday: i32,
    #[serde(rename = "IsThursday")]
    pub is_thursday: i32,
    #[serde(rename = "IsFriday")]
    pub is_friday: i32,
    #[serde(rename = "IsSaturday")]
    pub is_saturday: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyQuestStages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "DailyQuestSeriesId")]
    pub daily_quest_series_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Hint")]
    pub hint: String,
    #[serde(rename = "StageType")]
    pub stage_type: i32,
    #[serde(rename = "UseType")]
    pub use_type: i32,
    #[serde(rename = "UseItem")]
    pub use_item: i32,
    #[serde(rename = "UseNum")]
    pub use_num: i32,
    #[serde(rename = "LiveStagesId")]
    pub live_stages_id: i32,
    #[serde(rename = "QuestMusicsType")]
    pub quest_musics_type: i32,
    #[serde(rename = "QuestMusicsDetail")]
    pub quest_musics_detail: i32,
    #[serde(rename = "DeckRestrictedType")]
    pub deck_restricted_type: i32,
    #[serde(rename = "DeckRestrictedDetail")]
    pub deck_restricted_detail: i32,
    #[serde(rename = "QuestLevel")]
    pub quest_level: i32,
    #[serde(rename = "QuestRank")]
    pub quest_rank: i32,
    #[serde(rename = "FirstClearRewardSeriesId")]
    pub first_clear_reward_series_id: i64,
    #[serde(rename = "CompleteRewardSeriesId")]
    pub complete_reward_series_id: i64,
    #[serde(rename = "DropRewardSeriesId")]
    pub drop_reward_series_id: i64,
    #[serde(rename = "RandomDropRewardSeriesId")]
    pub random_drop_reward_series_id: i64,
    #[serde(rename = "Score1")]
    pub score1: i64,
    #[serde(rename = "Score2")]
    pub score2: i64,
    #[serde(rename = "Score3")]
    pub score3: i64,
    #[serde(rename = "GainStylePoint")]
    pub gain_style_point: i32,
    #[serde(rename = "GainMusicExp")]
    pub gain_music_exp: i32,
    #[serde(rename = "ItemSourceIconId")]
    pub item_source_icon_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckMemberPositions {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GenerationsId")]
    pub generations_id: i32,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyBgImages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MusicDifficulty")]
    pub music_difficulty: i32,
    #[serde(rename = "OutGameBgImageId")]
    pub out_game_bg_image_id: i32,
    #[serde(rename = "InGameBgImageId")]
    pub in_game_bg_image_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadImages {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "DownloadType")]
    pub download_type: i32,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamLiveReleaseConditions {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ReleaseDreamLiveId")]
    pub release_dream_live_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamLiveSeriesList {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "DisplayPosition")]
    pub display_position: i32,
    #[serde(rename = "ImageId")]
    pub image_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamQuestSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "DreamLiveSeriesListId")]
    pub dream_live_series_list_id: i32,
    #[serde(rename = "ReleaseStartTime")]
    pub release_start_time: DateTime<Utc>,
    #[serde(rename = "MessageTextId")]
    pub message_text_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamQuestStages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "DreamQuestSeriesId")]
    pub dream_quest_series_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Hint")]
    pub hint: String,
    #[serde(rename = "StageType")]
    pub stage_type: i32,
    #[serde(rename = "UseType")]
    pub use_type: i32,
    #[serde(rename = "UseItem")]
    pub use_item: i32,
    #[serde(rename = "UseNum")]
    pub use_num: i32,
    #[serde(rename = "LiveStagesId")]
    pub live_stages_id: i32,
    #[serde(rename = "QuestMusicsType")]
    pub quest_musics_type: i32,
    #[serde(rename = "QuestMusicsDetail")]
    pub quest_musics_detail: i32,
    #[serde(rename = "DeckRestrictedType")]
    pub deck_restricted_type: i32,
    #[serde(rename = "DeckRestrictedDetail")]
    pub deck_restricted_detail: i32,
    #[serde(rename = "FirstClearRewardSeriesId")]
    pub first_clear_reward_series_id: i64,
    #[serde(rename = "DropRewardSeriesId")]
    pub drop_reward_series_id: i64,
    #[serde(rename = "RandomDropRewardSeriesId")]
    pub random_drop_reward_series_id: i64,
    #[serde(rename = "Score1")]
    pub score1: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiCategory {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emojis {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Alias")]
    pub alias: String,
    #[serde(rename = "Category")]
    pub category: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "RequirementType")]
    pub requirement_type: i32,
    #[serde(rename = "RequirementDetail")]
    pub requirement_detail: String,
    #[serde(rename = "RequirementValue")]
    pub requirement_value: i32,
    #[serde(rename = "RequirementText")]
    pub requirement_text: String,
    #[serde(rename = "IsVisibleOnlyPossess")]
    pub is_visible_only_possess: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "AvailableStartTime")]
    pub available_start_time: DateTime<Utc>,
    #[serde(rename = "AvailableEndTime")]
    pub available_end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLoginBonuses {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "LoginBonusRewardSeriesId")]
    pub login_bonus_reward_series_id: i32,
    #[serde(rename = "EventLoginBonusType")]
    pub event_login_bonus_type: i32,
    #[serde(rename = "LoginBonusTextId")]
    pub login_bonus_text_id: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMissionAchieveRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "EvemtMissionSeriesId")]
    pub evemt_mission_series_id: i32,
    #[serde(rename = "AchieveMarkNum")]
    pub achieve_mark_num: i32,
    #[serde(rename = "RewardCategory")]
    pub reward_category: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "ItemsId")]
    pub items_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "RewardTextId")]
    pub reward_text_id: i32,
    #[serde(rename = "SortOrder")]
    pub sort_order: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMissionRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "EventMissionSeriesId")]
    pub event_mission_series_id: i32,
    #[serde(rename = "EventMissionsId")]
    pub event_missions_id: i32,
    #[serde(rename = "RewardCategory")]
    pub reward_category: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "ItemsId")]
    pub items_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "RewardTextId")]
    pub reward_text_id: i32,
    #[serde(rename = "SortOrder")]
    pub sort_order: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMissionSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "GrandPrixesId")]
    pub grand_prixes_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMissions {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "EventMissionSeriesId")]
    pub event_mission_series_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "MissionType")]
    pub mission_type: i32,
    #[serde(rename = "MissionCondition")]
    pub mission_condition: i32,
    #[serde(rename = "MissionConditionNum")]
    pub mission_condition_num: i32,
    #[serde(rename = "OpenType")]
    pub open_type: i32,
    #[serde(rename = "NextMissionsId")]
    pub next_missions_id: i32,
    #[serde(rename = "SortOrder")]
    pub sort_order: i32,
    #[serde(rename = "TransitionContentsId")]
    pub transition_contents_id: i32,
    #[serde(rename = "MissionConditionDetail")]
    pub mission_condition_detail: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangePointConvert {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ConvertItemType")]
    pub convert_item_type: i32,
    #[serde(rename = "ConvertItemId")]
    pub convert_item_id: i32,
    #[serde(rename = "ConvertItemQuantity")]
    pub convert_item_quantity: i32,
    #[serde(rename = "ConvertTime")]
    pub convert_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangePointRate {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ExchangePointId")]
    pub exchange_point_id: i32,
    #[serde(rename = "ExchangeItemType")]
    pub exchange_item_type: i32,
    #[serde(rename = "ExchangeItemId")]
    pub exchange_item_id: i32,
    #[serde(rename = "ExchangeItemQuantity")]
    pub exchange_item_quantity: i32,
    #[serde(rename = "ExchangePrice")]
    pub exchange_price: i32,
    #[serde(rename = "LimitedCount")]
    pub limited_count: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "GachaSeriesId")]
    pub gacha_series_id: i32,
    #[serde(rename = "BonusItemQuantity")]
    pub bonus_item_quantity: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowerStandColors {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ColorCode")]
    pub color_code: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowerStandIdolPictures {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowerStandTypes {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GachaCampaigns {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CampaignName")]
    pub campaign_name: String,
    #[serde(rename = "CampaignType")]
    pub campaign_type: i32,
    #[serde(rename = "ConsectiveTimesType")]
    pub consective_times_type: i32,
    #[serde(rename = "ResetType")]
    pub reset_type: i32,
    #[serde(rename = "PerDayCampaignTimes")]
    pub per_day_campaign_times: i32,
    #[serde(rename = "GachaSeriesId_1")]
    pub gacha_series_id_1: i64,
    #[serde(rename = "GachaSeriesId_2")]
    pub gacha_series_id_2: i64,
    #[serde(rename = "GachaSeriesId_3")]
    pub gacha_series_id_3: i64,
    #[serde(rename = "GachaSeriesId_4")]
    pub gacha_series_id_4: i64,
    #[serde(rename = "GachaSeriesId_5")]
    pub gacha_series_id_5: i64,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "MiniBannerPopId")]
    pub mini_banner_pop_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GachaSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GachaSeriesName")]
    pub gacha_series_name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "GachaType")]
    pub gacha_type: i32,
    #[serde(rename = "LimitedGachaCount")]
    pub limited_gacha_count: i32,
    #[serde(rename = "LimitedGachaResetType")]
    pub limited_gacha_reset_type: i32,
    #[serde(rename = "GachaExchangePointId")]
    pub gacha_exchange_point_id: i32,
    #[serde(rename = "ExchangePointNoticeNum")]
    pub exchange_point_notice_num: i32,
    #[serde(rename = "ExchangePointLockFlag")]
    pub exchange_point_lock_flag: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "FilterType")]
    pub filter_type: i32,
    #[serde(rename = "PickUpCardSeriesId_1")]
    pub pick_up_card_series_id_1: i32,
    #[serde(rename = "PickUpCardBonusItemQuantity_1")]
    pub pick_up_card_bonus_item_quantity_1: i32,
    #[serde(rename = "PickUpCardSeriesId_2")]
    pub pick_up_card_series_id_2: i32,
    #[serde(rename = "PickUpCardBonusItemQuantity_2")]
    pub pick_up_card_bonus_item_quantity_2: i32,
    #[serde(rename = "PickUpCardSeriesId_3")]
    pub pick_up_card_series_id_3: i32,
    #[serde(rename = "PickUpCardBonusItemQuantity_3")]
    pub pick_up_card_bonus_item_quantity_3: i32,
    #[serde(rename = "PickUpCardSeriesId_4")]
    pub pick_up_card_series_id_4: i32,
    #[serde(rename = "PickUpCardBonusItemQuantity_4")]
    pub pick_up_card_bonus_item_quantity_4: i32,
    #[serde(rename = "PickUpCardSeriesId_5")]
    pub pick_up_card_series_id_5: i32,
    #[serde(rename = "PickUpCardBonusItemQuantity_5")]
    pub pick_up_card_bonus_item_quantity_5: i32,
    #[serde(rename = "PickUpCardSeriesId_6")]
    pub pick_up_card_series_id_6: i32,
    #[serde(rename = "PickUpCardBonusItemQuantity_6")]
    pub pick_up_card_bonus_item_quantity_6: i32,
    #[serde(rename = "BgType")]
    pub bg_type: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "NoticeText")]
    pub notice_text: String,
    #[serde(rename = "GachaTimeLimitType")]
    pub gacha_time_limit_type: i32,
    #[serde(rename = "AvailableTime")]
    pub available_time: i32,
    #[serde(rename = "GachaStartBgm")]
    pub gacha_start_bgm: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generations {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftBonusGachas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "SingleGachaPrice")]
    pub single_gacha_price: i32,
    #[serde(rename = "SingleGachaPopId")]
    pub single_gacha_pop_id: i32,
    #[serde(rename = "ConsectiveGachaPrice")]
    pub consective_gacha_price: i32,
    #[serde(rename = "ConsectiveGachaTimes")]
    pub consective_gacha_times: i32,
    #[serde(rename = "ConsectiveGachaPopId")]
    pub consective_gacha_pop_id: i32,
    #[serde(rename = "PaidSIsCaOnlyGachaFlag")]
    pub paid_s_is_ca_only_gacha_flag: i32,
    #[serde(rename = "PaidSIsCaOnlyGachaPrice")]
    pub paid_s_is_ca_only_gacha_price: i32,
    #[serde(rename = "PaidSIsCaOnlyGachaTimes")]
    pub paid_s_is_ca_only_gacha_times: i32,
    #[serde(rename = "PaidSIsCaOnlyGachaPointFlag")]
    pub paid_s_is_ca_only_gacha_point_flag: i32,
    #[serde(rename = "PaidSIsCaOnlyGachaaPopId")]
    pub paid_s_is_ca_only_gachaa_pop_id: i32,
    #[serde(rename = "LimitedPaidGachaCount")]
    pub limited_paid_gacha_count: i32,
    #[serde(rename = "LimitedPaidGachaResetType")]
    pub limited_paid_gacha_reset_type: i32,
    #[serde(rename = "LimitedPaidButtonDesignType")]
    pub limited_paid_button_design_type: i32,
    #[serde(rename = "MiniBannerPopId")]
    pub mini_banner_pop_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftlessGachas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "SingleGachaPrice")]
    pub single_gacha_price: i32,
    #[serde(rename = "SingleGachaPopId")]
    pub single_gacha_pop_id: i32,
    #[serde(rename = "ConsectiveGachaPrice")]
    pub consective_gacha_price: i32,
    #[serde(rename = "ConsectiveGachaTimes")]
    pub consective_gacha_times: i32,
    #[serde(rename = "ConsectiveGachaPopId")]
    pub consective_gacha_pop_id: i32,
    #[serde(rename = "PaidSIsCaOnlyGachaFlag")]
    pub paid_s_is_ca_only_gacha_flag: i32,
    #[serde(rename = "PaidSIsCaOnlyGachaPrice")]
    pub paid_s_is_ca_only_gacha_price: i32,
    #[serde(rename = "PaidSIsCaOnlyGachaTimes")]
    pub paid_s_is_ca_only_gacha_times: i32,
    #[serde(rename = "PaidSIsCaOnlyGachaPointFlag")]
    pub paid_s_is_ca_only_gacha_point_flag: i32,
    #[serde(rename = "PaidSIsCaOnlyGachaaPopId")]
    pub paid_s_is_ca_only_gachaa_pop_id: i32,
    #[serde(rename = "LimitedPaidGachaCount")]
    pub limited_paid_gacha_count: i32,
    #[serde(rename = "LimitedPaidGachaResetType")]
    pub limited_paid_gacha_reset_type: i32,
    #[serde(rename = "LimitedPaidButtonDesignType")]
    pub limited_paid_button_design_type: i32,
    #[serde(rename = "MiniBannerPopId")]
    pub mini_banner_pop_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpPrizeExchanges {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ProductItemType")]
    pub product_item_type: i32,
    #[serde(rename = "ProductItemId")]
    pub product_item_id: i32,
    #[serde(rename = "ProductItemNum")]
    pub product_item_num: i32,
    #[serde(rename = "MaterialItemType")]
    pub material_item_type: i32,
    #[serde(rename = "MaterialItemId")]
    pub material_item_id: i32,
    #[serde(rename = "MaterialItemNum")]
    pub material_item_num: i32,
    #[serde(rename = "LimitNum")]
    pub limit_num: i32,
    #[serde(rename = "ResetType")]
    pub reset_type: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeAddSkillEffectDetails {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "SkillEffectDetailType")]
    pub skill_effect_detail_type: String,
    #[serde(rename = "TargetMood")]
    pub target_mood: i32,
    #[serde(rename = "EffectValue")]
    pub effect_value: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeAddSkillEffects {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ActionType")]
    pub action_type: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeAddSkills {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "SkillIcon")]
    pub skill_icon: i32,
    #[serde(rename = "GradeAddSkillEffectsId")]
    pub grade_add_skill_effects_id: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeChalQuestStageRewardDatas {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "GradeChalQuestStagesRewardsId")]
    pub grade_chal_quest_stages_rewards_id: i64,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeChalQuestStages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GradeChalSeasonId")]
    pub grade_chal_season_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Hint")]
    pub hint: String,
    #[serde(rename = "CharacterId")]
    pub character_id: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StageType")]
    pub stage_type: i32,
    #[serde(rename = "StageIconId")]
    pub stage_icon_id: i32,
    #[serde(rename = "LiveStagesId")]
    pub live_stages_id: i32,
    #[serde(rename = "QuestMusicsType")]
    pub quest_musics_type: i32,
    #[serde(rename = "QuestMusicsDetail")]
    pub quest_musics_detail: i32,
    #[serde(rename = "DeckRestrictedType")]
    pub deck_restricted_type: i32,
    #[serde(rename = "DeckRestrictedDetail")]
    pub deck_restricted_detail: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeChalQuestStagesRewards {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "GradeChalQuestStagesId")]
    pub grade_chal_quest_stages_id: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeChalSeason {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "GiveSeasonGradeId")]
    pub give_season_grade_id: i32,
    #[serde(rename = "BgImageId")]
    pub bg_image_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeChalTotalScoreRewardDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GradeChalTotalScoreRewardsId")]
    pub grade_chal_total_score_rewards_id: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeChalTotalScoreRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GradeChalSeasonId")]
    pub grade_chal_season_id: i32,
    #[serde(rename = "TotalScore")]
    pub total_score: i64,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GradeNum")]
    pub grade_num: i32,
    #[serde(rename = "GradeId")]
    pub grade_id: i32,
    #[serde(rename = "GradeTier")]
    pub grade_tier: i32,
    #[serde(rename = "RequireGradePoints")]
    pub require_grade_points: i32,
    #[serde(rename = "GradeLiveBonus")]
    pub grade_live_bonus: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeQuestLivePointBonus {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GradeQuestSeriesId")]
    pub grade_quest_series_id: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "BonusNum")]
    pub bonus_num: i32,
    #[serde(rename = "BonusLimitUp")]
    pub bonus_limit_up: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeQuestRewardsDatas {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "GradeQuestRewardsId")]
    pub grade_quest_rewards_id: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeQuestRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GradeQuestSeriesId")]
    pub grade_quest_series_id: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "ConditionsValue2")]
    pub conditions_value2: i32,
    #[serde(rename = "ConditionsDescription")]
    pub conditions_description: String,
    #[serde(rename = "DisplayType")]
    pub display_type: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeQuestSeason {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "Generation")]
    pub generation: i32,
    #[serde(rename = "Season")]
    pub season: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeQuestSeasonReleaseCond {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GradeQuestSeasonId")]
    pub grade_quest_season_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "ConditionsDescription")]
    pub conditions_description: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeQuestSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "GradeQuestSeasonId")]
    pub grade_quest_season_id: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "BackGroundId")]
    pub back_ground_id: i32,
    #[serde(rename = "MapImageId")]
    pub map_image_id: String,
    #[serde(rename = "SoundId")]
    pub sound_id: i32,
    #[serde(rename = "IsTutorial")]
    pub is_tutorial: i32,
    #[serde(rename = "DefaultGradeAddSkillsId")]
    pub default_grade_add_skills_id: String,
    #[serde(rename = "DefaultActionPoint")]
    pub default_action_point: i32,
    #[serde(rename = "LivePointBonusLimit")]
    pub live_point_bonus_limit: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeQuestSeriesReleaseCond {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GradeQuestSeriesId")]
    pub grade_quest_series_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeQuestSquareDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "SquareType")]
    pub square_type: i32,
    #[serde(rename = "TargetId")]
    pub target_id: i64,
    #[serde(rename = "MinActionPoint")]
    pub min_action_point: i32,
    #[serde(rename = "MaxActionPoint")]
    pub max_action_point: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeQuestSquare {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GradeQuestSeriesId")]
    pub grade_quest_series_id: i32,
    #[serde(rename = "SquareId")]
    pub square_id: i32,
    #[serde(rename = "XCoord")]
    pub x_coord: i32,
    #[serde(rename = "YCoord")]
    pub y_coord: i32,
    #[serde(rename = "OpenGradeQuestSquareIds")]
    pub open_grade_quest_square_ids: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeQuestStages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Hint")]
    pub hint: String,
    #[serde(rename = "StageType")]
    pub stage_type: i32,
    #[serde(rename = "LiveStagesId")]
    pub live_stages_id: i32,
    #[serde(rename = "QuestMusicsType")]
    pub quest_musics_type: i32,
    #[serde(rename = "QuestMusicsDetail")]
    pub quest_musics_detail: i32,
    #[serde(rename = "DeckRestrictedType")]
    pub deck_restricted_type: i32,
    #[serde(rename = "DeckRestrictedDetail")]
    pub deck_restricted_detail: i32,
    #[serde(rename = "LivePoint")]
    pub live_point: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grade {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MaxGradeTier")]
    pub max_grade_tier: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeRewardDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GradeRewardsId")]
    pub grade_rewards_id: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GradeNum")]
    pub grade_num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrandPrixDailyPoints {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MinRank")]
    pub min_rank: i32,
    #[serde(rename = "MaxRank")]
    pub max_rank: i32,
    #[serde(rename = "BasePoint")]
    pub base_point: i32,
    #[serde(rename = "AdditionalPoint")]
    pub additional_point: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrandPrixPointBonuses {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "GrandPrixesId")]
    pub grand_prixes_id: i32,
    #[serde(rename = "TargetType")]
    pub target_type: i32,
    #[serde(rename = "TargetDetail")]
    pub target_detail: i32,
    #[serde(rename = "TargetNum")]
    pub target_num: i32,
    #[serde(rename = "BonusValue")]
    pub bonus_value: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrandPrixQuestSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "GrandPrixesId")]
    pub grand_prixes_id: i32,
    #[serde(rename = "PlayLimitCount")]
    pub play_limit_count: i32,
    #[serde(rename = "RetireLimitCount")]
    pub retire_limit_count: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "SeriesNum")]
    pub series_num: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrandPrixQuestStages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GrandPrixSeriesId")]
    pub grand_prix_series_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Hint")]
    pub hint: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StageType")]
    pub stage_type: i32,
    #[serde(rename = "LiveStagesId")]
    pub live_stages_id: i32,
    #[serde(rename = "QuestMusicsType")]
    pub quest_musics_type: i32,
    #[serde(rename = "QuestMusicsDetail")]
    pub quest_musics_detail: i32,
    #[serde(rename = "DeckRestrictedType")]
    pub deck_restricted_type: i32,
    #[serde(rename = "DeckRestrictedDetail")]
    pub deck_restricted_detail: i32,
    #[serde(rename = "QuestLevel")]
    pub quest_level: i32,
    #[serde(rename = "QuestRank")]
    pub quest_rank: i32,
    #[serde(rename = "FirstClearRewardSeriesId")]
    pub first_clear_reward_series_id: i64,
    #[serde(rename = "CompleteRewardSeriesId")]
    pub complete_reward_series_id: i64,
    #[serde(rename = "DropRewardSeriesId")]
    pub drop_reward_series_id: i64,
    #[serde(rename = "RandomDropRewardSeriesId")]
    pub random_drop_reward_series_id: i64,
    #[serde(rename = "Score1")]
    pub score1: i64,
    #[serde(rename = "Score2")]
    pub score2: i64,
    #[serde(rename = "Score3")]
    pub score3: i64,
    #[serde(rename = "StylePoint")]
    pub style_point: i32,
    #[serde(rename = "GainMusicExp")]
    pub gain_music_exp: i32,
    #[serde(rename = "ScoreBonusValue0")]
    pub score_bonus_value0: i32,
    #[serde(rename = "ScoreBonusValue1")]
    pub score_bonus_value1: i32,
    #[serde(rename = "ScoreBonusValue2")]
    pub score_bonus_value2: i32,
    #[serde(rename = "ScoreBonusValue3")]
    pub score_bonus_value3: i32,
    #[serde(rename = "SkipRestrictedType")]
    pub skip_restricted_type: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrandPrix {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "GrandPrixType")]
    pub grand_prix_type: i32,
    #[serde(rename = "GuildRankingTabs")]
    pub guild_ranking_tabs: String,
    #[serde(rename = "PersonalRankingTabs")]
    pub personal_ranking_tabs: String,
    #[serde(rename = "GuildPresentCommentId")]
    pub guild_present_comment_id: i32,
    #[serde(rename = "PersonalPresentCommentId")]
    pub personal_present_comment_id: i32,
    #[serde(rename = "InfoStartTime")]
    pub info_start_time: DateTime<Utc>,
    #[serde(rename = "InfoEndTime")]
    pub info_end_time: DateTime<Utc>,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrandPrixReleaseCondition {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ReleaseGrandPrixId")]
    pub release_grand_prix_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrandPrixRewardDatas {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "GrandPrixRewardsId")]
    pub grand_prix_rewards_id: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "IsEmphasize")]
    pub is_emphasize: i32,
    #[serde(rename = "IsUsePresentBox")]
    pub is_use_present_box: i32,
    #[serde(rename = "LifeTimeDay")]
    pub life_time_day: i32,
    #[serde(rename = "RewardTextId")]
    pub reward_text_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrandPrixRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GrandPrixesId")]
    pub grand_prixes_id: i32,
    #[serde(rename = "GrandPrixRewardType")]
    pub grand_prix_reward_type: i32,
    #[serde(rename = "MinTargetNum")]
    pub min_target_num: i32,
    #[serde(rename = "MaxTargetNum")]
    pub max_target_num: i32,
    #[serde(rename = "IsDisplay")]
    pub is_display: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpImages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Scene")]
    pub scene: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "TitleTextId")]
    pub title_text_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeBgms {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "DaytimeBgmId")]
    pub daytime_bgm_id: i32,
    #[serde(rename = "NighttimeBgmId")]
    pub nighttime_bgm_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngameMissionSkillDetails {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "GroupId")]
    pub group_id: i64,
    #[serde(rename = "RareFlag")]
    pub rare_flag: i32,
    #[serde(rename = "MissionType")]
    pub mission_type: i32,
    #[serde(rename = "Param1")]
    pub param1: i64,
    #[serde(rename = "Param2")]
    pub param2: i64,
    #[serde(rename = "AddParam")]
    pub add_param: i64,
    #[serde(rename = "BaseScore")]
    pub base_score: i64,
    #[serde(rename = "AddScore")]
    pub add_score: i64,
    #[serde(rename = "Probability")]
    pub probability: i32,
    #[serde(rename = "MissionText")]
    pub mission_text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemExchangeCategoryDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ItemExchangeCategoryName")]
    pub item_exchange_category_name: String,
    #[serde(rename = "ItemId")]
    pub item_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemExchanges {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ItemExchangeCategoryId")]
    pub item_exchange_category_id: i32,
    #[serde(rename = "ProductItemType")]
    pub product_item_type: i32,
    #[serde(rename = "ProductItemId")]
    pub product_item_id: i32,
    #[serde(rename = "ProductItemNum")]
    pub product_item_num: i32,
    #[serde(rename = "MaterialItemType")]
    pub material_item_type: i32,
    #[serde(rename = "MaterialItemId")]
    pub material_item_id: i32,
    #[serde(rename = "MaterialItemNum")]
    pub material_item_num: i32,
    #[serde(rename = "LimitNum")]
    pub limit_num: i32,
    #[serde(rename = "ResetType")]
    pub reset_type: i32,
    #[serde(rename = "IsVisibleEndTime")]
    pub is_visible_end_time: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSources {
    #[serde(rename = "ItemsId")]
    pub items_id: i32,
    #[serde(rename = "StandardQuestStagesId")]
    pub standard_quest_stages_id: String,
    #[serde(rename = "DailyQuestStagesId")]
    pub daily_quest_stages_id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Items {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NameFurigana")]
    pub name_furigana: String,
    #[serde(rename = "ItemType")]
    pub item_type: i32,
    #[serde(rename = "ItemCategory")]
    pub item_category: i32,
    #[serde(rename = "Rarity")]
    pub rarity: i32,
    #[serde(rename = "EffectValue")]
    pub effect_value: i32,
    #[serde(rename = "LimitNum")]
    pub limit_num: i32,
    #[serde(rename = "RequestableNum")]
    pub requestable_num: i32,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherBanners {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "DisplayPosition")]
    pub display_position: i32,
    #[serde(rename = "ImageId")]
    pub image_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningLiveReleaseConditions {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ReleaseLearningLiveId")]
    pub release_learning_live_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelUpMaterialDetails {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MaterialDetailSeriesId")]
    pub material_detail_series_id: i32,
    #[serde(rename = "CostItemsId")]
    pub cost_items_id: i32,
    #[serde(rename = "CostNum")]
    pub cost_num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelUpMaterials {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MaterialSeriesId")]
    pub material_series_id: i32,
    #[serde(rename = "SkillLevel")]
    pub skill_level: i32,
    #[serde(rename = "MaterialDetailSeriesId")]
    pub material_detail_series_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitBreakMaterialConvertRate {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "LimitBreakMaterialRarity")]
    pub limit_break_material_rarity: i32,
    #[serde(rename = "LimitBreakMaterialNum")]
    pub limit_break_material_num: i32,
    #[serde(rename = "ProductItemType")]
    pub product_item_type: i32,
    #[serde(rename = "ProductItemId")]
    pub product_item_id: i32,
    #[serde(rename = "ProductItemNum")]
    pub product_item_num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitBreakMaterialRate {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CardSeriesId")]
    pub card_series_id: i32,
    #[serde(rename = "LimitBreakMaterialId")]
    pub limit_break_material_id: i32,
    #[serde(rename = "LimitBreakMaterialQuantity")]
    pub limit_break_material_quantity: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveChannels {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveStages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "BaseUseMental")]
    pub base_use_mental: i32,
    #[serde(rename = "MentalStepClass")]
    pub mental_step_class: i32,
    #[serde(rename = "BaseGainVoltage")]
    pub base_gain_voltage: i32,
    #[serde(rename = "VoltageStepClass")]
    pub voltage_step_class: i32,
    #[serde(rename = "BackGroundId")]
    pub back_ground_id: i32,
    #[serde(rename = "StageSkillConditionId")]
    pub stage_skill_condition_id: i64,
    #[serde(rename = "StageSkillEffectId")]
    pub stage_skill_effect_id: i64,
    #[serde(rename = "StageSkillDescription")]
    pub stage_skill_description: String,
    #[serde(rename = "StageSkillSetIds")]
    pub stage_skill_set_ids: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginBonuses {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "LoginBonusRewardSeriesID")]
    pub login_bonus_reward_series_id: i32,
    #[serde(rename = "IsLoop")]
    pub is_loop: i32,
    #[serde(rename = "LoginBonusTextId")]
    pub login_bonus_text_id: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginBonusRewardDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "LoginBonusRewardSeriesId")]
    pub login_bonus_reward_series_id: i32,
    #[serde(rename = "DayCount")]
    pub day_count: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardId")]
    pub reward_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "LifeTimeDay")]
    pub life_time_day: i32,
    #[serde(rename = "RewardTextId")]
    pub reward_text_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberFanLevels {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MemberFanLevel")]
    pub member_fan_level: i32,
    #[serde(rename = "Experience")]
    pub experience: i32,
    #[serde(rename = "CumulativeExperience")]
    pub cumulative_experience: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberMovies {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
    #[serde(rename = "MovieType")]
    pub movie_type: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Priority")]
    pub priority: i32,
    #[serde(rename = "ReleaseConditionText")]
    pub release_condition_text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberVoices {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Priority")]
    pub priority: i32,
    #[serde(rename = "VoiceName")]
    pub voice_name: String,
    #[serde(rename = "ReleaseConditionText")]
    pub release_condition_text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionAchieveRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MissionType")]
    pub mission_type: i32,
    #[serde(rename = "AchieveMarkNum")]
    pub achieve_mark_num: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "ItemsId")]
    pub items_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "RewardTextId")]
    pub reward_text_id: i32,
    #[serde(rename = "SortOrder")]
    pub sort_order: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MissionsId")]
    pub missions_id: i32,
    #[serde(rename = "RewardCategory")]
    pub reward_category: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "ItemsId")]
    pub items_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "RewardTextId")]
    pub reward_text_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Missions {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "MissionType")]
    pub mission_type: i32,
    #[serde(rename = "MissionConditionType")]
    pub mission_condition_type: i32,
    #[serde(rename = "MissionConditionNum")]
    pub mission_condition_num: i32,
    #[serde(rename = "MissionConditionDetail")]
    pub mission_condition_detail: i32,
    #[serde(rename = "OpenType")]
    pub open_type: i32,
    #[serde(rename = "NextMissionsId")]
    pub next_missions_id: i32,
    #[serde(rename = "SortOrder")]
    pub sort_order: i32,
    #[serde(rename = "TransitionContentsId")]
    pub transition_contents_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "Hint")]
    pub hint: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicDropRewardDetails {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MusicDropRewardsId")]
    pub music_drop_rewards_id: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "Odds")]
    pub odds: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicDropRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "DropRewardSeriesId")]
    pub drop_reward_series_id: i32,
    #[serde(rename = "MusicDropRewardOdds")]
    pub music_drop_reward_odds: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLearningQuestSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MusicsId")]
    pub musics_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "ExReleaseConditionsLevel")]
    pub ex_release_conditions_level: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLearningQuestStages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "LearningLiveSeriesId")]
    pub learning_live_series_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Hint")]
    pub hint: String,
    #[serde(rename = "StageType")]
    pub stage_type: i32,
    #[serde(rename = "UseType")]
    pub use_type: i32,
    #[serde(rename = "UseItem")]
    pub use_item: i32,
    #[serde(rename = "UseNum")]
    pub use_num: i32,
    #[serde(rename = "LiveStagesId")]
    pub live_stages_id: i32,
    #[serde(rename = "QuestMusicsType")]
    pub quest_musics_type: i32,
    #[serde(rename = "QuestMusicsDetail")]
    pub quest_musics_detail: i32,
    #[serde(rename = "DeckRestrictedType")]
    pub deck_restricted_type: i32,
    #[serde(rename = "DeckRestrictedDetail")]
    pub deck_restricted_detail: i32,
    #[serde(rename = "QuestLevel")]
    pub quest_level: i32,
    #[serde(rename = "QuestRank")]
    pub quest_rank: i32,
    #[serde(rename = "FirstClearRewardSeriesId")]
    pub first_clear_reward_series_id: i64,
    #[serde(rename = "DropRewardSeriesId")]
    pub drop_reward_series_id: i64,
    #[serde(rename = "RandomDropRewardSeriesId")]
    pub random_drop_reward_series_id: i64,
    #[serde(rename = "Score1")]
    pub score1: i64,
    #[serde(rename = "GainStylePoint")]
    pub gain_style_point: i32,
    #[serde(rename = "GainMusicExp")]
    pub gain_music_exp: i32,
    #[serde(rename = "SkipRestrictedType")]
    pub skip_restricted_type: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLevels {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ExperienceType")]
    pub experience_type: i32,
    #[serde(rename = "Level")]
    pub level: i32,
    #[serde(rename = "Experience")]
    pub experience: i32,
    #[serde(rename = "CumulativeExperience")]
    pub cumulative_experience: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicMasteryHeartBonuses {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Level")]
    pub level: i32,
    #[serde(rename = "LoveRate")]
    pub love_rate: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicMasteryLevels {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MusicsId")]
    pub musics_id: i32,
    #[serde(rename = "Level")]
    pub level: i32,
    #[serde(rename = "MusicMasterySkillsId")]
    pub music_mastery_skills_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicMasteryLoveBonuses {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Level")]
    pub level: i32,
    #[serde(rename = "LoveRate")]
    pub love_rate: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicMasteryMentalBonuses {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Level")]
    pub level: i32,
    #[serde(rename = "DemandDamagePt")]
    pub demand_damage_pt: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicMasterySkill {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MusicMasterySkillsName")]
    pub music_mastery_skills_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicMasteryVoltageBonuses {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Level")]
    pub level: i32,
    #[serde(rename = "DemandVoltagePt")]
    pub demand_voltage_pt: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicScoreRewardDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "MissionStars")]
    pub mission_stars: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicScoreRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ScoreRewardSeriesId")]
    pub score_reward_series_id: i32,
    #[serde(rename = "ScoreRewardType")]
    pub score_reward_type: i32,
    #[serde(rename = "ScoreRewardConditionValue1")]
    pub score_reward_condition_value1: i64,
    #[serde(rename = "ScoreRewardConditionValue2")]
    pub score_reward_condition_value2: i64,
    #[serde(rename = "ScoreRewardConditionValue3")]
    pub score_reward_condition_value3: i64,
    #[serde(rename = "ScoreRewardConditionValue4")]
    pub score_reward_condition_value4: i64,
    #[serde(rename = "ScoreRewardDatasId1")]
    pub score_reward_datas_id1: i32,
    #[serde(rename = "ScoreRewardDatasId2")]
    pub score_reward_datas_id2: i32,
    #[serde(rename = "ScoreRewardDatasId3")]
    pub score_reward_datas_id3: i32,
    #[serde(rename = "ScoreRewardDatasId4")]
    pub score_reward_datas_id4: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicScores {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "NormalLevel")]
    pub normal_level: i32,
    #[serde(rename = "HardLevel")]
    pub hard_level: i32,
    #[serde(rename = "ExpertLevel")]
    pub expert_level: i32,
    #[serde(rename = "MasterLevel")]
    pub master_level: i32,
    #[serde(rename = "NormalMaxCombo")]
    pub normal_max_combo: i32,
    #[serde(rename = "HardMaxCombo")]
    pub hard_max_combo: i32,
    #[serde(rename = "ExpertMaxCombo")]
    pub expert_max_combo: i32,
    #[serde(rename = "MasterMaxCombo")]
    pub master_max_combo: i32,
    #[serde(rename = "ShouldVerifyNotesCount")]
    pub should_verify_notes_count: i32,
    #[serde(rename = "ScoreRewardSeriesId")]
    pub score_reward_series_id: i32,
    #[serde(rename = "NormalGainMusicExp")]
    pub normal_gain_music_exp: i32,
    #[serde(rename = "HardGainMusicExp")]
    pub hard_gain_music_exp: i32,
    #[serde(rename = "ExpertGainMusicExp")]
    pub expert_gain_music_exp: i32,
    #[serde(rename = "MasterGainMusicExp")]
    pub master_gain_music_exp: i32,
    #[serde(rename = "NormalDropRewardSeriesId")]
    pub normal_drop_reward_series_id: i32,
    #[serde(rename = "HardDropRewardSeriesId")]
    pub hard_drop_reward_series_id: i32,
    #[serde(rename = "ExpertDropRewardSeriesId")]
    pub expert_drop_reward_series_id: i32,
    #[serde(rename = "MasterDropRewardSeriesId")]
    pub master_drop_reward_series_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Musics {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "TitleFurigana")]
    pub title_furigana: String,
    #[serde(rename = "JacketId")]
    pub jacket_id: i32,
    #[serde(rename = "SoundId")]
    pub sound_id: i32,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "GenerationsId")]
    pub generations_id: i32,
    #[serde(rename = "UnitId")]
    pub unit_id: i32,
    #[serde(rename = "CenterCharacterId")]
    pub center_character_id: i32,
    #[serde(rename = "SingerCharacterId")]
    pub singer_character_id: String,
    #[serde(rename = "SupportCharacterId")]
    pub support_character_id: String,
    #[serde(rename = "MusicType")]
    pub music_type: i32,
    #[serde(rename = "ExperienceType")]
    pub experience_type: i32,
    #[serde(rename = "BeatPointCoefficient")]
    pub beat_point_coefficient: i32,
    #[serde(rename = "ApIncrement")]
    pub ap_increment: i32,
    #[serde(rename = "SongTime")]
    pub song_time: i32,
    #[serde(rename = "PlayTime")]
    pub play_time: i32,
    #[serde(rename = "FeverSectionNo")]
    pub fever_section_no: i32,
    #[serde(rename = "PreviewStartTime")]
    pub preview_start_time: i32,
    #[serde(rename = "PreviewEndTime")]
    pub preview_end_time: i32,
    #[serde(rename = "PreviewFadeInTime")]
    pub preview_fade_in_time: i32,
    #[serde(rename = "PreviewFadeOutTime")]
    pub preview_fade_out_time: i32,
    #[serde(rename = "ReleaseConditionType")]
    pub release_condition_type: i32,
    #[serde(rename = "ReleaseConditionDetail")]
    pub release_condition_detail: i32,
    #[serde(rename = "ReleaseConditionText")]
    pub release_condition_text: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "MaxAp")]
    pub max_ap: i32,
    #[serde(rename = "IsVideoMode")]
    pub is_video_mode: i32,
    #[serde(rename = "VideoBgId")]
    pub video_bg_id: i32,
    #[serde(rename = "SongType")]
    pub song_type: i32,
    #[serde(rename = "MusicScoreReleaseTime")]
    pub music_score_release_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetalCoinExchangeRate {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Rarity")]
    pub rarity: i32,
    #[serde(rename = "PetalCoinQuantity")]
    pub petal_coin_quantity: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetalExchangeRates {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Rarity")]
    pub rarity: i32,
    #[serde(rename = "Price")]
    pub price: i32,
    #[serde(rename = "ExchangeLimitLower")]
    pub exchange_limit_lower: i32,
    #[serde(rename = "ExchangeLimitUpper")]
    pub exchange_limit_upper: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresentTexts {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Description")]
    pub description: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestAreaReleaseConditions {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ReleaseAreaId")]
    pub release_area_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestLiveDownloads {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestLiveLoadings {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestLiveReleaseConditions {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ReleaseQuestId")]
    pub release_quest_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestSections {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "SectionNo")]
    pub section_no: i32,
    #[serde(rename = "QuestStagesId")]
    pub quest_stages_id: i32,
    #[serde(rename = "SectionSkillsId")]
    pub section_skills_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidEvents {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "DropNumUp")]
    pub drop_num_up: i32,
    #[serde(rename = "PersonalRankingTabs")]
    pub personal_ranking_tabs: String,
    #[serde(rename = "InfoStartTime")]
    pub info_start_time: DateTime<Utc>,
    #[serde(rename = "InfoEndTime")]
    pub info_end_time: DateTime<Utc>,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidQuestDropRateUp {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Flame")]
    pub flame: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidQuestReleaseCondition {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ReleaseRaidId")]
    pub release_raid_id: i32,
    #[serde(rename = "ConditionsType")]
    pub conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidQuestSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "RaidEventId")]
    pub raid_event_id: i32,
    #[serde(rename = "PointAddLimit")]
    pub point_add_limit: i64,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidQuestStages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RaidQuestSeriesId")]
    pub raid_quest_series_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Hint")]
    pub hint: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StageType")]
    pub stage_type: i32,
    #[serde(rename = "LiveStagesId")]
    pub live_stages_id: i32,
    #[serde(rename = "QuestMusicsType")]
    pub quest_musics_type: i32,
    #[serde(rename = "QuestMusicsDetail")]
    pub quest_musics_detail: i32,
    #[serde(rename = "DeckRestrictedType")]
    pub deck_restricted_type: i32,
    #[serde(rename = "DeckRestrictedDetail")]
    pub deck_restricted_detail: i32,
    #[serde(rename = "QuestLevel")]
    pub quest_level: i32,
    #[serde(rename = "QuestRank")]
    pub quest_rank: i32,
    #[serde(rename = "FirstClearRewardSeriesId")]
    pub first_clear_reward_series_id: i64,
    #[serde(rename = "CompleteRewardSeriesId")]
    pub complete_reward_series_id: i64,
    #[serde(rename = "DropRewardSeriesId")]
    pub drop_reward_series_id: i64,
    #[serde(rename = "RandomDropRewardSeriesId")]
    pub random_drop_reward_series_id: i64,
    #[serde(rename = "Score1")]
    pub score1: i64,
    #[serde(rename = "Score2")]
    pub score2: i64,
    #[serde(rename = "Score3")]
    pub score3: i64,
    #[serde(rename = "StylePoint")]
    pub style_point: i32,
    #[serde(rename = "GainMusicExp")]
    pub gain_music_exp: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidResourceAddDate {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RaidResourceId")]
    pub raid_resource_id: i32,
    #[serde(rename = "AddNum")]
    pub add_num: i32,
    #[serde(rename = "AddTime")]
    pub add_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidResource {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RaidEventId")]
    pub raid_event_id: i32,
    #[serde(rename = "RaidResourceDefaultNum")]
    pub raid_resource_default_num: i32,
    #[serde(rename = "RaidResourceLimit")]
    pub raid_resource_limit: i32,
    #[serde(rename = "RaidResourceAddLimit")]
    pub raid_resource_add_limit: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidResourceRecoveryDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RaidResourceId")]
    pub raid_resource_id: i32,
    #[serde(rename = "Order")]
    pub order: i32,
    #[serde(rename = "RequireItemType")]
    pub require_item_type: i32,
    #[serde(rename = "RequireItemId")]
    pub require_item_id: i32,
    #[serde(rename = "RequireItemNum")]
    pub require_item_num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidRewardDatas {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "RaidRewardsId")]
    pub raid_rewards_id: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RaidEventId")]
    pub raid_event_id: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RequirePointAmount")]
    pub require_point_amount: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidTopProgressImage {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RaidEventId")]
    pub raid_event_id: i32,
    #[serde(rename = "Order")]
    pub order: i32,
    #[serde(rename = "RequirePoint")]
    pub require_point: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalCardDatas {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "CardDatasId")]
    pub card_datas_id: i32,
    #[serde(rename = "StyleLevel")]
    pub style_level: i32,
    #[serde(rename = "LimitBreakLevel")]
    pub limit_break_level: i32,
    #[serde(rename = "SpecialAppealLevel")]
    pub special_appeal_level: i32,
    #[serde(rename = "SkillLevel")]
    pub skill_level: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalDeckCards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RentalDecksId")]
    pub rental_decks_id: i32,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
    #[serde(rename = "RentalCardId_Main")]
    pub rental_card_id__main: i64,
    #[serde(rename = "RentalCardId_Side1")]
    pub rental_card_id__side1: i64,
    #[serde(rename = "RentalCardId_Side2")]
    pub rental_card_id__side2: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalDecks {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "DeckName")]
    pub deck_name: String,
    #[serde(rename = "GenerationsId")]
    pub generations_id: i32,
    #[serde(rename = "DeckNumber")]
    pub deck_number: i32,
    #[serde(rename = "AceCardId")]
    pub ace_card_id: i64,
    #[serde(rename = "ReleaseConditionsType")]
    pub release_conditions_type: i32,
    #[serde(rename = "ConditionsValue")]
    pub conditions_value: i32,
    #[serde(rename = "ReleaseConditonsDescription")]
    pub release_conditons_description: String,
    #[serde(rename = "ReleaseDeckStartTime")]
    pub release_deck_start_time: DateTime<Utc>,
    #[serde(rename = "ReleaseDeckEndTime")]
    pub release_deck_end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmGameClassDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "RhythmGameConditionType")]
    pub rhythm_game_condition_type: i32,
    #[serde(rename = "RhythmGameClassesId")]
    pub rhythm_game_classes_id: i32,
    #[serde(rename = "ClassTier")]
    pub class_tier: i32,
    #[serde(rename = "GoalCount")]
    pub goal_count: i64,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "ReleaseStartTime")]
    pub release_start_time: DateTime<Utc>,
    #[serde(rename = "ReleaseEndTime")]
    pub release_end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmGameClasses {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ClassName")]
    pub class_name: String,
    #[serde(rename = "MaxClassTier")]
    pub max_class_tier: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmGameClassMissionRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RhythmGameClassDatasId")]
    pub rhythm_game_class_datas_id: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmGameHelpImages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Scene")]
    pub scene: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmGameSkillConditions {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RhythmGameSkillConditionType")]
    pub rhythm_game_skill_condition_type: i32,
    #[serde(rename = "RhythmGameSkillConditionValue")]
    pub rhythm_game_skill_condition_value: i32,
    #[serde(rename = "RhythmGameSkillConditionValue2")]
    pub rhythm_game_skill_condition_value2: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmGameSkillEffects {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RhythmGameSkillEffectType")]
    pub rhythm_game_skill_effect_type: i32,
    #[serde(rename = "TargetIds")]
    pub target_ids: String,
    #[serde(rename = "RhythmGameSkillEffectValue")]
    pub rhythm_game_skill_effect_value: i32,
    #[serde(rename = "RhythmGameSkillEffectValue2")]
    pub rhythm_game_skill_effect_value2: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmGameSkillLvUpItemDetails {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ItemDetailSeriesId")]
    pub item_detail_series_id: i32,
    #[serde(rename = "CostItemsId")]
    pub cost_items_id: i32,
    #[serde(rename = "CostNum")]
    pub cost_num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmGameSkillLvUpItems {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ItemSeriesId")]
    pub item_series_id: i32,
    #[serde(rename = "SkillLevel")]
    pub skill_level: i32,
    #[serde(rename = "ItemDetailSeriesId")]
    pub item_detail_series_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmGameSkills {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "RhythmGameSkillSeriesId")]
    pub rhythm_game_skill_series_id: i32,
    #[serde(rename = "RhythmGameSkillName")]
    pub rhythm_game_skill_name: String,
    #[serde(rename = "SkillLevel")]
    pub skill_level: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "RhythmGameSkillConditionIds")]
    pub rhythm_game_skill_condition_ids: String,
    #[serde(rename = "RhythmGameSkillEffectId")]
    pub rhythm_game_skill_effect_id: i32,
    #[serde(rename = "ConsumeAP")]
    pub consume_ap: i32,
    #[serde(rename = "Description")]
    pub description: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmGameTotalMissionRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "RhythmGameTotalMissionsId")]
    pub rhythm_game_total_missions_id: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmGameTotalMissions {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "ConditionRhythmGameClassDatasOrderId")]
    pub condition_rhythm_game_class_datas_order_id: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "ReleaseStartTime")]
    pub release_start_time: DateTime<Utc>,
    #[serde(rename = "ReleaseEndTime")]
    pub release_end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonFanLevels {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "SeasonsId")]
    pub seasons_id: i32,
    #[serde(rename = "SeasonFanLevel")]
    pub season_fan_level: i32,
    #[serde(rename = "Experience")]
    pub experience: i32,
    #[serde(rename = "CumulativeExperience")]
    pub cumulative_experience: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonGrade {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "PersonalRankingTabs")]
    pub personal_ranking_tabs: String,
    #[serde(rename = "CountStartTime")]
    pub count_start_time: DateTime<Utc>,
    #[serde(rename = "CountEndTime")]
    pub count_end_time: DateTime<Utc>,
    #[serde(rename = "DisplayStartTime")]
    pub display_start_time: DateTime<Utc>,
    #[serde(rename = "DisplayEndTime")]
    pub display_end_time: DateTime<Utc>,
    #[serde(rename = "TermTitle")]
    pub term_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonGradeRewardDatas {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "SeasonGradeRewardsId")]
    pub season_grade_rewards_id: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "IsEmphasize")]
    pub is_emphasize: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonGradeRewards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "SeasonGradeId")]
    pub season_grade_id: i32,
    #[serde(rename = "MinTargetNum")]
    pub min_target_num: i32,
    #[serde(rename = "MaxTargetNum")]
    pub max_target_num: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seasons {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionSkillEffectDetails {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "SkillEffectDetailType")]
    pub skill_effect_detail_type: String,
    #[serde(rename = "TargetMood")]
    pub target_mood: i32,
    #[serde(rename = "EffectValue")]
    pub effect_value: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionSkillEffects {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ActionType")]
    pub action_type: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionSkills {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "ApperanceType")]
    pub apperance_type: i32,
    #[serde(rename = "SkillIcon")]
    pub skill_icon: i32,
    #[serde(rename = "SectionSkillsEffectId")]
    pub section_skills_effect_id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectTicketExchangeRate {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "SelectTicketSeriesId")]
    pub select_ticket_series_id: i32,
    #[serde(rename = "ExchangeItemType")]
    pub exchange_item_type: i32,
    #[serde(rename = "ExchangeItemId")]
    pub exchange_item_id: i32,
    #[serde(rename = "ExchangeItemQuantity")]
    pub exchange_item_quantity: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectTicketSeries {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ExchangeTicketName")]
    pub exchange_ticket_name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "ExchangeTicketId")]
    pub exchange_ticket_id: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "PickUpCardSeriesId_1")]
    pub pick_up_card_series_id_1: i32,
    #[serde(rename = "PickUpCardSeriesId_2")]
    pub pick_up_card_series_id_2: i32,
    #[serde(rename = "PickUpCardSeriesId_3")]
    pub pick_up_card_series_id_3: i32,
    #[serde(rename = "PickUpCardSeriesId_4")]
    pub pick_up_card_series_id_4: i32,
    #[serde(rename = "PickUpCardSeriesId_5")]
    pub pick_up_card_series_id_5: i32,
    #[serde(rename = "PickUpCardSeriesId_6")]
    pub pick_up_card_series_id_6: i32,
    #[serde(rename = "BgType")]
    pub bg_type: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "IsVisibleEndTime")]
    pub is_visible_end_time: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopItems {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ShopId")]
    pub shop_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ItemType")]
    pub item_type: i32,
    #[serde(rename = "ItemId")]
    pub item_id: i32,
    #[serde(rename = "ItemQuantity")]
    pub item_quantity: i32,
    #[serde(rename = "Price")]
    pub price: i32,
    #[serde(rename = "IsPaidSIsCaOnly")]
    pub is_paid_s_is_ca_only: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "RewardTextId")]
    pub reward_text_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shops {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ShopType")]
    pub shop_type: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideStyleSettings {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CardSeriesId")]
    pub card_series_id: i32,
    #[serde(rename = "GenerationsId")]
    pub generations_id: i32,
    #[serde(rename = "SideStyleSetCharacterIds")]
    pub side_style_set_character_ids: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationGraphLimit {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "NumberOfMember")]
    pub number_of_member: i32,
    #[serde(rename = "UpperLimitSmile")]
    pub upper_limit_smile: i32,
    #[serde(rename = "UpperLimitPure")]
    pub upper_limit_pure: i32,
    #[serde(rename = "UpperLimitCool")]
    pub upper_limit_cool: i32,
    #[serde(rename = "UpperLimitMental")]
    pub upper_limit_mental: i32,
    #[serde(rename = "UpperLimitBP")]
    pub upper_limit_bp: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageScoreMultiplierSettings {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "MinTargetNum")]
    pub min_target_num: i32,
    #[serde(rename = "MaxTargetNum")]
    pub max_target_num: i32,
    #[serde(rename = "BaseMultiplier")]
    pub base_multiplier: i64,
    #[serde(rename = "AdditionalMultiplier")]
    pub additional_multiplier: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageSkillConditionDetails {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "StageSkillConditionId")]
    pub stage_skill_condition_id: i64,
    #[serde(rename = "SkillConditionDetailType")]
    pub skill_condition_detail_type: String,
    #[serde(rename = "ConditionValue")]
    pub condition_value: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageSkillConditions {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ConditionType")]
    pub condition_type: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageSkillEffectDetails {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "StageSkillEffectId")]
    pub stage_skill_effect_id: i64,
    #[serde(rename = "SkillEffectDetailType")]
    pub skill_effect_detail_type: String,
    #[serde(rename = "EffectValue")]
    pub effect_value: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageSkillEffects {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ActionType")]
    pub action_type: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageSkillSets {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "StageSkillConditionId")]
    pub stage_skill_condition_id: i64,
    #[serde(rename = "StageSkillEffectId")]
    pub stage_skill_effect_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stamps {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "StampNo")]
    pub stamp_no: i32,
    #[serde(rename = "StampType")]
    pub stamp_type: i32,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardQuestAreas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Number")]
    pub number: i32,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "ImageType")]
    pub image_type: i32,
    #[serde(rename = "BgImageId")]
    pub bg_image_id: i32,
    #[serde(rename = "SoundId")]
    pub sound_id: i32,
    #[serde(rename = "GenerationsId")]
    pub generations_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardQuestStages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "StandardQuestAreasId")]
    pub standard_quest_areas_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Hint")]
    pub hint: String,
    #[serde(rename = "StageType")]
    pub stage_type: i32,
    #[serde(rename = "UseType")]
    pub use_type: i32,
    #[serde(rename = "UseItem")]
    pub use_item: i32,
    #[serde(rename = "UseNum")]
    pub use_num: i32,
    #[serde(rename = "LiveStagesId")]
    pub live_stages_id: i32,
    #[serde(rename = "QuestMusicsType")]
    pub quest_musics_type: i32,
    #[serde(rename = "QuestMusicsDetail")]
    pub quest_musics_detail: i32,
    #[serde(rename = "DeckRestrictedType")]
    pub deck_restricted_type: i32,
    #[serde(rename = "DeckRestrictedDetail")]
    pub deck_restricted_detail: i32,
    #[serde(rename = "QuestLevel")]
    pub quest_level: i32,
    #[serde(rename = "FirstClearRewardSeriesId")]
    pub first_clear_reward_series_id: i64,
    #[serde(rename = "CompleteRewardSeriesId")]
    pub complete_reward_series_id: i64,
    #[serde(rename = "DropRewardSeriesId")]
    pub drop_reward_series_id: i64,
    #[serde(rename = "RandomDropRewardSeriesId")]
    pub random_drop_reward_series_id: i64,
    #[serde(rename = "Score1")]
    pub score1: i64,
    #[serde(rename = "Score2")]
    pub score2: i64,
    #[serde(rename = "Score3")]
    pub score3: i64,
    #[serde(rename = "GainStylePoint")]
    pub gain_style_point: i32,
    #[serde(rename = "GainMusicExp")]
    pub gain_music_exp: i32,
    #[serde(rename = "ItemSourceIconId")]
    pub item_source_icon_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerExchanges {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "ProductStickerId")]
    pub product_sticker_id: i32,
    #[serde(rename = "ProductStickerNum")]
    pub product_sticker_num: i32,
    #[serde(rename = "MaterialItemType")]
    pub material_item_type: i32,
    #[serde(rename = "MaterialItemId")]
    pub material_item_id: i32,
    #[serde(rename = "MaterialItemNum")]
    pub material_item_num: i32,
    #[serde(rename = "LimitNum")]
    pub limit_num: i32,
    #[serde(rename = "ResetType")]
    pub reset_type: i32,
    #[serde(rename = "IsVisibleEndTime")]
    pub is_visible_end_time: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stickers {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "CategoryType")]
    pub category_type: i32,
    #[serde(rename = "CategoryName")]
    pub category_name: i32,
    #[serde(rename = "SeasonId")]
    pub season_id: i32,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
    #[serde(rename = "Priority")]
    pub priority: i32,
    #[serde(rename = "IsVariant")]
    pub is_variant: i32,
    #[serde(rename = "RequirementType")]
    pub requirement_type: i32,
    #[serde(rename = "RequirementDetail")]
    pub requirement_detail: String,
    #[serde(rename = "RequirementValue")]
    pub requirement_value: i32,
    #[serde(rename = "RequirementText")]
    pub requirement_text: String,
    #[serde(rename = "EditRequirementType")]
    pub edit_requirement_type: i32,
    #[serde(rename = "EditRequirementDetail")]
    pub edit_requirement_detail: String,
    #[serde(rename = "EditRequirementValue")]
    pub edit_requirement_value: i32,
    #[serde(rename = "EditRequirementText")]
    pub edit_requirement_text: String,
    #[serde(rename = "IsVisibleOnlyPossess")]
    pub is_visible_only_possess: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "AvailableStartTime")]
    pub available_start_time: DateTime<Utc>,
    #[serde(rename = "AvailableEndTime")]
    pub available_end_time: DateTime<Utc>,
    #[serde(rename = "VariantStartTime")]
    pub variant_start_time: DateTime<Utc>,
    #[serde(rename = "VariantEndTime")]
    pub variant_end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleMovies {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CardSeriesId")]
    pub card_series_id: i32,
    #[serde(rename = "MovieType")]
    pub movie_type: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ReleaseConditionText")]
    pub release_condition_text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleVoices {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "CardSeriesId")]
    pub card_series_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Priority")]
    pub priority: i32,
    #[serde(rename = "VoiceName")]
    pub voice_name: String,
    #[serde(rename = "ReleaseConditionText")]
    pub release_condition_text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubCharacters {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Label")]
    pub label: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabList {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "TabListName")]
    pub tab_list_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Targets {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "TargetType")]
    pub target_type: i32,
    #[serde(rename = "TargetValue")]
    pub target_value: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextsPlaceHolder {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Description")]
    pub description: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketOnlyGachas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "GachaTicketId")]
    pub gacha_ticket_id: i32,
    #[serde(rename = "TicketNum")]
    pub ticket_num: i32,
    #[serde(rename = "ConsectiveGachaTimes")]
    pub consective_gacha_times: i32,
    #[serde(rename = "GachaButtonType")]
    pub gacha_button_type: i32,
    #[serde(rename = "ButtonPopId")]
    pub button_pop_id: i32,
    #[serde(rename = "MiniBannerPopId")]
    pub mini_banner_pop_id: i32,
    #[serde(rename = "MiniBannerDispCondition")]
    pub mini_banner_disp_condition: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialDeckCards {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "TutorialDeckDatasId")]
    pub tutorial_deck_datas_id: i32,
    #[serde(rename = "SlotNo")]
    pub slot_no: i32,
    #[serde(rename = "CardDatasId")]
    pub card_datas_id: i32,
    #[serde(rename = "StyleLevel")]
    pub style_level: i32,
    #[serde(rename = "LimitBreakTimes")]
    pub limit_break_times: i32,
    #[serde(rename = "SpecialAppealLevel")]
    pub special_appeal_level: i32,
    #[serde(rename = "SkillLevel")]
    pub skill_level: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialDeckDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "DeckName")]
    pub deck_name: String,
    #[serde(rename = "DeckNo")]
    pub deck_no: i32,
    #[serde(rename = "GenerationsId")]
    pub generations_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialQuestAreas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Number")]
    pub number: i32,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "ImageType")]
    pub image_type: i32,
    #[serde(rename = "BgImageId")]
    pub bg_image_id: i32,
    #[serde(rename = "Next_TutorialQuestAreasId")]
    pub next__tutorial_quest_areas_id: i32,
    #[serde(rename = "GenerationsId")]
    pub generations_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialQuestStages {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "TutorialQuestAreasId")]
    pub tutorial_quest_areas_id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Hint")]
    pub hint: String,
    #[serde(rename = "MapNumber")]
    pub map_number: i32,
    #[serde(rename = "StageType")]
    pub stage_type: i32,
    #[serde(rename = "UseType")]
    pub use_type: i32,
    #[serde(rename = "Use_ItemsId")]
    pub use__items_id: i32,
    #[serde(rename = "UseNum")]
    pub use_num: i32,
    #[serde(rename = "LiveStagesId")]
    pub live_stages_id: i32,
    #[serde(rename = "QuestMusicsId")]
    pub quest_musics_id: i32,
    #[serde(rename = "LoveCorrectionValue")]
    pub love_correction_value: i32,
    #[serde(rename = "BonusVoltage")]
    pub bonus_voltage: i32,
    #[serde(rename = "BonusMental")]
    pub bonus_mental: i32,
    #[serde(rename = "BonusHeart")]
    pub bonus_heart: i32,
    #[serde(rename = "BonusLove")]
    pub bonus_love: i32,
    #[serde(rename = "FirstClearRewardSeriesId")]
    pub first_clear_reward_series_id: i64,
    #[serde(rename = "CompleteRewardSeriesId")]
    pub complete_reward_series_id: i64,
    #[serde(rename = "RandomDropRewardSeriesId")]
    pub random_drop_reward_series_id: i64,
    #[serde(rename = "Score1")]
    pub score1: i64,
    #[serde(rename = "Score2")]
    pub score2: i64,
    #[serde(rename = "Score3")]
    pub score3: i64,
    #[serde(rename = "StylePoint")]
    pub style_point: i32,
    #[serde(rename = "GainMusicExp")]
    pub gain_music_exp: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialRewardDatas {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "TutorialsId")]
    pub tutorials_id: i32,
    #[serde(rename = "RewardType")]
    pub reward_type: i32,
    #[serde(rename = "RewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "RewardNum")]
    pub reward_num: i32,
    #[serde(rename = "LifeTimeDay")]
    pub life_time_day: i32,
    #[serde(rename = "RewardTextId")]
    pub reward_text_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialSchoolIdolStageMovies {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tutorials {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "TutorialType")]
    pub tutorial_type: i32,
    #[serde(rename = "Step")]
    pub step: i32,
    #[serde(rename = "Description")]
    pub description: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitCharacters {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "UnitsId")]
    pub units_id: i32,
    #[serde(rename = "CharactersId")]
    pub characters_id: i32,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Units {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "UnitName")]
    pub unit_name: String,
    #[serde(rename = "OrderId")]
    pub order_id: i32,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveCharacters {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "SkeletonName")]
    pub skeleton_name: String,
    #[serde(rename = "ItemsIds")]
    pub items_ids: String,
    #[serde(rename = "PosesIds")]
    pub poses_ids: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveEventsEvol {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "ContentsType")]
    pub contents_type: i32,
    #[serde(rename = "LocationsId")]
    pub locations_id: i32,
    #[serde(rename = "CharactersIds")]
    pub characters_ids: String,
    #[serde(rename = "CostumesIds")]
    pub costumes_ids: String,
    #[serde(rename = "TimelinesIds")]
    pub timelines_ids: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveItems {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "BindBoneId")]
    pub bind_bone_id: i32,
    #[serde(rename = "PosesId")]
    pub poses_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveLocations {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "PropsIds")]
    pub props_ids: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveMovies {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Label")]
    pub label: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveMusic {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "MusicId")]
    pub music_id: i32,
    #[serde(rename = "HaveMusic")]
    pub have_music: i32,
    #[serde(rename = "HaveMotion")]
    pub have_motion: i32,
    #[serde(rename = "CharactersCount")]
    pub characters_count: i32,
    #[serde(rename = "CharactersIds")]
    pub characters_ids: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivePoses {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "HandSide")]
    pub hand_side: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveProps {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Label")]
    pub label: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveTimelinesEvol {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "MusicId")]
    pub music_id: i32,
    #[serde(rename = "LocationsId")]
    pub locations_id: i32,
    #[serde(rename = "FreeId")]
    pub free_id: i32,
    #[serde(rename = "NextId")]
    pub next_id: i64,
    #[serde(rename = "MovieIds")]
    pub movie_ids: String,
}
