use std::collections::HashSet;

pub fn get_available_tables() -> HashSet<String> {
    HashSet::from([
        "_root".to_string(),
        "Achievement".to_string(),
        "Achievement_Category".to_string(),
        "ActionBarGroup".to_string(),
        "AdventureJournal".to_string(),
        "AdventureMapPOI".to_string(),
        "AlliedRace".to_string(),
        "AnimKit".to_string(),
        "AnimKitBoneSet".to_string(),
        "AnimKitBoneSetAlias".to_string(),
        "AnimKitConfig".to_string(),
        "AnimKitConfigBoneSet".to_string(),
        "AnimKitPriority".to_string(),
        "AnimKitSegment".to_string(),
        "AnimReplacement".to_string(),
        "AnimReplacementSet".to_string(),
        "AnimaCable".to_string(),
        "AnimaCylinder".to_string(),
        "AnimaMaterial".to_string(),
        "AreaConditionalData".to_string(),
        "AreaGroupMember".to_string(),
        "AreaPOIState".to_string(),
        "AreaPOIUiWidgetSet".to_string(),
        "AreaTable".to_string(),
        "AreaTrigger".to_string(),
        "AreaTriggerActionSet".to_string(),
        "AreaTriggerBox".to_string(),
        "AreaTriggerCreateProperties".to_string(),
        "AreaTriggerCylinder".to_string(),
        "AreaTriggerDisk".to_string(),
        "AreaTriggerSphere".to_string(),
        "Artifact".to_string(),
        "ArtifactAppearance".to_string(),
        "ArtifactAppearanceSet".to_string(),
        "ArtifactItemToTransmog".to_string(),
        "ArtifactPower".to_string(),
        "ArtifactPowerLink".to_string(),
        "ArtifactPowerPicker".to_string(),
        "ArtifactPowerRank".to_string(),
        "ArtifactQuestXP".to_string(),
        "ArtifactTier".to_string(),
        "ArtifactUnlock".to_string(),
        "AuctionHouse".to_string(),
        "AuctionHouseCategory".to_string(),
        "AzeriteEmpoweredItem".to_string(),
        "AzeriteEssence".to_string(),
        "AzeriteEssencePower".to_string(),
        "AzeriteItemMilestonePower".to_string(),
        "AzeriteKnowledgeMultiplier".to_string(),
        "AzeriteLevelInfo".to_string(),
        "AzeritePower".to_string(),
        "AzeritePowerSetMember".to_string(),
        "AzeriteTierUnlock".to_string(),
        "AzeriteTierUnlockSet".to_string(),
        "AzeriteUnlockMapping".to_string(),
        "BankBagSlotPrices".to_string(),
        "BannedAddons".to_string(),
        "BarrageEffect".to_string(),
        "BarberShopStyle".to_string(),
        "BattlePetAbility".to_string(),
        "BattlePetAbilityEffect".to_string(),
        "BattlePetAbilityState".to_string(),
        "BattlePetAbilityTurn".to_string(),
        "BattlePetBreedQuality".to_string(),
        "BattlePetBreedState".to_string(),
        "BattlePetDisplayOverride".to_string(),
        "BattlePetEffectProperties".to_string(),
        "BattlePetSpecies".to_string(),
        "BattlePetSpeciesState".to_string(),
        "BattlePetSpeciesXAbility".to_string(),
        "BattlePetVisual".to_string(),
        "BattlepayCurrency".to_string(),
        "BeamEffect".to_string(),
        "BoneWindModifierModel".to_string(),
        "BoneWindModifiers".to_string(),
        "BonusRoll".to_string(),
        "Bounty".to_string(),
        "BountySet".to_string(),
        "BroadcastText".to_string(),
        "BroadcastTextDuration".to_string(),
        "CameraEffect".to_string(),
        "CameraEffectEntry".to_string(),
        "CameraMode".to_string(),
        "Campaign".to_string(),
        "CampaignXCondition".to_string(),
        "CampaignXQuestLine".to_string(),
        "CastableRaidBuffs".to_string(),
        "CelestialBody".to_string(),
        "Cfg_Categories".to_string(),
        "Cfg_Configs".to_string(),
        "Cfg_Regions".to_string(),
        "CharBaseInfo".to_string(),
        "CharBaseSection".to_string(),
        "CharComponentTextureLayouts".to_string(),
        "CharComponentTextureSections".to_string(),
        "CharHairGeosets".to_string(),
        "CharSectionCondition".to_string(),
        "CharShipment".to_string(),
        "CharShipmentContainer".to_string(),
        "CharTitles".to_string(),
        "CharacterCreateScreens".to_string(),
        "CharacterLoadout".to_string(),
        "CharacterLoadoutItem".to_string(),
        "CharacterLoadoutPet".to_string(),
        "CharacterServiceInfo".to_string(),
        "ChatChannels".to_string(),
        "ChatProfanity".to_string(),
        "ChrClassRaceSex".to_string(),
        "ChrClassTitle".to_string(),
        "ChrClassUIChrModelInfo".to_string(),
        "ChrClassUIDisplay".to_string(),
        "ChrClassVillain".to_string(),
        "ChrClasses".to_string(),
        "ChrClassesXPowerTypes".to_string(),
        "ChrCreateClassAnimTarget".to_string(),
        "ChrCreateClassAnimTargetInfo".to_string(),
        "ChrCustGeoComponentLink".to_string(),
        "ChrCustItemGeoModify".to_string(),
        "ChrCustomization".to_string(),
        "ChrCustomizationBoneSet".to_string(),
        "ChrCustomizationCategory".to_string(),
        "ChrCustomizationCondModel".to_string(),
        "ChrCustomizationConversion".to_string(),
        "ChrCustomizationDisplayInfo".to_string(),
        "ChrCustomizationElement".to_string(),
        "ChrCustomizationGeoset".to_string(),
        "ChrCustomizationGlyphPet".to_string(),
        "ChrCustomizationMaterial".to_string(),
        "ChrCustomizationOption".to_string(),
        "ChrCustomizationReq".to_string(),
        "ChrCustomizationSkinnedModel".to_string(),
        "ChrCustomizationVisReq".to_string(),
        "ChrCustomizationVoice".to_string(),
        "ChrModel".to_string(),
        "ChrModelMaterial".to_string(),
        "ChrModelTextureLayer".to_string(),
        "ChrRaceRacialAbility".to_string(),
        "ChrRaceXChrModel".to_string(),
        "ChrRaces".to_string(),
        "ChrRacesCreateScreenIcon".to_string(),
        "ChrSelectBackgroundCDI".to_string(),
        "ChrSpecialization".to_string(),
        "ChrUpgradeBucket".to_string(),
        "ChrUpgradeBucketSpell".to_string(),
        "ChrUpgradeTier".to_string(),
        "CinematicCamera".to_string(),
        "CinematicSequences".to_string(),
        "ClientSceneEffect".to_string(),
        "ClientSettings".to_string(),
        "CollectableSourceEncounter".to_string(),
        "CollectableSourceInfo".to_string(),
        "CollectableSourceQuest".to_string(),
        "CollectableSourceVendor".to_string(),
        "CollectableSourceVendorSparse".to_string(),
        "CombatCondition".to_string(),
        "CommentatorIndirectSpell".to_string(),
        "CommentatorStartLocation".to_string(),
        "CommentatorTrackedCooldown".to_string(),
        "CommunityIcon".to_string(),
        "ComponentModelFileData".to_string(),
        "ComponentTextureFileData".to_string(),
        "ConditionalChrModel".to_string(),
        "ConditionalContentTuning".to_string(),
        "ConditionalItemAppearance".to_string(),
        "ConfigurationWarning".to_string(),
        "ContentRestrictionRule".to_string(),
        "ContentRestrictionRuleSet".to_string(),
        "ContentTuning".to_string(),
        "ContentTuningXExpected".to_string(),
        "ContentTuningXLabel".to_string(),
        "Contribution".to_string(),
        "ContributionStyle".to_string(),
        "ContributionStyleContainer".to_string(),
        "ConversationLine".to_string(),
        "CorruptionEffects".to_string(),
        "Covenant".to_string(),
        "CraftingData".to_string(),
        "CraftingDataEnchantQuality".to_string(),
        "CraftingDifficulty".to_string(),
        "CraftingDifficultyQuality".to_string(),
        "CraftingOrder".to_string(),
        "CraftingOrderHouse".to_string(),
        "CraftingOrderXLabel".to_string(),
        "CraftingQuality".to_string(),
        "CraftingReagentEffect".to_string(),
        "CraftingReagentQuality".to_string(),
        "CraftingReagentRequirement".to_string(),
        "Creature".to_string(),
        "CreatureDifficulty".to_string(),
        "CreatureDifficultyTreasure".to_string(),
        "CreatureDisplayInfo".to_string(),
        "CreatureDisplayInfoCond".to_string(),
        "CreatureDisplayInfoCondXChoice".to_string(),
        "CreatureDisplayInfoEvt".to_string(),
        "CreatureDisplayInfoExtra".to_string(),
        "CreatureDisplayInfoGeosetData".to_string(),
        "CreatureDisplayInfoOption".to_string(),
        "CreatureDisplayInfoTrn".to_string(),
        "CreatureFamily".to_string(),
        "CreatureFamilyXUIModelScene".to_string(),
        "CreatureImmunities".to_string(),
        "CreatureLabel".to_string(),
        "CreatureModelData".to_string(),
        "CreatureMovementInfo".to_string(),
        "CreatureSoundData".to_string(),
        "CreatureType".to_string(),
        "CreatureXContribution".to_string(),
        "CreatureXDisplayInfo".to_string(),
        "CreatureXUiWidgetSet".to_string(),
        "Criteria".to_string(),
        "CriteriaTree".to_string(),
        "CurrencyCategory".to_string(),
        "CurrencyTypes".to_string(),
        "Curve".to_string(),
        "CurvePoint".to_string(),
        "DeathThudLookups".to_string(),
        "DecalProperties".to_string(),
        "DelvesSeason".to_string(),
        "DelvesSeasonXSpell".to_string(),
        "DestructibleModelData".to_string(),
        "DeviceBlacklist".to_string(),
        "Difficulty".to_string(),
        "DissolveEffect".to_string(),
        "DriverBlacklist".to_string(),
        "DungeonEncounter".to_string(),
        "DurabilityQuality".to_string(),
        "EdgeGlowEffect".to_string(),
        "EmotesText".to_string(),
        "EmotesTextData".to_string(),
        "EmotesTextSound".to_string(),
        "EnvironmentalDamage".to_string(),
        "Exhaustion".to_string(),
        "ExpectedStat".to_string(),
        "ExpectedStatMod".to_string(),
        "ExtraAbilityInfo".to_string(),
        "Faction".to_string(),
        "FactionGroup".to_string(),
        "FactionTemplate".to_string(),
        "FileData".to_string(),
        "FlightCapability".to_string(),
        "FlightCapabilityXGlideEvent".to_string(),
        "FootprintTextures".to_string(),
        "FootstepTerrainLookup".to_string(),
        "FriendshipRepReaction".to_string(),
        "FriendshipReputation".to_string(),
        "FullScreenEffect".to_string(),
        "GMSurveyAnswers".to_string(),
        "GMSurveyCurrentSurvey".to_string(),
        "GMSurveyQuestions".to_string(),
        "GMSurveySurveys".to_string(),
        "GameObjectArtKit".to_string(),
        "GameObjectDiffAnimMap".to_string(),
        "GameObjectDisplayCondition".to_string(),
        "GameObjectDisplayInfo".to_string(),
        "GameObjectDisplayInfoXSoundKit".to_string(),
        "GameObjectLabel".to_string(),
        "GameObjects".to_string(),
        "GameTips".to_string(),
        "GarrAbility".to_string(),
        "GarrAbilityCategory".to_string(),
        "GarrAbilityEffect".to_string(),
        "GarrAutoSpell".to_string(),
        "GarrAutoSpellEffect".to_string(),
        "GarrBuilding".to_string(),
        "GarrBuildingDoodadSet".to_string(),
        "GarrBuildingPlotInst".to_string(),
        "GarrClassSpec".to_string(),
        "GarrClassSpecPlayerCond".to_string(),
        "GarrEncounter".to_string(),
        "GarrEncounterSetXEncounter".to_string(),
        "GarrEncounterXMechanic".to_string(),
        "GarrFollItemSetMember".to_string(),
        "GarrFollSupportSpell".to_string(),
        "GarrFollower".to_string(),
        "GarrFollowerLevelXP".to_string(),
        "GarrFollowerSetXFollower".to_string(),
        "GarrFollowerType".to_string(),
        "GarrFollowerUICreature".to_string(),
        "GarrFollowerXAbility".to_string(),
        "GarrItemLevelUpgradeData".to_string(),
        "GarrMechanic".to_string(),
        "GarrMechanicSetXMechanic".to_string(),
        "GarrMechanicType".to_string(),
        "GarrMission".to_string(),
        "GarrMissionType".to_string(),
        "GarrMissionXFollower".to_string(),
        "GarrMssnBonusAbility".to_string(),
        "GarrPlot".to_string(),
        "GarrPlotBuilding".to_string(),
        "GarrPlotInstance".to_string(),
        "GarrSiteLevel".to_string(),
        "GarrSiteLevelPlotInst".to_string(),
        "GarrSpecialization".to_string(),
        "GarrString".to_string(),
        "GarrTalTreeXGarrTalResearch".to_string(),
        "GarrTalent".to_string(),
        "GarrTalentMapPOI".to_string(),
        "GarrTalentRank".to_string(),
        "GarrTalentRankGroupEntry".to_string(),
        "GarrTalentRankGroupResearchMod".to_string(),
        "GarrTalentResearch".to_string(),
        "GarrTalentSocketProperties".to_string(),
        "GarrTalentTree".to_string(),
        "GarrType".to_string(),
        "GarrUiAnimClassInfo".to_string(),
        "GarrUiAnimRaceInfo".to_string(),
        "GemProperties".to_string(),
        "GlobalColor".to_string(),
        "GlobalGameContentTuning".to_string(),
        "GlobalPlayerCondition".to_string(),
        "GlobalPlayerConditionSet".to_string(),
        "GlobalStrings".to_string(),
        "GlideEvent".to_string(),
        "GlideEventBlendTimes".to_string(),
        "GlyphBindableSpell".to_string(),
        "GlyphExclusiveCategory".to_string(),
        "GlyphProperties".to_string(),
        "GlyphRequiredSpec".to_string(),
        "GossipNPCOption".to_string(),
        "GossipXGarrTalentTrees".to_string(),
        "GradientEffect".to_string(),
        "GroundEffectDoodad".to_string(),
        "GroundEffectTexture".to_string(),
        "GroupFinderActivity".to_string(),
        "GroupFinderActivityGrp".to_string(),
        "GroupFinderCategory".to_string(),
        "GuildColorBackground".to_string(),
        "GuildColorBorder".to_string(),
        "GuildEmblem".to_string(),
        "GuildPerkSpells".to_string(),
        "GuildShirtBackground".to_string(),
        "GuildShirtBorder".to_string(),
        "GuildTabardBackground".to_string(),
        "Heirloom".to_string(),
        "HelmetAnimScaling".to_string(),
        "HighlightColor".to_string(),
        "HolidayDescriptions".to_string(),
        "HolidayNames".to_string(),
        "Holidays".to_string(),
        "Hotfixes".to_string(),
        "ImportPriceArmor".to_string(),
        "ImportPriceQuality".to_string(),
        "ImportPriceShield".to_string(),
        "ImportPriceWeapon".to_string(),
        "InvasionClientData".to_string(),
        "Item".to_string(),
        "ItemAppearance".to_string(),
        "ItemAppearanceXUiCamera".to_string(),
        "ItemArmorQuality".to_string(),
        "ItemArmorShield".to_string(),
        "ItemArmorTotal".to_string(),
        "ItemBagFamily".to_string(),
        "ItemBonus".to_string(),
        "ItemBonusList".to_string(),
        "ItemBonusListGroup".to_string(),
        "ItemBonusListGroupEntry".to_string(),
        "ItemBonusListWarforgeLevelDelta".to_string(),
        "ItemBonusSeason".to_string(),
        "ItemBonusSeasonUpgradeCost".to_string(),
        "ItemBonusSequenceSpell".to_string(),
        "ItemBonusTree".to_string(),
        "ItemBonusTreeGroupEntry".to_string(),
        "ItemBonusTreeNode".to_string(),
        "ItemChildEquipment".to_string(),
        "ItemClass".to_string(),
        "ItemCondition".to_string(),
        "ItemConversion".to_string(),
        "ItemConversionEntry".to_string(),
        "ItemCurrencyCost".to_string(),
        "ItemCurrencyValue".to_string(),
        "ItemDamageAmmo".to_string(),
        "ItemDamageOneHand".to_string(),
        "ItemDamageOneHandCaster".to_string(),
        "ItemDamageTwoHand".to_string(),
        "ItemDamageTwoHandCaster".to_string(),
        "ItemDisenchantLoot".to_string(),
        "ItemDisplayInfo".to_string(),
        "ItemDisplayInfoMaterialRes".to_string(),
        "ItemDisplayInfoModelMatRes".to_string(),
        "ItemEffect".to_string(),
        "ItemExtendedCost".to_string(),
        "ItemFixup".to_string(),
        "ItemFixupAction".to_string(),
        "ItemGroupIlvlScalingEntry_zhCN".to_string(),
        "ItemGroupSounds".to_string(),
        "ItemLevelSelector".to_string(),
        "ItemLevelSelectorQuality".to_string(),
        "ItemLevelSelectorQualitySet".to_string(),
        "ItemLimitCategory".to_string(),
        "ItemLimitCategoryCondition".to_string(),
        "ItemLogicalCost".to_string(),
        "ItemModifiedAppearance".to_string(),
        "ItemModifiedAppearanceExtra".to_string(),
        "ItemNameDescription".to_string(),
        "ItemNameSlotOverride".to_string(),
        "ItemPetFood".to_string(),
        "ItemPriceBase".to_string(),
        "ItemRangedDisplayInfo".to_string(),
        "ItemRecraft".to_string(),
        "ItemSalvage".to_string(),
        "ItemSalvageLoot".to_string(),
        "ItemSearchName".to_string(),
        "ItemSet".to_string(),
        "ItemSetSpell".to_string(),
        "ItemSparse".to_string(),
        "ItemSpec".to_string(),
        "ItemSpecOverride".to_string(),
        "ItemSubClass".to_string(),
        "ItemSubClassMask".to_string(),
        "ItemVisuals".to_string(),
        "ItemVisualsXEffect".to_string(),
        "ItemXBonusTree".to_string(),
        "ItemXItemEffect".to_string(),
        "JournalEncounter".to_string(),
        "JournalEncounterCreature".to_string(),
        "JournalEncounterItem".to_string(),
        "JournalEncounterSection".to_string(),
        "JournalEncounterXDifficulty".to_string(),
        "JournalEncounterXMapLoc".to_string(),
        "JournalInstance".to_string(),
        "JournalInstanceEntrance".to_string(),
        "JournalItemXDifficulty".to_string(),
        "JournalTier".to_string(),
        "JournalTierXInstance".to_string(),
        "Keychain".to_string(),
        "KeystoneAffix".to_string(),
        "LFGDungeons".to_string(),
        "LFGDungeonsGroupingMap".to_string(),
        "LFGRoleRequirement".to_string(),
        "LabelXContentRestrictRuleSet".to_string(),
        "Languages".to_string(),
        "Languages_zhCN".to_string(),
        "LanguageWords".to_string(),
        "Light".to_string(),
        "LightData".to_string(),
        "LightParams".to_string(),
        "LightParamsLightShaft".to_string(),
        "LightShaft".to_string(),
        "LightSkybox".to_string(),
        "LightWorldShadow".to_string(),
        "Lightning".to_string(),
        "LiquidMaterial".to_string(),
        "LiquidObject".to_string(),
        "LiquidType".to_string(),
        "LiquidTypeXTexture".to_string(),
        "LiveEvent".to_string(),
        "LoadingScreenSkin".to_string(),
        "LoadingScreenTaxiSplines".to_string(),
        "LoadingScreens".to_string(),
        "Lock".to_string(),
        "LockType".to_string(),
        "LookAtController".to_string(),
        "LoreText".to_string(),
        "MCRSlotXMCRCategory".to_string(),
        "MailTemplate".to_string(),
        "ManagedWorldState".to_string(),
        "ManagedWorldStateBuff".to_string(),
        "ManagedWorldStateInput".to_string(),
        "ManifestInterfaceData".to_string(),
        "ManifestInterfaceItemIcon".to_string(),
        "ManifestInterfaceTOCData".to_string(),
        "Map".to_string(),
        "MapChallengeMode".to_string(),
        "MapDifficulty".to_string(),
        "MapDifficultyXCondition".to_string(),
        "MapLoadingScreen".to_string(),
        "MapRenderScale".to_string(),
        "MarketingPromotionsXLocale".to_string(),
        "Material".to_string(),
        "MawPowerRarity".to_string(),
        "MinorTalent".to_string(),
        "MissileTargeting".to_string(),
        "ModelFileData".to_string(),
        "ModelRibbonQuality".to_string(),
        "ModifiedCraftingCategory".to_string(),
        "ModifiedCraftingItem".to_string(),
        "ModifiedCraftingReagentItem".to_string(),
        "ModifiedCraftingReagentSlot".to_string(),
        "ModifiedCraftingSpellSlot".to_string(),
        "ModifierTree".to_string(),
        "Mount".to_string(),
        "MountCapability".to_string(),
        "MountEquipment".to_string(),
        "MountType".to_string(),
        "MountTypeXCapability".to_string(),
        "MountXDisplay".to_string(),
        "MountXSpellVisualKitPicker".to_string(),
        "Movie".to_string(),
        "MovieFileData".to_string(),
        "MovieVariation".to_string(),
        "MultiStateProperties".to_string(),
        "MultiTransitionProperties".to_string(),
        "MusicOverride".to_string(),
        "MythicPlusSeason".to_string(),
        "MythicPlusSeasonRewardLevels".to_string(),
        "MythicPlusSeasonTrackedAffix".to_string(),
        "MythicPlusSeasonTrackedMap".to_string(),
        "NPCCraftingOrderSet".to_string(),
        "NPCCraftingOrderSetXCraftOrder".to_string(),
        "NPCCraftingOrderSetXCustomer".to_string(),
        "NPCModelItemSlotDisplayInfo".to_string(),
        "NPCSounds".to_string(),
        "NameGen".to_string(),
        "NamesProfanity".to_string(),
        "NamesReserved".to_string(),
        "NamesReservedLocale".to_string(),
        "NumTalentsAtLevel".to_string(),
        "ObjectEffect".to_string(),
        "ObjectEffectGroup".to_string(),
        "ObjectEffectModifier".to_string(),
        "ObjectEffectPackageElem".to_string(),
        "Occluder".to_string(),
        "OccluderCurtain".to_string(),
        "OccluderNode".to_string(),
        "OutlineEffect".to_string(),
        "OverrideSpellData".to_string(),
        "PVPBracketTypes".to_string(),
        "PVPDifficulty".to_string(),
        "PVPItem".to_string(),
        "PVPScoreboardCellInfo".to_string(),
        "PVPScoreboardColumnHeader".to_string(),
        "PVPScoreboardLayout".to_string(),
        "PVPStat".to_string(),
        "PageTextMaterial".to_string(),
        "ParticleColor".to_string(),
        "Particulate".to_string(),
        "Path".to_string(),
        "PathEdge".to_string(),
        "PathNode".to_string(),
        "PathNodeProperty".to_string(),
        "PathProperty".to_string(),
        "PerksActivity".to_string(),
        "PerksActivityCondition".to_string(),
        "PerksActivityTag".to_string(),
        "PerksActivityXHolidays".to_string(),
        "PerksActivityXInterval".to_string(),
        "PerksActivityXTag".to_string(),
        "PerksUITheme".to_string(),
        "PerksVendorCategory".to_string(),
        "PerksVendorItem".to_string(),
        "PerksVendorItemUIGroup".to_string(),
        "PerksVendorItemXInterval".to_string(),
        "Phase".to_string(),
        "PhaseShiftZoneSounds".to_string(),
        "PhaseXPhaseGroup".to_string(),
        "PingType".to_string(),
        "PlayerCondition".to_string(),
        "PlayerCompanionInfo".to_string(),
        "PlayerDataFlagAccount".to_string(),
        "PlayerDataFlagCharacter".to_string(),
        "PlayerInteractionInfo".to_string(),
        "PointLightConditionMap".to_string(),
        "Positioner".to_string(),
        "PositionerState".to_string(),
        "PositionerStateEntry".to_string(),
        "PowerDisplay".to_string(),
        "PowerType".to_string(),
        "PrestigeLevelInfo".to_string(),
        "ProfTraitPerkNode".to_string(),
        "ProfTraitPathNode".to_string(),
        "ProfTraitTree".to_string(),
        "ProfTraitTreeHighlight".to_string(),
        "ProfessionEffect".to_string(),
        "ProfessionEffectType".to_string(),
        "ProfessionExpansion_zhCN".to_string(),
        "ProfessionPropPoints".to_string(),
        "ProfessionRating".to_string(),
        "ProfessionTrait".to_string(),
        "ProfessionTraitXEffect".to_string(),
        "ProfessionXRating".to_string(),
        "PvpBrawl".to_string(),
        "PvpScalingEffect".to_string(),
        "PvpSeason".to_string(),
        "PvpSeasonRewardLevels".to_string(),
        "PvpTalent".to_string(),
        "PvpTalentCategory".to_string(),
        "PvpTalentSlotUnlock".to_string(),
        "PvpTier".to_string(),
        "QuestFactionReward".to_string(),
        "QuestFeedbackEffect".to_string(),
        "QuestInfo".to_string(),
        "QuestLine".to_string(),
        "QuestLineXQuest".to_string(),
        "QuestMoneyReward".to_string(),
        "QuestObjective".to_string(),
        "QuestPOIBlob".to_string(),
        "QuestPOIPoint".to_string(),
        "QuestSort".to_string(),
        "QuestV2".to_string(),
        "QuestV2CliTask".to_string(),
        "QuestXP".to_string(),
        "QuestXUIQuestDetailsTheme".to_string(),
        "RTPCData".to_string(),
        "RafActivity".to_string(),
        "ResearchBranch".to_string(),
        "ResearchField".to_string(),
        "ResearchProject".to_string(),
        "ResearchSite".to_string(),
        "Resistances".to_string(),
        "RewardPack".to_string(),
        "RewardPackXCurrencyType".to_string(),
        "RibbonQuality".to_string(),
        "RopeEffect".to_string(),
        "RuneforgeLegendaryAbility".to_string(),
        "SSAOSettings".to_string(),
        "ScalingStatDistribution".to_string(),
        "Scenario".to_string(),
        "ScenarioEventEntry".to_string(),
        "ScenarioStep".to_string(),
        "SceneScript".to_string(),
        "SceneScriptPackage".to_string(),
        "SceneScriptPackageMember".to_string(),
        "SceneScriptText".to_string(),
        "ScreenEffect".to_string(),
        "ScreenEffectType".to_string(),
        "ScreenLocation".to_string(),
        "SeamlessSite".to_string(),
        "ServerMessages".to_string(),
        "SharedString".to_string(),
        "SiegeableProperties".to_string(),
        "SkillLine".to_string(),
        "SkillLineAbility".to_string(),
        "SkillRaceClassInfo".to_string(),
        "SkySceneXPlayerCondition".to_string(),
        "SoundAmbienceFlavor".to_string(),
        "SoundBus".to_string(),
        "SoundBusOverride".to_string(),
        "SoundEmitterPillPoints".to_string(),
        "SoundEmitters".to_string(),
        "SoundEnvelope".to_string(),
        "SoundFilter".to_string(),
        "SoundFilterElem".to_string(),
        "SoundKit".to_string(),
        "SoundKitAdvanced".to_string(),
        "SoundKitChild".to_string(),
        "SoundKitEntry".to_string(),
        "SoundKitFallback".to_string(),
        "SoundMixGroup".to_string(),
        "SoundOverride".to_string(),
        "SoundParameter".to_string(),
        "SoundProviderPreferences".to_string(),
        "SourceInfo".to_string(),
        "Soulbind".to_string(),
        "SoulbindConduit".to_string(),
        "SoulbindConduitEnhancedSocket".to_string(),
        "SoulbindConduitItem".to_string(),
        "SoulbindConduitRank".to_string(),
        "SoulbindConduitRankProperties".to_string(),
        "SoulbindUIDisplayInfo".to_string(),
        "SpamMessages".to_string(),
        "SpecSetMember".to_string(),
        "SpecializationSpells".to_string(),
        "SpecializationSpellsDisplay".to_string(),
        "Spell".to_string(),
        "SpellActivationOverlay".to_string(),
        "SpellAuraOptions".to_string(),
        "SpellAuraRestrictions".to_string(),
        "SpellAuraVisXChrSpec".to_string(),
        "SpellAuraVisibility".to_string(),
        "SpellCastTimes".to_string(),
        "SpellCastingRequirements".to_string(),
        "SpellCategories".to_string(),
        "SpellCategory".to_string(),
        "SpellChainEffects".to_string(),
        "SpellClassOptions".to_string(),
        "SpellClutterAreaEffectCounts".to_string(),
        "SpellClutterFrameRates".to_string(),
        "SpellClutterImpactModelCounts".to_string(),
        "SpellClutterMissileDist".to_string(),
        "SpellClutterWeaponTrailDist".to_string(),
        "SpellCooldowns".to_string(),
        "SpellDescriptionVariables".to_string(),
        "SpellDuration".to_string(),
        "SpellEffect".to_string(),
        "SpellEffectEmission".to_string(),
        "SpellEmpower".to_string(),
        "SpellEmpowerStage".to_string(),
        "SpellEquippedItems".to_string(),
        "SpellFlyout".to_string(),
        "SpellFlyoutItem".to_string(),
        "SpellFocusObject".to_string(),
        "SpellKeyboundOverride".to_string(),
        "SpellLearnSpell".to_string(),
        "SpellLevels".to_string(),
        "SpellMisc".to_string(),
        "SpellMissile".to_string(),
        "SpellMissileMotion".to_string(),
        "SpellName".to_string(),
        "SpellOverrideName".to_string(),
        "SpellPower".to_string(),
        "SpellPowerDifficulty".to_string(),
        "SpellProceduralEffect".to_string(),
        "SpellProcsPerMinute".to_string(),
        "SpellProcsPerMinuteMod".to_string(),
        "SpellRadius".to_string(),
        "SpellRange".to_string(),
        "SpellReagents".to_string(),
        "SpellReagentsCurrency".to_string(),
        "SpellReplacement".to_string(),
        "SpellScaling".to_string(),
        "SpellScript".to_string(),
        "SpellShapeshift".to_string(),
        "SpellShapeshiftForm".to_string(),
        "SpellSpecialUnitEffect".to_string(),
        "SpellTargetRestrictions".to_string(),
        "SpellTotems".to_string(),
        "SpellVisual".to_string(),
        "SpellVisualAnim".to_string(),
        "SpellVisualColorEffect".to_string(),
        "SpellVisualEffectName".to_string(),
        "SpellVisualEvent".to_string(),
        "SpellVisualKit".to_string(),
        "SpellVisualKitAreaModel".to_string(),
        "SpellVisualKitEffect".to_string(),
        "SpellVisualKitModelAttach".to_string(),
        "SpellVisualKitPicker".to_string(),
        "SpellVisualKitPickerEntry".to_string(),
        "SpellVisualMissile".to_string(),
        "SpellXDescriptionVariables".to_string(),
        "SpellXSpellVisual".to_string(),
        "Startup_Strings".to_string(),
        "Stationery".to_string(),
        "SummonProperties".to_string(),
        "TactKey".to_string(),
        "TactKeyLookup".to_string(),
        "Talent".to_string(),
        "TaxiNodes".to_string(),
        "TaxiPath".to_string(),
        "TerrainColorGradingRamp".to_string(),
        "TerrainMaterial".to_string(),
        "TerrainType".to_string(),
        "TerrainTypeSounds".to_string(),
        "TextureFileData".to_string(),
        "TierTransition".to_string(),
        "TotemCategory".to_string(),
        "Toy".to_string(),
        "TradeSkillCategory".to_string(),
        "TraitCost".to_string(),
        "TraitCurrency".to_string(),
        "TraitCurrencySource".to_string(),
        "TraitDefinition".to_string(),
        "TraitDefinitionEffectPoints".to_string(),
        "TraitEdge".to_string(),
        "TraitNode".to_string(),
        "TraitNodeEntry".to_string(),
        "TraitNodeEntryXTraitCond".to_string(),
        "TraitNodeEntryXTraitCost".to_string(),
        "TraitNodeGroup".to_string(),
        "TraitNodeGroupXTraitCond".to_string(),
        "TraitNodeGroupXTraitCost".to_string(),
        "TraitNodeXTraitCond".to_string(),
        "TraitNodeXTraitCost".to_string(),
        "TraitNodeXTraitNodeEntry".to_string(),
        "TraitSubTree".to_string(),
        "TraitTree".to_string(),
        "TraitTreeLoadout".to_string(),
        "TraitTreeLoadoutEntry".to_string(),
        "TraitTreeXTraitCurrency".to_string(),
        "TransformMatrix".to_string(),
        "TransmogDefaultLevel".to_string(),
        "TransmogHoliday".to_string(),
        "TransmogIllusion".to_string(),
        "TransmogSet".to_string(),
        "TransmogSetGroup".to_string(),
        "TransmogSetItem".to_string(),
        "TransportAnimation".to_string(),
        "TransportPhysics".to_string(),
        "TransportRotation".to_string(),
        "Trophy".to_string(),
        "UIChromieTimeExpansionInfo".to_string(),
        "UICinematicIntroInfo".to_string(),
        "UICovenantAbility".to_string(),
        "UICovenantPreview".to_string(),
        "UIDungeonScoreRarity".to_string(),
        "UIEventToast".to_string(),
        "UIExpansionDisplayInfoIcon".to_string(),
        "UIGenericWidgetDisplay".to_string(),
        "UIModifiedInstance".to_string(),
        "UIQuestDetailsTheme".to_string(),
        "UIScriptedAnimationEffect".to_string(),
        "UiCamera".to_string(),
        "UiCameraType".to_string(),
        "UiCanvas".to_string(),
        "UiMap".to_string(),
        "UiMapArt".to_string(),
        "UiMapArtStyleLayer".to_string(),
        "UiMapAssignment".to_string(),
        "UiMapFogOfWar".to_string(),
        "UiMapFogOfWarVisualization".to_string(),
        "UiMapGroup".to_string(),
        "UiMapGroupMember".to_string(),
        "UiMapLink".to_string(),
        "UiMapXMapArt".to_string(),
        "UiModelScene".to_string(),
        "UiModelSceneActor".to_string(),
        "UiModelSceneActorDisplay".to_string(),
        "UiModelSceneCamera".to_string(),
        "UiPartyPose".to_string(),
        "UiTextureAtlas".to_string(),
        "UiTextureAtlasElement".to_string(),
        "UiTextureAtlasElementSliceData".to_string(),
        "UiTextureAtlasMember".to_string(),
        "UiTextureKit".to_string(),
        "UiWeeklyReward".to_string(),
        "UiWidgetConstantSource".to_string(),
        "UiWidgetSet".to_string(),
        "UiWidgetStringSource".to_string(),
        "UiWidgetVisTypeDataReq".to_string(),
        "UiWidgetXWidgetSet".to_string(),
        "UnitBloodLevels".to_string(),
        "UnitCondition".to_string(),
        "UnitPowerBar".to_string(),
        "Vehicle".to_string(),
        "VehiclePOIType".to_string(),
        "VehicleSeat".to_string(),
        "VehicleUIIndicator".to_string(),
        "VehicleUIIndSeat".to_string(),
        "VignetteUiWidgetSet".to_string(),
        "VirtualAttachment".to_string(),
        "VirtualAttachmentCustomization".to_string(),
        "VocalUISounds".to_string(),
        "VolumeFogCondition".to_string(),
        "WMOAreaTable".to_string(),
        "WMOMinimapTexture".to_string(),
        "WarbandScene".to_string(),
        "WaypointEdge".to_string(),
        "WaypointNode".to_string(),
        "WaypointSafeLocs".to_string(),
        "WbAccessControlList".to_string(),
        "WeaponImpactSounds".to_string(),
        "WeaponSwingSounds2".to_string(),
        "WeaponTrail".to_string(),
        "WeaponTrailModelDef".to_string(),
        "WeaponTrailParam".to_string(),
        "Weather".to_string(),
        "WeatherXParticulate".to_string(),
        "WeeklyRewardChestActivityTier".to_string(),
        "WeeklyRewardChestThreshold".to_string(),
        "WindSettings".to_string(),
        "WorldBossLockout".to_string(),
        "WorldChunkSounds".to_string(),
        "WorldEffect".to_string(),
        "WorldMapOverlay".to_string(),
        "WorldSafeLocs".to_string(),
        "WorldShadow".to_string(),
        "WorldStateExpression".to_string(),
        "WorldStateZoneSounds".to_string(),
        "World_PVP_Area".to_string(),
        "ZoneIntroMusicTable".to_string(),
        "ZoneLight".to_string(),
        "ZoneMusic".to_string(),
        "ZoneStory".to_string()
    ])
}

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub has_locale: bool,
}

#[derive(Debug)]
pub struct TableCollection {
    tables: HashSet<Table>,
}

impl TableCollection {
    pub fn new() -> Self {
        let tables = crate::config::constants::AVAILABLE_TABLES
            .iter()
            .map(|&name| Table {
                name: name.to_string(),
                has_locale: true,
            })
            .collect();

        Self { tables }
    }

    pub fn get_all(&self) -> &HashSet<Table> {
        &self.tables
    }

    pub fn contains(&self, table_name: &str) -> bool {
        self.tables.iter().any(|t| t.name == table_name)
    }

    pub fn get_localized_tables(&self) -> impl Iterator<Item = &Table> {
        self.tables.iter().filter(|t| t.has_locale)
    }
}