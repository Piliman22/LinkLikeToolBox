use std::collections::HashMap;

pub fn get_master_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    
    map.insert("advalbums.tsv", "AdvAlbums");
    map.insert("advdatas.tsv", "AdvDatas");
    map.insert("advseries.tsv", "AdvSeries");
    map.insert("advstorydigestmovies.tsv", "AdvStoryDigestMovies");
    map.insert("beginnermissionbannerrewards.tsv", "BeginnerMissionBannerRewards");
    map.insert("beginnermissionshintimages.tsv", "BeginnerMissionsHintImages");
    map.insert("beginnermissionshint.tsv", "BeginnerMissionsHint");
    map.insert("birthdayrarebonuses.tsv", "BirthdayRareBonuses");
    map.insert("campaignaddrewardseries.tsv", "CampaignAddRewardSeries");
    map.insert("campaignaddrewards.tsv", "CampaignAddRewards");
    map.insert("campaign.tsv", "Campaign");
    map.insert("cardcoordinates.tsv", "CardCoordinates");
    map.insert("carddatas.tsv", "CardDatas");
    map.insert("cardduetvoice.tsv", "CardDuetVoice");
    map.insert("cardevolutionmaterials.tsv", "CardEvolutionMaterials");
    map.insert("cardgetmoviesettings.tsv", "CardGetMovieSettings");
    map.insert("cardlevels.tsv", "CardLevels");
    map.insert("cardlimitbreakmaterials.tsv", "CardLimitBreakMaterials");
    map.insert("cardrarities.tsv", "CardRarities");
    map.insert("cardseries.tsv", "CardSeries");
    map.insert("cardskilleffectdetailparams.tsv", "CardSkillEffectDetailParams");
    map.insert("cardskilleffectdetails.tsv", "CardSkillEffectDetails");
    map.insert("cardskilleffects.tsv", "CardSkillEffects");
    map.insert("cardskilllevelupmaterials.tsv", "CardSkillLevelUpMaterials");
    map.insert("cardskillmodes.tsv", "CardSkillModes");
    map.insert("cardskillseries.tsv", "CardSkillSeries");
    map.insert("cardskills.tsv", "CardSkills");
    map.insert("centerattributeeffects.tsv", "CenterAttributeEffects");
    map.insert("centerattributes.tsv", "CenterAttributes");
    map.insert("centerskillconditions.tsv", "CenterSkillConditions");
    map.insert("centerskilleffects.tsv", "CenterSkillEffects");
    map.insert("centerskills.tsv", "CenterSkills");
    map.insert("challengemodeeffectdetails.tsv", "ChallengeModeEffectDetails");
    map.insert("challengemodeeffects.tsv", "ChallengeModeEffects");
    map.insert("challengemodereleasecondition.tsv", "ChallengeModeReleaseCondition");
    map.insert("challengemodestages.tsv", "ChallengeModeStages");
    map.insert("characterfavoritegifts.tsv", "CharacterFavoriteGifts");
    map.insert("characters.tsv", "Characters");
    map.insert("comics.tsv", "Comics");
    map.insert("commonmissions.tsv", "CommonMissions");
    map.insert("contentguidances.tsv", "ContentGuidances");
    map.insert("contentsreleaseconditions.tsv", "ContentsReleaseConditions");
    map.insert("costumemodels.tsv", "CostumeModels");
    map.insert("costumes.tsv", "Costumes");
    map.insert("customcomplementmaterials.tsv", "CustomComplementMaterials");
    map.insert("dailylivereleaseconditions.tsv", "DailyLiveReleaseConditions");
    map.insert("dailyquestseries.tsv", "DailyQuestSeries");
    map.insert("dailyqueststages.tsv", "DailyQuestStages");
    map.insert("deckmemberpositions.tsv", "DeckMemberPositions");
    map.insert("difficultybgimages.tsv", "DifficultyBgImages");
    map.insert("downloadimages.tsv", "DownloadImages");
    map.insert("dreamlivereleaseconditions.tsv", "DreamLiveReleaseConditions");
    map.insert("dreamliveserieslist.tsv", "DreamLiveSeriesList");
    map.insert("dreamquestseries.tsv", "DreamQuestSeries");
    map.insert("dreamqueststages.tsv", "DreamQuestStages");
    map.insert("emojicategory.tsv", "EmojiCategory");
    map.insert("emojis.tsv", "Emojis");
    map.insert("eventloginbonuses.tsv", "EventLoginBonuses");
    map.insert("eventmissionachieverewards.tsv", "EventMissionAchieveRewards");
    map.insert("eventmissionrewards.tsv", "EventMissionRewards");
    map.insert("eventmissionseries.tsv", "EventMissionSeries");
    map.insert("eventmissions.tsv", "EventMissions");
    map.insert("exchangepointconvert.tsv", "ExchangePointConvert");
    map.insert("exchangepointrate.tsv", "ExchangePointRate");
    map.insert("flowerstandcolors.tsv", "FlowerStandColors");
    map.insert("flowerstandidolpictures.tsv", "FlowerStandIdolPictures");
    map.insert("flowerstandtypes.tsv", "FlowerStandTypes");
    map.insert("gachacampaigns.tsv", "GachaCampaigns");
    map.insert("gachaseries.tsv", "GachaSeries");
    map.insert("generations.tsv", "Generations");
    map.insert("giftbonusgachas.tsv", "GiftBonusGachas");
    map.insert("giftlessgachas.tsv", "GiftlessGachas");
    map.insert("gpprizeexchanges.tsv", "GpPrizeExchanges");
    map.insert("gradeaddskilleffectdetails.tsv", "GradeAddSkillEffectDetails");
    map.insert("gradeaddskilleffects.tsv", "GradeAddSkillEffects");
    map.insert("gradeaddskills.tsv", "GradeAddSkills");
    map.insert("gradechalqueststagerewarddatas.tsv", "GradeChalQuestStageRewardDatas");
    map.insert("gradechalqueststages.tsv", "GradeChalQuestStages");
    map.insert("gradechalqueststagesrewards.tsv", "GradeChalQuestStagesRewards");
    map.insert("gradechalseason.tsv", "GradeChalSeason");
    map.insert("gradechaltotalscorerewarddatas.tsv", "GradeChalTotalScoreRewardDatas");
    map.insert("gradechaltotalscorerewards.tsv", "GradeChalTotalScoreRewards");
    map.insert("gradedatas.tsv", "GradeDatas");
    map.insert("gradequestlivepointbonus.tsv", "GradeQuestLivePointBonus");
    map.insert("gradequestrewardsdatas.tsv", "GradeQuestRewardsDatas");
    map.insert("gradequestrewards.tsv", "GradeQuestRewards");
    map.insert("gradequestseason.tsv", "GradeQuestSeason");
    map.insert("gradequestseasonreleasecond.tsv", "GradeQuestSeasonReleaseCond");
    map.insert("gradequeststagescaseries.tsv", "GradeQuestSeries");
    map.insert("gradequestseriesreleasecond.tsv", "GradeQuestSeriesReleaseCond");
    map.insert("gradequestsquaredatas.tsv", "GradeQuestSquareDatas");
    map.insert("gradequestsquare.tsv", "GradeQuestSquare");
    map.insert("gradequeststages.tsv", "GradeQuestStages");
    map.insert("grade.tsv", "Grade");
    map.insert("graderewarddatas.tsv", "GradeRewardDatas");
    map.insert("graderewards.tsv", "GradeRewards");
    map.insert("grandprixdailypoints.tsv", "GrandPrixDailyPoints");
    map.insert("grandprixpointbonuses.tsv", "GrandPrixPointBonuses");
    map.insert("grandprixquestseries.tsv", "GrandPrixQuestSeries");
    map.insert("grandprixqueststages.tsv", "GrandPrixQuestStages");
    map.insert("grandprix.tsv", "GrandPrix");
    map.insert("grandprixreleasecondition.tsv", "GrandPrixReleaseCondition");
    map.insert("grandprixrewarddatas.tsv", "GrandPrixRewardDatas");
    map.insert("grandprixrewards.tsv", "GrandPrixRewards");
    map.insert("helpimages.tsv", "HelpImages");
    map.insert("homebgms.tsv", "HomeBgms");
    map.insert("ingamemissionskilldetails.tsv", "IngameMissionSkillDetails");
    map.insert("itemexchangecategorydatas.tsv", "ItemExchangeCategoryDatas");
    map.insert("itemexchanges.tsv", "ItemExchanges");
    map.insert("itemsources.tsv", "ItemSources");
    map.insert("items.tsv", "Items");
    map.insert("launcherbanners.tsv", "LauncherBanners");
    map.insert("learninglivereleaseconditions.tsv", "LearningLiveReleaseConditions");
    map.insert("levelupmaterialdetails.tsv", "LevelUpMaterialDetails");
    map.insert("levelupmaterials.tsv", "LevelUpMaterials");
    map.insert("limitbreakmaterialconvertrate.tsv", "LimitBreakMaterialConvertRate");
    map.insert("limitbreakmaterialrate.tsv", "LimitBreakMaterialRate");
    map.insert("livechannels.tsv", "LiveChannels");
    map.insert("livestages.tsv", "LiveStages");
    map.insert("loginbonuses.tsv", "LoginBonuses");
    map.insert("loginbonusrewarddatas.tsv", "LoginBonusRewardDatas");
    map.insert("memberfanlevels.tsv", "MemberFanLevels");
    map.insert("membermovies.tsv", "MemberMovies");
    map.insert("membervoices.tsv", "MemberVoices");
    map.insert("missionachieverewards.tsv", "MissionAchieveRewards");
    map.insert("missionrewards.tsv", "MissionRewards");
    map.insert("missions.tsv", "Missions");
    map.insert("musicdroprewarddetails.tsv", "MusicDropRewardDetails");
    map.insert("musicdroprewards.tsv", "MusicDropRewards");
    map.insert("musiclearningquestseries.tsv", "MusicLearningQuestSeries");
    map.insert("musiclearningqueststages.tsv", "MusicLearningQuestStages");
    map.insert("musiclevels.tsv", "MusicLevels");
    map.insert("musicmasteryheartbonuses.tsv", "MusicMasteryHeartBonuses");
    map.insert("musicmasterylevels.tsv", "MusicMasteryLevels");
    map.insert("musicmasterylovebonuses.tsv", "MusicMasteryLoveBonuses");
    map.insert("musicmasterymentalbonuses.tsv", "MusicMasteryMentalBonuses");
    map.insert("musicmasteryskill.tsv", "MusicMasterySkill");
    map.insert("musicmasteryvoltagebonuses.tsv", "MusicMasteryVoltageBonuses");
    map.insert("musicscorerewarddatas.tsv", "MusicScoreRewardDatas");
    map.insert("musicscorerewards.tsv", "MusicScoreRewards");
    map.insert("musicscores.tsv", "MusicScores");
    map.insert("musics.tsv", "Musics");
    map.insert("petalcoinexchangerate.tsv", "PetalCoinExchangeRate");
    map.insert("petalexchangerates.tsv", "PetalExchangeRates");
    map.insert("presenttexts.tsv", "PresentTexts");
    map.insert("questareareleaseconditions.tsv", "QuestAreaReleaseConditions");
    map.insert("questlivedownloads.tsv", "QuestLiveDownloads");
    map.insert("questliveloadings.tsv", "QuestLiveLoadings");
    map.insert("questlivereleaseconditions.tsv", "QuestLiveReleaseConditions");
    map.insert("questsections.tsv", "QuestSections");
    map.insert("raidevents.tsv", "RaidEvents");
    map.insert("raidquestdroprateup.tsv", "RaidQuestDropRateUp");
    map.insert("raidquestreleasecondition.tsv", "RaidQuestReleaseCondition");
    map.insert("raidquestseries.tsv", "RaidQuestSeries");
    map.insert("raidqueststages.tsv", "RaidQuestStages");
    map.insert("raidresourceadddate.tsv", "RaidResourceAddDate");
    map.insert("raidresource.tsv", "RaidResource");
    map.insert("raidresourcerecoverydatas.tsv", "RaidResourceRecoveryDatas");
    map.insert("raidrewarddatas.tsv", "RaidRewardDatas");
    map.insert("raidrewards.tsv", "RaidRewards");
    map.insert("raidtopprogressimage.tsv", "RaidTopProgressImage");
    map.insert("rentalcarddatas.tsv", "RentalCardDatas");
    map.insert("rentaldeckcards.tsv", "RentalDeckCards");
    map.insert("rentaldecks.tsv", "RentalDecks");
    map.insert("rhythmgameclassdatas.tsv", "RhythmGameClassDatas");
    map.insert("rhythmgameclasses.tsv", "RhythmGameClasses");
    map.insert("rhythmgameclassmissionrewards.tsv", "RhythmGameClassMissionRewards");
    map.insert("rhythmgamehelpimages.tsv", "RhythmGameHelpImages");
    map.insert("rhythmgameskillconditions.tsv", "RhythmGameSkillConditions");
    map.insert("rhythmgameskilleffects.tsv", "RhythmGameSkillEffects");
    map.insert("rhythmgameskilllvupitemdetails.tsv", "RhythmGameSkillLvUpItemDetails");
    map.insert("rhythmgameskilllvupitems.tsv", "RhythmGameSkillLvUpItems");
    map.insert("rhythmgameskills.tsv", "RhythmGameSkills");
    map.insert("rhythmgametotalmissionrewards.tsv", "RhythmGameTotalMissionRewards");
    map.insert("rhythmgametotalmissions.tsv", "RhythmGameTotalMissions");
    map.insert("seasonfanlevels.tsv", "SeasonFanLevels");
    map.insert("seasongrade.tsv", "SeasonGrade");
    map.insert("seasongraderewarddatas.tsv", "SeasonGradeRewardDatas");
    map.insert("seasongraderewards.tsv", "SeasonGradeRewards");
    map.insert("seasons.tsv", "Seasons");
    map.insert("sectionskilleffectdetails.tsv", "SectionSkillEffectDetails");
    map.insert("sectionskilleffects.tsv", "SectionSkillEffects");
    map.insert("sectionskills.tsv", "SectionSkills");
    map.insert("selectticketexchangerate.tsv", "SelectTicketExchangeRate");
    map.insert("selectticketseries.tsv", "SelectTicketSeries");
    map.insert("shopitems.tsv", "ShopItems");
    map.insert("shops.tsv", "Shops");
    map.insert("sidestylesettings.tsv", "SideStyleSettings");
    map.insert("simulationgraphlimit.tsv", "SimulationGraphLimit");
    map.insert("stagescoremultipliersettings.tsv", "StageScoreMultiplierSettings");
    map.insert("stageskillconditiondetails.tsv", "StageSkillConditionDetails");
    map.insert("stageskillconditions.tsv", "StageSkillConditions");
    map.insert("stageskilleffectdetails.tsv", "StageSkillEffectDetails");
    map.insert("stageskilleffects.tsv", "StageSkillEffects");
    map.insert("stageskillsets.tsv", "StageSkillSets");
    map.insert("stamps.tsv", "Stamps");
    map.insert("standardquestareas.tsv", "StandardQuestAreas");
    map.insert("standardqueststages.tsv", "StandardQuestStages");
    map.insert("stickerexchanges.tsv", "StickerExchanges");
    map.insert("stickers.tsv", "Stickers");
    map.insert("stylemovies.tsv", "StyleMovies");
    map.insert("stylevoices.tsv", "StyleVoices");
    map.insert("subcharacters.tsv", "SubCharacters");
    map.insert("tablist.tsv", "TabList");
    map.insert("targets.tsv", "Targets");
    map.insert("textsplaceholder.tsv", "TextsPlaceHolder");
    map.insert("ticketonlygachas.tsv", "TicketOnlyGachas");
    map.insert("tutorialdeckcards.tsv", "TutorialDeckCards");
    map.insert("tutorialdeckdatas.tsv", "TutorialDeckDatas");
    map.insert("tutorialquestareas.tsv", "TutorialQuestAreas");
    map.insert("tutorialqueststages.tsv", "TutorialQuestStages");
    map.insert("tutorialrewarddatas.tsv", "TutorialRewardDatas");
    map.insert("tutorialschoolidolstagemovies.tsv", "TutorialSchoolIdolStageMovies");
    map.insert("tutorials.tsv", "Tutorials");
    map.insert("unitcharacters.tsv", "UnitCharacters");
    map.insert("units.tsv", "Units");
    map.insert("livecharacters.tsv", "LiveCharacters");
    map.insert("liveeventsevol.tsv", "LiveEventsEvol");
    map.insert("liveitems.tsv", "LiveItems");
    map.insert("livelocations.tsv", "LiveLocations");
    map.insert("livemovies.tsv", "LiveMovies");
    map.insert("livemusic.tsv", "LiveMusic");
    map.insert("liveposes.tsv", "LivePoses");
    map.insert("liveprops.tsv", "LiveProps");
    map.insert("livetimeslinesevol.tsv", "LiveTimelinesEvol");
    
    map
}

pub fn get_field_mapping(tsv_label: &str) -> Option<Vec<String>> {
    match tsv_label {
        "advalbums.tsv" => Some(vec![
            "Id".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "advdatas.tsv" => Some(vec![
            "Id".to_string(),
            "AdvSeriesId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "ScriptId".to_string(),
            "OpenSeasonFanLevel".to_string(),
            "RewardType".to_string(),
            "WatchRewardId".to_string(),
            "WatchRewardNum".to_string(),
            "RewardTextId".to_string(),
            "OrderId".to_string(),
            "SubTitleName".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "IsPeriod".to_string(),
        ]),
        "advseries.tsv" => Some(vec![
            "Id".to_string(),
            "SeasonsId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "FiscalYearDisplay".to_string(),
            "AdvAlbumId".to_string(),
        ]),
        "advstorydigestmovies.tsv" => Some(vec![
            "Id".to_string(),
            "Title".to_string(),
            "OrderId".to_string(),
            "DigestFiscalYearDisplay".to_string(),
        ]),
        "beginnermissionbannerrewards.tsv" => Some(vec![
            "Id".to_string(),
            "MissionType".to_string(),
            "ItemType".to_string(),
            "ItemId".to_string(),
            "Num".to_string(),
        ]),
        "beginnermissionshint.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
        ]),
        "beginnermissionshintimages.tsv" => Some(vec![
            "Id".to_string(),
            "BeginnerMissionsHintId".to_string(),
            "ImageId".to_string(),
        ]),
        "birthdayrarebonuses.tsv" => Some(vec![
            "Id".to_string(),
            "CardSeriesId".to_string(),
            "SkillName".to_string(),
            "LimitBreakTimes".to_string(),
            "MentalBonus".to_string(),
            "VoltageBonus".to_string(),
            "HeartBonus".to_string(),
            "LoveBonus".to_string(),
        ]),
        "campaign.tsv" => Some(vec![
            "Id".to_string(),
            "CampaignType".to_string(),
            "TargetContents".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "EffectValue".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "campaignaddrewards.tsv" => Some(vec![
            "Id".to_string(),
            "AddRewardOddsId".to_string(),
            "RewardType".to_string(),
            "AddRewardItemId".to_string(),
            "AddRewardItemQuantity".to_string(),
            "AddRewardItemOdds".to_string(),
            "AddRewardItemOddsSum".to_string(),
        ]),
        "campaignaddrewardseries.tsv" => Some(vec![
            "Id".to_string(),
            "AddRewardOddsId_1".to_string(),
            "AddRewardOddsId_2".to_string(),
            "AddRewardOddsId_3".to_string(),
        ]),
        "cardcoordinates.tsv" => Some(vec![
            "Id".to_string(),
            "CardDatasId".to_string(),
            "CardCoordSceneType".to_string(),
            "XCoord".to_string(),
            "YCoord".to_string(),
            "Scale".to_string(),
        ]),
        "carddatas.tsv" => Some(vec![
            "Id".to_string(),
            "CardSeriesId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "CharactersId".to_string(),
            "Rarity".to_string(),
            "EvolveTimes".to_string(),
            "CardLevelLimitAddition".to_string(),
            "Style".to_string(),
            "Mood".to_string(),
            "ExperienceType".to_string(),
            "InitialSmile".to_string(),
            "InitialPure".to_string(),
            "InitialCool".to_string(),
            "InitialMental".to_string(),
            "MaxSmile".to_string(),
            "MaxPure".to_string(),
            "MaxCool".to_string(),
            "MaxMental".to_string(),
            "BeatPoint".to_string(),
            "SpecialAppealSeriesId".to_string(),
            "SkillSeriesId".to_string(),
            "AttributeId".to_string(),
            "SpineId".to_string(),
            "OrderId".to_string(),
            "CenterSkillSeriesId".to_string(),
            "CenterAttributeSeriesId".to_string(),
            "RhythmGameSkillSeriesId".to_string(),
            "CenterSkillLvUpItemId".to_string(),
            "RhythmGameSkillLvUpItemId".to_string(),
        ]),
        "cardduetvoice.tsv" => Some(vec![
            "Id".to_string(),
            "CardSeriesId".to_string(),
            "CharacterIds".to_string(),
        ]),
        "cardevolutionmaterials.tsv" => Some(vec![
            "Id".to_string(),
            "CostItemsId1".to_string(),
            "CostNum1".to_string(),
            "CostItemsId2".to_string(),
            "CostNum2".to_string(),
            "CostItemsId3".to_string(),
            "CostNum3".to_string(),
        ]),
        "cardgetmoviesettings.tsv" => Some(vec![
            "Id".to_string(),
            "CardInfoPositionType".to_string(),
            "CardInfoDisplayStartTimeSeconds".to_string(),
            "UrCardEffectBackgroundId".to_string(),
        ]),
        "cardlevels.tsv" => Some(vec![
            "Id".to_string(),
            "ExperienceType".to_string(),
            "CardLevel".to_string(),
            "Experience".to_string(),
            "CumulativeExperience".to_string(),
        ]),
        "cardlimitbreakmaterials.tsv" => Some(vec![
            "Id".to_string(),
            "CardSeriesId".to_string(),
            "LimitBreakTimes".to_string(),
            "CostItemsId".to_string(),
            "CostNum".to_string(),
        ]),
        "cardrarities.tsv" => Some(vec![
            "Id".to_string(),
            "RarityName".to_string(),
            "Evolution0_MaxLevel".to_string(),
            "Evolution1_MaxLevel".to_string(),
            "Evolution2_MaxLevel".to_string(),
            "Evolution3_MaxLevel".to_string(),
            "Evolution4_MaxLevel".to_string(),
        ]),
        "cardseries.tsv" => Some(vec![
            "Id".to_string(),
            "Evolution0Id".to_string(),
            "Evolution1Id".to_string(),
            "Evolution2Id".to_string(),
            "ObtainFanLvPt".to_string(),
            "Evolution1FanLvPt".to_string(),
            "Evolution2FanLvPt".to_string(),
            "LimitBreak1FanLvPt".to_string(),
            "LimitBreak2FanLvPt".to_string(),
            "LimitBreak3FanLvPt".to_string(),
            "LimitBreak4FanLvPt".to_string(),
            "LimitedType".to_string(),
            "SideStyleSettingCharacterId".to_string(),
            "Evolution3Id".to_string(),
            "Evolution4Id".to_string(),
            "Evolution3FanLvPt".to_string(),
            "Evolution4FanLvPt".to_string(),
            "Evolution3RequiredFanLv".to_string(),
            "Evolution4RequiredFanLv".to_string(),
        ]),
        "cardskilleffectdetailparams.tsv" => Some(vec![
            "Id".to_string(),
            "ParamType".to_string(),
            "ParamValue".to_string(),
        ]),
        "cardskilleffectdetails.tsv" => Some(vec![
            "Id".to_string(),
            "SkillEffectDetailType".to_string(),
            "TargetMood".to_string(),
            "EffectValue".to_string(),
        ]),
        "cardskilleffects.tsv" => Some(vec![
            "Id".to_string(),
            "ActionType".to_string(),
            "OrderId".to_string(),
        ]),
        "cardskilllevelupmaterials.tsv" => Some(vec![
            "Id".to_string(),
            "CardSeriesId".to_string(),
            "SkillType".to_string(),
            "SkillLevel".to_string(),
            "Cost_ItemsId1".to_string(),
            "CostNum1".to_string(),
            "Cost_ItemsId2".to_string(),
            "CostNum2".to_string(),
            "Cost_ItemsId3".to_string(),
            "CostNum3".to_string(),
            "Cost_ItemsIds".to_string(),
            "CostNums".to_string(),
        ]),
        "cardskillmodes.tsv" => Some(vec![
            "Id".to_string(),
            "ModeOffName".to_string(),
            "ModeOnName".to_string(),
            "CharactersId".to_string(),
        ]),
        "cardskills.tsv" => Some(vec![
            "Id".to_string(),
            "CardSkillSeriesId".to_string(),
            "SkillLevel".to_string(),
            "SkillCost".to_string(),
            "ApperanceType".to_string(),
            "CardSkillEffectId".to_string(),
            "Description".to_string(),
        ]),
        "cardskillseries.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "SkillIcon".to_string(),
            "SkillMainEffect".to_string(),
        ]),
        "centerattributeeffects.tsv" => Some(vec![
            "Id".to_string(),
            "CenterAttributeEffectType".to_string(),
            "CenterAttributeEffectValue".to_string(),
        ]),
        "centerattributes.tsv" => Some(vec![
            "Id".to_string(),
            "CenterAttributeSeriesId".to_string(),
            "CenterAttributeName".to_string(),
            "TargetIds".to_string(),
            "CenterAttributeEffectId".to_string(),
            "Description".to_string(),
        ]),
        "centerskillconditions.tsv" => Some(vec![
            "Id".to_string(),
            "CenterSkillConditionType".to_string(),
            "CenterSkillConditionValue".to_string(),
            "CenterSkillConditionValue2".to_string(),
        ]),
        "centerskilleffects.tsv" => Some(vec![
            "Id".to_string(),
            "CenterSkillEffectType".to_string(),
            "CenterSkillEffectValue".to_string(),
        ]),
        "centerskills.tsv" => Some(vec![
            "Id".to_string(),
            "CenterSkillSeriesId".to_string(),
            "CenterSkillName".to_string(),
            "SkillLevel".to_string(),
            "OrderId".to_string(),
            "CenterSkillConditionIds".to_string(),
            "CenterSkillEffectId".to_string(),
            "Description".to_string(),
        ]),
        "challengemodeeffectdetails.tsv" => Some(vec![
            "Id".to_string(),
            "EffectDetailType".to_string(),
            "TargetMood".to_string(),
            "EffectValue".to_string(),
        ]),
        "challengemodeeffects.tsv" => Some(vec![
            "Id".to_string(),
            "StandardQuestStagesId".to_string(),
            "ActionType".to_string(),
            "OrderId".to_string(),
            "Description".to_string(),
        ]),
        "challengemodereleasecondition.tsv" => Some(vec![
            "Id".to_string(),
            "ReleaseChallengeModeId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "challengemodestages.tsv" => Some(vec![
            "Id".to_string(),
            "ChallengeModeAreasId".to_string(),
            "CorrespondedQuestStageId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "Hint".to_string(),
            "MapNumber".to_string(),
            "StageType".to_string(),
            "UseType".to_string(),
            "UseItem".to_string(),
            "UseNum".to_string(),
            "LiveStagesId".to_string(),
            "QuestMusicsType".to_string(),
            "QuestMusicsDetail".to_string(),
            "DeckRestrictedType".to_string(),
            "DeckRestrictedDetail".to_string(),
            "ChallengeModeEffectId".to_string(),
            "QuestLevel".to_string(),
            "FirstClearRewardSeriesId".to_string(),
            "ChallengeModeScore".to_string(),
        ]),
        "characterfavoritegifts.tsv" => Some(vec![
            "Id".to_string(),
            "ItemsId".to_string(),
            "CharactersId".to_string(),
            "FavoriteRank".to_string(),
        ]),
        "characters.tsv" => Some(vec![
            "Id".to_string(),
            "NameLast".to_string(),
            "NameFirst".to_string(),
            "LatinAlphabetNameLast".to_string(),
            "LatinAlphabetNameFirst".to_string(),
            "GenerationsId".to_string(),
            "SeriesType".to_string(),
            "IconOrderId".to_string(),
            "CharacterVoice".to_string(),
            "ThemeColor".to_string(),
            "Introduction".to_string(),
            "ShowSeasonFanLvStartTime".to_string(),
            "ShowSeasonFanLvEndTime".to_string(),
            "IsExistFanLv".to_string(),
            "StyleType".to_string(),
            "PrintFilterType".to_string(),
            "DisplayFullName".to_string(),
            "LatinAlpabetFullName".to_string(),
            "NameDisplayType".to_string(),
            "DisplayGeneration".to_string(),
            "GraduateIntroduction".to_string(),
        ]),
        "comics.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "ViewType".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "TabListId".to_string(),
            "AppearanceCharacterIds".to_string(),
        ]),
        "commonmissions.tsv" => Some(vec![
            "Id".to_string(),
            "ClearConditionNum".to_string(),
            "RewardType".to_string(),
            "ItemsId".to_string(),
            "RewardNum".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "contentguidances.tsv" => Some(vec![
            "Id".to_string(),
            "ContentId".to_string(),
            "Priority".to_string(),
            "IsSkip".to_string(),
            "IsEnable".to_string(),
            "ConditionValues".to_string(),
        ]),
        "contentsreleaseconditions.tsv" => Some(vec![
            "Id".to_string(),
            "ReleaseContentsId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "costumemodels.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
            "CharactersId".to_string(),
            "CostumesId".to_string(),
            "HairStyleId".to_string(),
        ]),
        "costumes.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
        ]),
        "customcomplementmaterials.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "dailylivereleaseconditions.tsv" => Some(vec![
            "Id".to_string(),
            "ReleaseDailyLivesId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "dailyquestseries.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "IsSunday".to_string(),
            "IsMonday".to_string(),
            "IsTuesday".to_string(),
            "IsWednesday".to_string(),
            "IsThursday".to_string(),
            "IsFriday".to_string(),
            "IsSaturday".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "dailyqueststages.tsv" => Some(vec![
            "Id".to_string(),
            "DailyQuestSeriesId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "Hint".to_string(),
            "StageType".to_string(),
            "UseType".to_string(),
            "UseItem".to_string(),
            "UseNum".to_string(),
            "LiveStagesId".to_string(),
            "QuestMusicsType".to_string(),
            "QuestMusicsDetail".to_string(),
            "DeckRestrictedType".to_string(),
            "DeckRestrictedDetail".to_string(),
            "QuestLevel".to_string(),
            "QuestRank".to_string(),
            "FirstClearRewardSeriesId".to_string(),
            "CompleteRewardSeriesId".to_string(),
            "DropRewardSeriesId".to_string(),
            "RandomDropRewardSeriesId".to_string(),
            "Score1".to_string(),
            "Score2".to_string(),
            "Score3".to_string(),
            "GainStylePoint".to_string(),
            "GainMusicExp".to_string(),
            "ItemSourceIconId".to_string(),
        ]),
        "deckmemberpositions.tsv" => Some(vec![
            "Id".to_string(),
            "GenerationsId".to_string(),
            "CharactersId".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "difficultybgimages.tsv" => Some(vec![
            "Id".to_string(),
            "MusicDifficulty".to_string(),
            "OutGameBgImageId".to_string(),
            "InGameBgImageId".to_string(),
        ]),
        "downloadimages.tsv" => Some(vec![
            "Id".to_string(),
            "DownloadType".to_string(),
            "Title".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "dreamlivereleaseconditions.tsv" => Some(vec![
            "Id".to_string(),
            "ReleaseDreamLiveId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "dreamliveserieslist.tsv" => Some(vec![
            "Id".to_string(),
            "DisplayPosition".to_string(),
            "ImageId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "dreamquestseries.tsv" => Some(vec![
            "Id".to_string(),
            "CharactersId".to_string(),
            "Name".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "DreamLiveSeriesListId".to_string(),
            "ReleaseStartTime".to_string(),
            "MessageTextId".to_string(),
        ]),
        "dreamqueststages.tsv" => Some(vec![
            "Id".to_string(),
            "DreamQuestSeriesId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "Hint".to_string(),
            "StageType".to_string(),
            "UseType".to_string(),
            "UseItem".to_string(),
            "UseNum".to_string(),
            "LiveStagesId".to_string(),
            "QuestMusicsType".to_string(),
            "QuestMusicsDetail".to_string(),
            "DeckRestrictedType".to_string(),
            "DeckRestrictedDetail".to_string(),
            "FirstClearRewardSeriesId".to_string(),
            "DropRewardSeriesId".to_string(),
            "RandomDropRewardSeriesId".to_string(),
            "Score1".to_string(),
        ]),
        "emojicategory.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "emojis.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Alias".to_string(),
            "Category".to_string(),
            "OrderId".to_string(),
            "RequirementType".to_string(),
            "RequirementDetail".to_string(),
            "RequirementValue".to_string(),
            "RequirementText".to_string(),
            "IsVisibleOnlyPossess".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "AvailableStartTime".to_string(),
            "AvailableEndTime".to_string(),
        ]),
        "eventloginbonuses.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "LoginBonusRewardSeriesId".to_string(),
            "EventLoginBonusType".to_string(),
            "LoginBonusTextId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "eventmissionachieverewards.tsv" => Some(vec![
            "Id".to_string(),
            "EvemtMissionSeriesId".to_string(),
            "AchieveMarkNum".to_string(),
            "RewardCategory".to_string(),
            "RewardType".to_string(),
            "ItemsId".to_string(),
            "RewardNum".to_string(),
            "RewardTextId".to_string(),
            "SortOrder".to_string(),
        ]),
        "eventmissionrewards.tsv" => Some(vec![
            "Id".to_string(),
            "EventMissionSeriesId".to_string(),
            "EventMissionsId".to_string(),
            "RewardCategory".to_string(),
            "RewardType".to_string(),
            "ItemsId".to_string(),
            "RewardNum".to_string(),
            "RewardTextId".to_string(),
            "SortOrder".to_string(),
        ]),
        "eventmissions.tsv" => Some(vec![
            "Id".to_string(),
            "EventMissionSeriesId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "MissionType".to_string(),
            "MissionCondition".to_string(),
            "MissionConditionNum".to_string(),
            "OpenType".to_string(),
            "NextMissionsId".to_string(),
            "SortOrder".to_string(),
            "TransitionContentsId".to_string(),
            "MissionConditionDetail".to_string(),
        ]),
        "eventmissionseries.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "GrandPrixesId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "exchangepointconvert.tsv" => Some(vec![
            "Id".to_string(),
            "ConvertItemType".to_string(),
            "ConvertItemId".to_string(),
            "ConvertItemQuantity".to_string(),
            "ConvertTime".to_string(),
        ]),
        "exchangepointrate.tsv" => Some(vec![
            "Id".to_string(),
            "ExchangePointId".to_string(),
            "ExchangeItemType".to_string(),
            "ExchangeItemId".to_string(),
            "ExchangeItemQuantity".to_string(),
            "ExchangePrice".to_string(),
            "LimitedCount".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "GachaSeriesId".to_string(),
            "BonusItemQuantity".to_string(),
        ]),
        "flowerstandcolors.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "ColorCode".to_string(),
            "OrderId".to_string(),
        ]),
        "flowerstandidolpictures.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "CharactersId".to_string(),
        ]),
        "flowerstandtypes.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
        ]),
        "gachacampaigns.tsv" => Some(vec![
            "Id".to_string(),
            "CampaignName".to_string(),
            "CampaignType".to_string(),
            "ConsectiveTimesType".to_string(),
            "ResetType".to_string(),
            "PerDayCampaignTimes".to_string(),
            "GachaSeriesId_1".to_string(),
            "GachaSeriesId_2".to_string(),
            "GachaSeriesId_3".to_string(),
            "GachaSeriesId_4".to_string(),
            "GachaSeriesId_5".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "MiniBannerPopId".to_string(),
        ]),
        "gachaseries.tsv" => Some(vec![
            "Id".to_string(),
            "GachaSeriesName".to_string(),
            "Description".to_string(),
            "GachaType".to_string(),
            "LimitedGachaCount".to_string(),
            "LimitedGachaResetType".to_string(),
            "GachaExchangePointId".to_string(),
            "ExchangePointNoticeNum".to_string(),
            "ExchangePointLockFlag".to_string(),
            "OrderId".to_string(),
            "FilterType".to_string(),
            "PickUpCardSeriesId_1".to_string(),
            "PickUpCardBonusItemQuantity_1".to_string(),
            "PickUpCardSeriesId_2".to_string(),
            "PickUpCardBonusItemQuantity_2".to_string(),
            "PickUpCardSeriesId_3".to_string(),
            "PickUpCardBonusItemQuantity_3".to_string(),
            "PickUpCardSeriesId_4".to_string(),
            "PickUpCardBonusItemQuantity_4".to_string(),
            "PickUpCardSeriesId_5".to_string(),
            "PickUpCardBonusItemQuantity_5".to_string(),
            "PickUpCardSeriesId_6".to_string(),
            "PickUpCardBonusItemQuantity_6".to_string(),
            "BgType".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "NoticeText".to_string(),
            "GachaTimeLimitType".to_string(),
            "AvailableTime".to_string(),
            "GachaStartBgm".to_string(),
        ]),
        "generations.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "giftbonusgachas.tsv" => Some(vec![
            "Id".to_string(),
            "SingleGachaPrice".to_string(),
            "SingleGachaPopId".to_string(),
            "ConsectiveGachaPrice".to_string(),
            "ConsectiveGachaTimes".to_string(),
            "ConsectiveGachaPopId".to_string(),
            "PaidSIsCaOnlyGachaFlag".to_string(),
            "PaidSIsCaOnlyGachaPrice".to_string(),
            "PaidSIsCaOnlyGachaTimes".to_string(),
            "PaidSIsCaOnlyGachaPointFlag".to_string(),
            "PaidSIsCaOnlyGachaaPopId".to_string(),
            "LimitedPaidGachaCount".to_string(),
            "LimitedPaidGachaResetType".to_string(),
            "LimitedPaidButtonDesignType".to_string(),
            "MiniBannerPopId".to_string(),
        ]),
        "giftlessgachas.tsv" => Some(vec![
            "Id".to_string(),
            "SingleGachaPrice".to_string(),
            "SingleGachaPopId".to_string(),
            "ConsectiveGachaPrice".to_string(),
            "ConsectiveGachaTimes".to_string(),
            "ConsectiveGachaPopId".to_string(),
            "PaidSIsCaOnlyGachaFlag".to_string(),
            "PaidSIsCaOnlyGachaPrice".to_string(),
            "PaidSIsCaOnlyGachaTimes".to_string(),
            "PaidSIsCaOnlyGachaPointFlag".to_string(),
            "PaidSIsCaOnlyGachaaPopId".to_string(),
            "LimitedPaidGachaCount".to_string(),
            "LimitedPaidGachaResetType".to_string(),
            "LimitedPaidButtonDesignType".to_string(),
            "MiniBannerPopId".to_string(),
        ]),
        "gpprizeexchanges.tsv" => Some(vec![
            "Id".to_string(),
            "ProductItemType".to_string(),
            "ProductItemId".to_string(),
            "ProductItemNum".to_string(),
            "MaterialItemType".to_string(),
            "MaterialItemId".to_string(),
            "MaterialItemNum".to_string(),
            "LimitNum".to_string(),
            "ResetType".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "grade.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "MaxGradeTier".to_string(),
        ]),
        "gradeaddskilleffectdetails.tsv" => Some(vec![
            "Id".to_string(),
            "SkillEffectDetailType".to_string(),
            "TargetMood".to_string(),
            "EffectValue".to_string(),
        ]),
        "gradeaddskilleffects.tsv" => Some(vec![
            "Id".to_string(),
            "ActionType".to_string(),
            "OrderId".to_string(),
        ]),
        "gradeaddskills.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "SkillIcon".to_string(),
            "GradeAddSkillEffectsId".to_string(),
            "OrderId".to_string(),
        ]),
        "gradechalqueststagerewarddatas.tsv" => Some(vec![
            "Id".to_string(),
            "GradeChalQuestStagesRewardsId".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
            "OrderId".to_string(),
        ]),
        "gradechalqueststages.tsv" => Some(vec![
            "Id".to_string(),
            "GradeChalSeasonId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "Hint".to_string(),
            "CharacterId".to_string(),
            "OrderId".to_string(),
            "StageType".to_string(),
            "StageIconId".to_string(),
            "LiveStagesId".to_string(),
            "QuestMusicsType".to_string(),
            "QuestMusicsDetail".to_string(),
            "DeckRestrictedType".to_string(),
            "DeckRestrictedDetail".to_string(),
        ]),
        "gradechalqueststagesrewards.tsv" => Some(vec![
            "Id".to_string(),
            "GradeChalQuestStagesId".to_string(),
            "OrderId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
        ]),
        "gradechalseason.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "GiveSeasonGradeId".to_string(),
            "BgImageId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "gradechaltotalscorerewarddatas.tsv" => Some(vec![
            "Id".to_string(),
            "GradeChalTotalScoreRewardsId".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
            "OrderId".to_string(),
        ]),
        "gradechaltotalscorerewards.tsv" => Some(vec![
            "Id".to_string(),
            "GradeChalSeasonId".to_string(),
            "TotalScore".to_string(),
            "OrderId".to_string(),
        ]),
        "gradedatas.tsv" => Some(vec![
            "Id".to_string(),
            "GradeNum".to_string(),
            "GradeId".to_string(),
            "GradeTier".to_string(),
            "RequireGradePoints".to_string(),
            "GradeLiveBonus".to_string(),
        ]),
        "gradequestlivepointbonus.tsv" => Some(vec![
            "Id".to_string(),
            "GradeQuestSeriesId".to_string(),
            "OrderId".to_string(),
            "Description".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "BonusNum".to_string(),
            "BonusLimitUp".to_string(),
        ]),
        "gradequestrewards.tsv" => Some(vec![
            "Id".to_string(),
            "GradeQuestSeriesId".to_string(),
            "OrderId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "ConditionsValue2".to_string(),
            "ConditionsDescription".to_string(),
            "DisplayType".to_string(),
        ]),
        "gradequestrewardsdatas.tsv" => Some(vec![
            "Id".to_string(),
            "GradeQuestRewardsId".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
        ]),
        "gradequestseason.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "OrderId".to_string(),
            "Generation".to_string(),
            "Season".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "gradequestseasonreleasecond.tsv" => Some(vec![
            "Id".to_string(),
            "GradeQuestSeasonId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "ConditionsDescription".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "gradequestseriesreleasecond.tsv" => Some(vec![
            "Id".to_string(),
            "GradeQuestSeriesId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "gradequestsquare.tsv" => Some(vec![
            "Id".to_string(),
            "GradeQuestSeriesId".to_string(),
            "SquareId".to_string(),
            "XCoord".to_string(),
            "YCoord".to_string(),
            "OpenGradeQuestSquareIds".to_string(),
        ]),
        "gradequestsquaredatas.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "SquareType".to_string(),
            "TargetId".to_string(),
            "MinActionPoint".to_string(),
            "MaxActionPoint".to_string(),
        ]),
        "gradequeststages.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "Hint".to_string(),
            "StageType".to_string(),
            "LiveStagesId".to_string(),
            "QuestMusicsType".to_string(),
            "QuestMusicsDetail".to_string(),
            "DeckRestrictedType".to_string(),
            "DeckRestrictedDetail".to_string(),
            "LivePoint".to_string(),
        ]),
        "gradequeststagescaseries.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "GradeQuestSeasonId".to_string(),
            "OrderId".to_string(),
            "BackGroundId".to_string(),
            "MapImageId".to_string(),
            "SoundId".to_string(),
            "IsTutorial".to_string(),
            "DefaultGradeAddSkillsId".to_string(),
            "DefaultActionPoint".to_string(),
            "LivePointBonusLimit".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "graderewarddatas.tsv" => Some(vec![
            "Id".to_string(),
            "GradeRewardsId".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
        ]),
        "graderewards.tsv" => Some(vec![
            "Id".to_string(),
            "GradeNum".to_string(),
        ]),
        "grandprix.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "GrandPrixType".to_string(),
            "GuildRankingTabs".to_string(),
            "PersonalRankingTabs".to_string(),
            "GuildPresentCommentId".to_string(),
            "PersonalPresentCommentId".to_string(),
            "InfoStartTime".to_string(),
            "InfoEndTime".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "grandprixdailypoints.tsv" => Some(vec![
            "Id".to_string(),
            "MinRank".to_string(),
            "MaxRank".to_string(),
            "BasePoint".to_string(),
            "AdditionalPoint".to_string(),
        ]),
        "grandprixpointbonuses.tsv" => Some(vec![
            "Id".to_string(),
            "GrandPrixesId".to_string(),
            "TargetType".to_string(),
            "TargetDetail".to_string(),
            "TargetNum".to_string(),
            "BonusValue".to_string(),
        ]),
        "grandprixquestseries.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "GrandPrixesId".to_string(),
            "PlayLimitCount".to_string(),
            "RetireLimitCount".to_string(),
            "OrderId".to_string(),
            "SeriesNum".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "grandprixqueststages.tsv" => Some(vec![
            "Id".to_string(),
            "GrandPrixSeriesId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "Hint".to_string(),
            "OrderId".to_string(),
            "StageType".to_string(),
            "LiveStagesId".to_string(),
            "QuestMusicsType".to_string(),
            "QuestMusicsDetail".to_string(),
            "DeckRestrictedType".to_string(),
            "DeckRestrictedDetail".to_string(),
            "QuestLevel".to_string(),
            "QuestRank".to_string(),
            "FirstClearRewardSeriesId".to_string(),
            "CompleteRewardSeriesId".to_string(),
            "DropRewardSeriesId".to_string(),
            "RandomDropRewardSeriesId".to_string(),
            "Score1".to_string(),
            "Score2".to_string(),
            "Score3".to_string(),
            "StylePoint".to_string(),
            "GainMusicExp".to_string(),
            "ScoreBonusValue0".to_string(),
            "ScoreBonusValue1".to_string(),
            "ScoreBonusValue2".to_string(),
            "ScoreBonusValue3".to_string(),
            "SkipRestrictedType".to_string(),
        ]),
        "grandprixreleasecondition.tsv" => Some(vec![
            "Id".to_string(),
            "ReleaseGrandPrixId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "grandprixrewarddatas.tsv" => Some(vec![
            "Id".to_string(),
            "GrandPrixRewardsId".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
            "IsEmphasize".to_string(),
            "IsUsePresentBox".to_string(),
            "LifeTimeDay".to_string(),
            "RewardTextId".to_string(),
        ]),
        "grandprixrewards.tsv" => Some(vec![
            "Id".to_string(),
            "GrandPrixesId".to_string(),
            "GrandPrixRewardType".to_string(),
            "MinTargetNum".to_string(),
            "MaxTargetNum".to_string(),
            "IsDisplay".to_string(),
        ]),
        "helpimages.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Scene".to_string(),
            "OrderId".to_string(),
            "TitleTextId".to_string(),
        ]),
        "homebgms.tsv" => Some(vec![
            "Id".to_string(),
            "DaytimeBgmId".to_string(),
            "NighttimeBgmId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "ingamemissionskilldetails.tsv" => Some(vec![
            "Id".to_string(),
            "GroupId".to_string(),
            "RareFlag".to_string(),
            "MissionType".to_string(),
            "Param1".to_string(),
            "Param2".to_string(),
            "AddParam".to_string(),
            "BaseScore".to_string(),
            "AddScore".to_string(),
            "Probability".to_string(),
            "MissionText".to_string(),
        ]),
        "itemexchangecategorydatas.tsv" => Some(vec![
            "Id".to_string(),
            "ItemExchangeCategoryName".to_string(),
            "ItemId".to_string(),
        ]),
        "itemexchanges.tsv" => Some(vec![
            "Id".to_string(),
            "ItemExchangeCategoryId".to_string(),
            "ProductItemType".to_string(),
            "ProductItemId".to_string(),
            "ProductItemNum".to_string(),
            "MaterialItemType".to_string(),
            "MaterialItemId".to_string(),
            "MaterialItemNum".to_string(),
            "LimitNum".to_string(),
            "ResetType".to_string(),
            "IsVisibleEndTime".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "items.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "NameFurigana".to_string(),
            "ItemType".to_string(),
            "ItemCategory".to_string(),
            "Rarity".to_string(),
            "EffectValue".to_string(),
            "LimitNum".to_string(),
            "RequestableNum".to_string(),
            "Description".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "itemsources.tsv" => Some(vec![
            "ItemsId".to_string(),
            "StandardQuestStagesId".to_string(),
            "DailyQuestStagesId".to_string(),
        ]),
        "launcherbanners.tsv" => Some(vec![
            "Id".to_string(),
            "DisplayPosition".to_string(),
            "ImageId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "learninglivereleaseconditions.tsv" => Some(vec![
            "Id".to_string(),
            "ReleaseLearningLiveId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "levelupmaterialdetails.tsv" => Some(vec![
            "Id".to_string(),
            "MaterialDetailSeriesId".to_string(),
            "CostItemsId".to_string(),
            "CostNum".to_string(),
        ]),
        "levelupmaterials.tsv" => Some(vec![
            "Id".to_string(),
            "MaterialSeriesId".to_string(),
            "SkillLevel".to_string(),
            "MaterialDetailSeriesId".to_string(),
        ]),
        "limitbreakmaterialconvertrate.tsv" => Some(vec![
            "Id".to_string(),
            "LimitBreakMaterialRarity".to_string(),
            "LimitBreakMaterialNum".to_string(),
            "ProductItemType".to_string(),
            "ProductItemId".to_string(),
            "ProductItemNum".to_string(),
        ]),
        "limitbreakmaterialrate.tsv" => Some(vec![
            "Id".to_string(),
            "CardSeriesId".to_string(),
            "LimitBreakMaterialId".to_string(),
            "LimitBreakMaterialQuantity".to_string(),
        ]),
        "livechannels.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "livecharacters.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
            "SkeletonName".to_string(),
            "ItemsIds".to_string(),
            "PosesIds".to_string(),
        ]),
        "liveeventsevol.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
            "ContentsType".to_string(),
            "LocationsId".to_string(),
            "CharactersIds".to_string(),
            "CostumesIds".to_string(),
            "TimelinesIds".to_string(),
        ]),
        "liveitems.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
            "BindBoneId".to_string(),
            "PosesId".to_string(),
        ]),
        "livelocations.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
            "PropsIds".to_string(),
        ]),
        "livemovies.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
        ]),
        "livemusic.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
            "MusicId".to_string(),
            "HaveMusic".to_string(),
            "HaveMotion".to_string(),
            "CharactersCount".to_string(),
            "CharactersIds".to_string(),
        ]),
        "liveposes.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
            "HandSide".to_string(),
        ]),
        "liveprops.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
        ]),
        "livestages.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "BaseUseMental".to_string(),
            "MentalStepClass".to_string(),
            "BaseGainVoltage".to_string(),
            "VoltageStepClass".to_string(),
            "BackGroundId".to_string(),
            "StageSkillConditionId".to_string(),
            "StageSkillEffectId".to_string(),
            "StageSkillDescription".to_string(),
            "StageSkillSetIds".to_string(),
        ]),
        "livetimeslinesevol.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
            "MusicId".to_string(),
            "LocationsId".to_string(),
            "FreeId".to_string(),
            "NextId".to_string(),
            "MovieIds".to_string(),
        ]),
        "loginbonuses.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "LoginBonusRewardSeriesID".to_string(),
            "IsLoop".to_string(),
            "LoginBonusTextId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "loginbonusrewarddatas.tsv" => Some(vec![
            "Id".to_string(),
            "LoginBonusRewardSeriesId".to_string(),
            "DayCount".to_string(),
            "RewardType".to_string(),
            "RewardId".to_string(),
            "RewardNum".to_string(),
            "LifeTimeDay".to_string(),
            "RewardTextId".to_string(),
        ]),
        "memberfanlevels.tsv" => Some(vec![
            "Id".to_string(),
            "MemberFanLevel".to_string(),
            "Experience".to_string(),
            "CumulativeExperience".to_string(),
        ]),
        "membermovies.tsv" => Some(vec![
            "Id".to_string(),
            "CharactersId".to_string(),
            "MovieType".to_string(),
            "Name".to_string(),
            "Priority".to_string(),
            "ReleaseConditionText".to_string(),
        ]),
        "membervoices.tsv" => Some(vec![
            "Id".to_string(),
            "CharactersId".to_string(),
            "Name".to_string(),
            "Priority".to_string(),
            "VoiceName".to_string(),
            "ReleaseConditionText".to_string(),
        ]),
        "missionachieverewards.tsv" => Some(vec![
            "Id".to_string(),
            "MissionType".to_string(),
            "AchieveMarkNum".to_string(),
            "RewardType".to_string(),
            "ItemsId".to_string(),
            "RewardNum".to_string(),
            "RewardTextId".to_string(),
            "SortOrder".to_string(),
        ]),
        "missionrewards.tsv" => Some(vec![
            "Id".to_string(),
            "MissionsId".to_string(),
            "RewardCategory".to_string(),
            "RewardType".to_string(),
            "ItemsId".to_string(),
            "RewardNum".to_string(),
            "RewardTextId".to_string(),
        ]),
        "missions.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "MissionType".to_string(),
            "MissionConditionType".to_string(),
            "MissionConditionNum".to_string(),
            "MissionConditionDetail".to_string(),
            "OpenType".to_string(),
            "NextMissionsId".to_string(),
            "SortOrder".to_string(),
            "TransitionContentsId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "Hint".to_string(),
        ]),
        "musicdroprewarddetails.tsv" => Some(vec![
            "Id".to_string(),
            "MusicDropRewardsId".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
            "Odds".to_string(),
        ]),
        "musicdroprewards.tsv" => Some(vec![
            "Id".to_string(),
            "DropRewardSeriesId".to_string(),
            "MusicDropRewardOdds".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "musiclearningquestseries.tsv" => Some(vec![
            "Id".to_string(),
            "MusicsId".to_string(),
            "Name".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "ExReleaseConditionsLevel".to_string(),
        ]),
        "musiclearningqueststages.tsv" => Some(vec![
            "Id".to_string(),
            "LearningLiveSeriesId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "Hint".to_string(),
            "StageType".to_string(),
            "UseType".to_string(),
            "UseItem".to_string(),
            "UseNum".to_string(),
            "LiveStagesId".to_string(),
            "QuestMusicsType".to_string(),
            "QuestMusicsDetail".to_string(),
            "DeckRestrictedType".to_string(),
            "DeckRestrictedDetail".to_string(),
            "QuestLevel".to_string(),
            "QuestRank".to_string(),
            "FirstClearRewardSeriesId".to_string(),
            "DropRewardSeriesId".to_string(),
            "RandomDropRewardSeriesId".to_string(),
            "Score1".to_string(),
            "GainStylePoint".to_string(),
            "GainMusicExp".to_string(),
            "SkipRestrictedType".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "musiclevels.tsv" => Some(vec![
            "Id".to_string(),
            "ExperienceType".to_string(),
            "Level".to_string(),
            "Experience".to_string(),
            "CumulativeExperience".to_string(),
        ]),
        "musicmasteryheartbonuses.tsv" => Some(vec![
            "Id".to_string(),
            "Level".to_string(),
            "LoveRate".to_string(),
        ]),
        "musicmasterylevels.tsv" => Some(vec![
            "Id".to_string(),
            "MusicsId".to_string(),
            "Level".to_string(),
            "MusicMasterySkillsId".to_string(),
        ]),
        "musicmasterylovebonuses.tsv" => Some(vec![
            "Id".to_string(),
            "Level".to_string(),
            "LoveRate".to_string(),
        ]),
        "musicmasterymentalbonuses.tsv" => Some(vec![
            "Id".to_string(),
            "Level".to_string(),
            "DemandDamagePt".to_string(),
        ]),
        "musicmasteryskill.tsv" => Some(vec![
            "Id".to_string(),
            "MusicMasterySkillsName".to_string(),
        ]),
        "musicmasteryvoltagebonuses.tsv" => Some(vec![
            "Id".to_string(),
            "Level".to_string(),
            "DemandVoltagePt".to_string(),
        ]),
        "musics.tsv" => Some(vec![
            "Id".to_string(),
            "OrderId".to_string(),
            "Title".to_string(),
            "TitleFurigana".to_string(),
            "JacketId".to_string(),
            "SoundId".to_string(),
            "Description".to_string(),
            "GenerationsId".to_string(),
            "UnitId".to_string(),
            "CenterCharacterId".to_string(),
            "SingerCharacterId".to_string(),
            "SupportCharacterId".to_string(),
            "MusicType".to_string(),
            "ExperienceType".to_string(),
            "BeatPointCoefficient".to_string(),
            "ApIncrement".to_string(),
            "SongTime".to_string(),
            "PlayTime".to_string(),
            "FeverSectionNo".to_string(),
            "PreviewStartTime".to_string(),
            "PreviewEndTime".to_string(),
            "PreviewFadeInTime".to_string(),
            "PreviewFadeOutTime".to_string(),
            "ReleaseConditionType".to_string(),
            "ReleaseConditionDetail".to_string(),
            "ReleaseConditionText".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "MaxAp".to_string(),
            "IsVideoMode".to_string(),
            "VideoBgId".to_string(),
            "SongType".to_string(),
            "MusicScoreReleaseTime".to_string(),
        ]),
        "musicscorerewarddatas.tsv" => Some(vec![
            "Id".to_string(),
            "MissionStars".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
        ]),
        "musicscorerewards.tsv" => Some(vec![
            "Id".to_string(),
            "ScoreRewardSeriesId".to_string(),
            "ScoreRewardType".to_string(),
            "ScoreRewardConditionValue1".to_string(),
            "ScoreRewardConditionValue2".to_string(),
            "ScoreRewardConditionValue3".to_string(),
            "ScoreRewardConditionValue4".to_string(),
            "ScoreRewardDatasId1".to_string(),
            "ScoreRewardDatasId2".to_string(),
            "ScoreRewardDatasId3".to_string(),
            "ScoreRewardDatasId4".to_string(),
        ]),
        "musicscores.tsv" => Some(vec![
            "Id".to_string(),
            "NormalLevel".to_string(),
            "HardLevel".to_string(),
            "ExpertLevel".to_string(),
            "MasterLevel".to_string(),
            "NormalMaxCombo".to_string(),
            "HardMaxCombo".to_string(),
            "ExpertMaxCombo".to_string(),
            "MasterMaxCombo".to_string(),
            "ShouldVerifyNotesCount".to_string(),
            "ScoreRewardSeriesId".to_string(),
            "NormalGainMusicExp".to_string(),
            "HardGainMusicExp".to_string(),
            "ExpertGainMusicExp".to_string(),
            "MasterGainMusicExp".to_string(),
            "NormalDropRewardSeriesId".to_string(),
            "HardDropRewardSeriesId".to_string(),
            "ExpertDropRewardSeriesId".to_string(),
            "MasterDropRewardSeriesId".to_string(),
        ]),
        "petalcoinexchangerate.tsv" => Some(vec![
            "Id".to_string(),
            "Rarity".to_string(),
            "PetalCoinQuantity".to_string(),
        ]),
        "petalexchangerates.tsv" => Some(vec![
            "Id".to_string(),
            "Rarity".to_string(),
            "Price".to_string(),
            "ExchangeLimitLower".to_string(),
            "ExchangeLimitUpper".to_string(),
        ]),
        "presenttexts.tsv" => Some(vec![
            "Id".to_string(),
            "Description".to_string(),
        ]),
        "questareareleaseconditions.tsv" => Some(vec![
            "Id".to_string(),
            "ReleaseAreaId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "questlivedownloads.tsv" => Some(vec![
            "Id".to_string(),
            "Title".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "questliveloadings.tsv" => Some(vec![
            "Id".to_string(),
            "Title".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "questlivereleaseconditions.tsv" => Some(vec![
            "Id".to_string(),
            "ReleaseQuestId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "questsections.tsv" => Some(vec![
            "Id".to_string(),
            "SectionNo".to_string(),
            "QuestStagesId".to_string(),
            "SectionSkillsId".to_string(),
        ]),
        "raidevents.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "DropNumUp".to_string(),
            "PersonalRankingTabs".to_string(),
            "InfoStartTime".to_string(),
            "InfoEndTime".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "raidquestdroprateup.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Flame".to_string(),
        ]),
        "raidquestreleasecondition.tsv" => Some(vec![
            "Id".to_string(),
            "ReleaseRaidId".to_string(),
            "ConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "raidquestseries.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "RaidEventId".to_string(),
            "PointAddLimit".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "raidqueststages.tsv" => Some(vec![
            "Id".to_string(),
            "RaidQuestSeriesId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "Hint".to_string(),
            "OrderId".to_string(),
            "StageType".to_string(),
            "LiveStagesId".to_string(),
            "QuestMusicsType".to_string(),
            "QuestMusicsDetail".to_string(),
            "DeckRestrictedType".to_string(),
            "DeckRestrictedDetail".to_string(),
            "QuestLevel".to_string(),
            "QuestRank".to_string(),
            "FirstClearRewardSeriesId".to_string(),
            "CompleteRewardSeriesId".to_string(),
            "DropRewardSeriesId".to_string(),
            "RandomDropRewardSeriesId".to_string(),
            "Score1".to_string(),
            "Score2".to_string(),
            "Score3".to_string(),
            "StylePoint".to_string(),
            "GainMusicExp".to_string(),
        ]),
        "raidresource.tsv" => Some(vec![
            "Id".to_string(),
            "RaidEventId".to_string(),
            "RaidResourceDefaultNum".to_string(),
            "RaidResourceLimit".to_string(),
            "RaidResourceAddLimit".to_string(),
        ]),
        "raidresourceadddate.tsv" => Some(vec![
            "Id".to_string(),
            "RaidResourceId".to_string(),
            "AddNum".to_string(),
            "AddTime".to_string(),
        ]),
        "raidresourcerecoverydatas.tsv" => Some(vec![
            "Id".to_string(),
            "RaidResourceId".to_string(),
            "Order".to_string(),
            "RequireItemType".to_string(),
            "RequireItemId".to_string(),
            "RequireItemNum".to_string(),
        ]),
        "raidrewarddatas.tsv" => Some(vec![
            "Id".to_string(),
            "RaidRewardsId".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
        ]),
        "raidrewards.tsv" => Some(vec![
            "Id".to_string(),
            "RaidEventId".to_string(),
            "RewardType".to_string(),
            "RequirePointAmount".to_string(),
        ]),
        "raidtopprogressimage.tsv" => Some(vec![
            "Id".to_string(),
            "RaidEventId".to_string(),
            "Order".to_string(),
            "RequirePoint".to_string(),
        ]),
        "rentalcarddatas.tsv" => Some(vec![
            "Id".to_string(),
            "CardDatasId".to_string(),
            "StyleLevel".to_string(),
            "LimitBreakLevel".to_string(),
            "SpecialAppealLevel".to_string(),
            "SkillLevel".to_string(),
        ]),
        "rentaldeckcards.tsv" => Some(vec![
            "Id".to_string(),
            "RentalDecksId".to_string(),
            "CharactersId".to_string(),
            "RentalCardId_Main".to_string(),
            "RentalCardId_Side1".to_string(),
            "RentalCardId_Side2".to_string(),
        ]),
        "rentaldecks.tsv" => Some(vec![
            "Id".to_string(),
            "DeckName".to_string(),
            "GenerationsId".to_string(),
            "DeckNumber".to_string(),
            "AceCardId".to_string(),
            "ReleaseConditionsType".to_string(),
            "ConditionsValue".to_string(),
            "ReleaseConditonsDescription".to_string(),
            "ReleaseDeckStartTime".to_string(),
            "ReleaseDeckEndTime".to_string(),
        ]),
        "rhythmgameclassdatas.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "RhythmGameConditionType".to_string(),
            "RhythmGameClassesId".to_string(),
            "ClassTier".to_string(),
            "GoalCount".to_string(),
            "OrderId".to_string(),
            "ReleaseStartTime".to_string(),
            "ReleaseEndTime".to_string(),
        ]),
        "rhythmgameclasses.tsv" => Some(vec![
            "Id".to_string(),
            "ClassName".to_string(),
            "MaxClassTier".to_string(),
        ]),
        "rhythmgameclassmissionrewards.tsv" => Some(vec![
            "Id".to_string(),
            "RhythmGameClassDatasId".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
            "OrderId".to_string(),
        ]),
        "rhythmgamehelpimages.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Scene".to_string(),
            "OrderId".to_string(),
        ]),
        "rhythmgameskillconditions.tsv" => Some(vec![
            "Id".to_string(),
            "RhythmGameSkillConditionType".to_string(),
            "RhythmGameSkillConditionValue".to_string(),
            "RhythmGameSkillConditionValue2".to_string(),
        ]),
        "rhythmgameskilleffects.tsv" => Some(vec![
            "Id".to_string(),
            "RhythmGameSkillEffectType".to_string(),
            "TargetIds".to_string(),
            "RhythmGameSkillEffectValue".to_string(),
            "RhythmGameSkillEffectValue2".to_string(),
        ]),
        "rhythmgameskilllvupitemdetails.tsv" => Some(vec![
            "Id".to_string(),
            "ItemDetailSeriesId".to_string(),
            "CostItemsId".to_string(),
            "CostNum".to_string(),
        ]),
        "rhythmgameskilllvupitems.tsv" => Some(vec![
            "Id".to_string(),
            "ItemSeriesId".to_string(),
            "SkillLevel".to_string(),
            "ItemDetailSeriesId".to_string(),
        ]),
        "rhythmgameskills.tsv" => Some(vec![
            "Id".to_string(),
            "RhythmGameSkillSeriesId".to_string(),
            "RhythmGameSkillName".to_string(),
            "SkillLevel".to_string(),
            "OrderId".to_string(),
            "RhythmGameSkillConditionIds".to_string(),
            "RhythmGameSkillEffectId".to_string(),
            "ConsumeAP".to_string(),
            "Description".to_string(),
        ]),
        "rhythmgametotalmissionrewards.tsv" => Some(vec![
            "Id".to_string(),
            "RhythmGameTotalMissionsId".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
            "OrderId".to_string(),
        ]),
        "rhythmgametotalmissions.tsv" => Some(vec![
            "Id".to_string(),
            "Description".to_string(),
            "ConditionRhythmGameClassDatasOrderId".to_string(),
            "OrderId".to_string(),
            "ReleaseStartTime".to_string(),
            "ReleaseEndTime".to_string(),
        ]),
        "seasonfanlevels.tsv" => Some(vec![
            "Id".to_string(),
            "SeasonsId".to_string(),
            "SeasonFanLevel".to_string(),
            "Experience".to_string(),
            "CumulativeExperience".to_string(),
        ]),
        "seasongrade.tsv" => Some(vec![
            "Id".to_string(),
            "Description".to_string(),
            "PersonalRankingTabs".to_string(),
            "CountStartTime".to_string(),
            "CountEndTime".to_string(),
            "DisplayStartTime".to_string(),
            "DisplayEndTime".to_string(),
            "TermTitle".to_string(),
        ]),
        "seasongraderewarddatas.tsv" => Some(vec![
            "Id".to_string(),
            "SeasonGradeRewardsId".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
            "IsEmphasize".to_string(),
        ]),
        "seasongraderewards.tsv" => Some(vec![
            "Id".to_string(),
            "SeasonGradeId".to_string(),
            "MinTargetNum".to_string(),
            "MaxTargetNum".to_string(),
        ]),
        "seasons.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "sectionskilleffectdetails.tsv" => Some(vec![
            "Id".to_string(),
            "SkillEffectDetailType".to_string(),
            "TargetMood".to_string(),
            "EffectValue".to_string(),
        ]),
        "sectionskilleffects.tsv" => Some(vec![
            "Id".to_string(),
            "ActionType".to_string(),
            "OrderId".to_string(),
        ]),
        "sectionskills.tsv" => Some(vec![
            "Id".to_string(),
            "Description".to_string(),
            "ApperanceType".to_string(),
            "SkillIcon".to_string(),
            "SectionSkillsEffectId".to_string(),
        ]),
        "selectticketexchangerate.tsv" => Some(vec![
            "Id".to_string(),
            "SelectTicketSeriesId".to_string(),
            "ExchangeItemType".to_string(),
            "ExchangeItemId".to_string(),
            "ExchangeItemQuantity".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "selectticketseries.tsv" => Some(vec![
            "Id".to_string(),
            "ExchangeTicketName".to_string(),
            "Description".to_string(),
            "ExchangeTicketId".to_string(),
            "OrderId".to_string(),
            "PickUpCardSeriesId_1".to_string(),
            "PickUpCardSeriesId_2".to_string(),
            "PickUpCardSeriesId_3".to_string(),
            "PickUpCardSeriesId_4".to_string(),
            "PickUpCardSeriesId_5".to_string(),
            "PickUpCardSeriesId_6".to_string(),
            "BgType".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "IsVisibleEndTime".to_string(),
        ]),
        "shopitems.tsv" => Some(vec![
            "Id".to_string(),
            "ShopId".to_string(),
            "Name".to_string(),
            "ItemType".to_string(),
            "ItemId".to_string(),
            "ItemQuantity".to_string(),
            "Price".to_string(),
            "IsPaidSIsCaOnly".to_string(),
            "OrderId".to_string(),
            "Description".to_string(),
            "RewardTextId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "shops.tsv" => Some(vec![
            "Id".to_string(),
            "ShopType".to_string(),
            "Name".to_string(),
            "OrderId".to_string(),
        ]),
        "sidestylesettings.tsv" => Some(vec![
            "Id".to_string(),
            "CardSeriesId".to_string(),
            "GenerationsId".to_string(),
            "SideStyleSetCharacterIds".to_string(),
        ]),
        "simulationgraphlimit.tsv" => Some(vec![
            "Id".to_string(),
            "NumberOfMember".to_string(),
            "UpperLimitSmile".to_string(),
            "UpperLimitPure".to_string(),
            "UpperLimitCool".to_string(),
            "UpperLimitMental".to_string(),
            "UpperLimitBP".to_string(),
        ]),
        "stagescoremultipliersettings.tsv" => Some(vec![
            "Id".to_string(),
            "MinTargetNum".to_string(),
            "MaxTargetNum".to_string(),
            "BaseMultiplier".to_string(),
            "AdditionalMultiplier".to_string(),
        ]),
        "stageskillconditiondetails.tsv" => Some(vec![
            "Id".to_string(),
            "StageSkillConditionId".to_string(),
            "SkillConditionDetailType".to_string(),
            "ConditionValue".to_string(),
        ]),
        "stageskillconditions.tsv" => Some(vec![
            "Id".to_string(),
            "ConditionType".to_string(),
        ]),
        "stageskilleffectdetails.tsv" => Some(vec![
            "Id".to_string(),
            "StageSkillEffectId".to_string(),
            "SkillEffectDetailType".to_string(),
            "EffectValue".to_string(),
        ]),
        "stageskilleffects.tsv" => Some(vec![
            "Id".to_string(),
            "ActionType".to_string(),
        ]),
        "stageskillsets.tsv" => Some(vec![
            "Id".to_string(),
            "StageSkillConditionId".to_string(),
            "StageSkillEffectId".to_string(),
        ]),
        "stamps.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "StampNo".to_string(),
            "StampType".to_string(),
            "CharactersId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "standardquestareas.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Number".to_string(),
            "Description".to_string(),
            "OrderId".to_string(),
            "ImageType".to_string(),
            "BgImageId".to_string(),
            "SoundId".to_string(),
            "GenerationsId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "standardqueststages.tsv" => Some(vec![
            "Id".to_string(),
            "StandardQuestAreasId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "Hint".to_string(),
            "StageType".to_string(),
            "UseType".to_string(),
            "UseItem".to_string(),
            "UseNum".to_string(),
            "LiveStagesId".to_string(),
            "QuestMusicsType".to_string(),
            "QuestMusicsDetail".to_string(),
            "DeckRestrictedType".to_string(),
            "DeckRestrictedDetail".to_string(),
            "QuestLevel".to_string(),
            "FirstClearRewardSeriesId".to_string(),
            "CompleteRewardSeriesId".to_string(),
            "DropRewardSeriesId".to_string(),
            "RandomDropRewardSeriesId".to_string(),
            "Score1".to_string(),
            "Score2".to_string(),
            "Score3".to_string(),
            "GainStylePoint".to_string(),
            "GainMusicExp".to_string(),
            "ItemSourceIconId".to_string(),
        ]),
        "stickerexchanges.tsv" => Some(vec![
            "Id".to_string(),
            "ProductStickerId".to_string(),
            "ProductStickerNum".to_string(),
            "MaterialItemType".to_string(),
            "MaterialItemId".to_string(),
            "MaterialItemNum".to_string(),
            "LimitNum".to_string(),
            "ResetType".to_string(),
            "IsVisibleEndTime".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        "stickers.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Text".to_string(),
            "CategoryType".to_string(),
            "CategoryName".to_string(),
            "SeasonId".to_string(),
            "CharactersId".to_string(),
            "Priority".to_string(),
            "IsVariant".to_string(),
            "RequirementType".to_string(),
            "RequirementDetail".to_string(),
            "RequirementValue".to_string(),
            "RequirementText".to_string(),
            "EditRequirementType".to_string(),
            "EditRequirementDetail".to_string(),
            "EditRequirementValue".to_string(),
            "EditRequirementText".to_string(),
            "IsVisibleOnlyPossess".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
            "AvailableStartTime".to_string(),
            "AvailableEndTime".to_string(),
            "VariantStartTime".to_string(),
            "VariantEndTime".to_string(),
        ]),
        "stylemovies.tsv" => Some(vec![
            "Id".to_string(),
            "CardSeriesId".to_string(),
            "MovieType".to_string(),
            "Name".to_string(),
            "ReleaseConditionText".to_string(),
        ]),
        "stylevoices.tsv" => Some(vec![
            "Id".to_string(),
            "CardSeriesId".to_string(),
            "Name".to_string(),
            "Priority".to_string(),
            "VoiceName".to_string(),
            "ReleaseConditionText".to_string(),
        ]),
        "subcharacters.tsv" => Some(vec![
            "Id".to_string(),
            "Label".to_string(),
        ]),
        "tablist.tsv" => Some(vec![
            "Id".to_string(),
            "TabListName".to_string(),
        ]),
        "targets.tsv" => Some(vec![
            "Id".to_string(),
            "TargetType".to_string(),
            "TargetValue".to_string(),
        ]),
        "textsplaceholder.tsv" => Some(vec![
            "Id".to_string(),
            "Description".to_string(),
        ]),
        "ticketonlygachas.tsv" => Some(vec![
            "Id".to_string(),
            "GachaTicketId".to_string(),
            "TicketNum".to_string(),
            "ConsectiveGachaTimes".to_string(),
            "GachaButtonType".to_string(),
            "ButtonPopId".to_string(),
            "MiniBannerPopId".to_string(),
            "MiniBannerDispCondition".to_string(),
        ]),
        "tutorialdeckcards.tsv" => Some(vec![
            "Id".to_string(),
            "TutorialDeckDatasId".to_string(),
            "SlotNo".to_string(),
            "CardDatasId".to_string(),
            "StyleLevel".to_string(),
            "LimitBreakTimes".to_string(),
            "SpecialAppealLevel".to_string(),
            "SkillLevel".to_string(),
        ]),
        "tutorialdeckdatas.tsv" => Some(vec![
            "Id".to_string(),
            "DeckName".to_string(),
            "DeckNo".to_string(),
            "GenerationsId".to_string(),
        ]),
        "tutorialquestareas.tsv" => Some(vec![
            "Id".to_string(),
            "Name".to_string(),
            "Number".to_string(),
            "Description".to_string(),
            "OrderId".to_string(),
            "ImageType".to_string(),
            "BgImageId".to_string(),
            "Next_TutorialQuestAreasId".to_string(),
            "GenerationsId".to_string(),
        ]),
        "tutorialqueststages.tsv" => Some(vec![
            "Id".to_string(),
            "TutorialQuestAreasId".to_string(),
            "Name".to_string(),
            "Description".to_string(),
            "Hint".to_string(),
            "MapNumber".to_string(),
            "StageType".to_string(),
            "UseType".to_string(),
            "Use_ItemsId".to_string(),
            "UseNum".to_string(),
            "LiveStagesId".to_string(),
            "QuestMusicsId".to_string(),
            "LoveCorrectionValue".to_string(),
            "BonusVoltage".to_string(),
            "BonusMental".to_string(),
            "BonusHeart".to_string(),
            "BonusLove".to_string(),
            "FirstClearRewardSeriesId".to_string(),
            "CompleteRewardSeriesId".to_string(),
            "RandomDropRewardSeriesId".to_string(),
            "Score1".to_string(),
            "Score2".to_string(),
            "Score3".to_string(),
            "StylePoint".to_string(),
            "GainMusicExp".to_string(),
        ]),
        "tutorialrewarddatas.tsv" => Some(vec![
            "Id".to_string(),
            "TutorialsId".to_string(),
            "RewardType".to_string(),
            "RewardItemId".to_string(),
            "RewardNum".to_string(),
            "LifeTimeDay".to_string(),
            "RewardTextId".to_string(),
        ]),
        "tutorials.tsv" => Some(vec![
            "Id".to_string(),
            "TutorialType".to_string(),
            "Step".to_string(),
            "Description".to_string(),
        ]),
        "tutorialschoolidolstagemovies.tsv" => Some(vec![
            "Id".to_string(),
            "Title".to_string(),
            "OrderId".to_string(),
        ]),
        "unitcharacters.tsv" => Some(vec![
            "Id".to_string(),
            "UnitsId".to_string(),
            "CharactersId".to_string(),
            "OrderId".to_string(),
        ]),
        "units.tsv" => Some(vec![
            "Id".to_string(),
            "UnitName".to_string(),
            "OrderId".to_string(),
            "StartTime".to_string(),
            "EndTime".to_string(),
        ]),
        _ => None,
    }
}