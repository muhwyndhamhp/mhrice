use super::*;
use crate::rsz_bitflags;
use crate::rsz_enum;
use crate::rsz_struct;
use crate::rsz_with_singleton;
use serde::*;
use std::cmp::*;

// snow.quest.QuestType
rsz_bitflags! {
    pub struct QuestType: u32 {
        const HUNTING  = 0x00000001;
        const KILL     = 0x00000002;
        const CAPTURE  = 0x00000004;
        const BOSSRUSH = 0x00000008;
        const COLLECTS = 0x00000010;
        const TOUR     = 0x00000020;
        const ARENA    = 0x00000040;
        const SPECIAL  = 0x00000080;
        const HYAKURYU = 0x00000100;
        const TRAINING = 0x00000200;
        const KYOUSEI  = 0x00000400;
    }
}

impl QuestType {
    pub fn icon_index(&self) -> u32 {
        if self.contains(QuestType::HUNTING) {
            return 0;
        }
        if self.contains(QuestType::KILL) {
            return 1;
        }
        if self.contains(QuestType::CAPTURE) {
            return 2;
        }
        if self.contains(QuestType::BOSSRUSH) {
            return 6;
        }
        if self.contains(QuestType::COLLECTS) {
            return 3;
        }
        if self.contains(QuestType::TOUR) {
            return 7;
        }
        if self.contains(QuestType::ARENA) {
            return 4;
        }
        if self.contains(QuestType::SPECIAL) {
            return 5;
        }
        if self.contains(QuestType::HYAKURYU) {
            return 8;
        }
        if self.contains(QuestType::TRAINING) {
            return 0;
        }

        0
    }
}

// snow.quest.QuestLevel
rsz_enum! {
    #[rsz(i32)]
    #[allow(clippy::upper_case_acronyms)]
    #[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub enum QuestLevel {
        QL1 = 0,
        QL2 = 1,
        QL3 = 2,
        QL4 = 3,
        QL5 = 4,
        QL6 = 5,
        QL7 = 6,
        QL7Ex = 7,
    }
}

// snow.quest.EnemyLv
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub enum EnemyLevel {
        Village = 0,
        Low = 1,
        High = 2,
        Master = 3,
    }
}

// snow.quest.QuestOrderType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, Eq, PartialEq)]
    pub enum QuestOrderType {
        None = 0,
        Under2 = 1,
        H2 = 2,
        H3 = 3,
        H4 = 4,
        H5 = 5,
        H6 = 6,
        H7 = 7,
        H8 = 8,
        H20 = 9,
        H30 = 10,
        H40 = 11,
        H45 = 12,
        H50 = 13,
        H90 = 14,
        H100 = 15,
        M1 = 16,
        M2 = 17,
        M3 = 18,
        M4 = 19,
        M5 = 20,
        M6 = 21,
        M10 = 22,
        M20 = 23,
        M30 = 24,
        M40 = 25,
        M50 = 26,
        M60 = 27,
        M100 = 28,
        Only1 = 29,
    }
}

// snow.quest.QuestTargetType
rsz_enum! {
    #[rsz(u8)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum QuestTargetType {
        None = 0,
        ItemGet = 1,
        Hunting = 2,
        Kill = 3,
        Capture = 4,
        AllMainEnemy = 5,
        EmTotal = 6,
        FinalBarrierDefense = 7,
        FortLevelUp = 8,
        PlayerDown = 9,
        FinalBoss = 10,
        HuntingMachine = 11,
        DropItem = 12,
        EmStun = 13,
        EmElement = 14,
        EmCondition = 15,
        EmCntWeapon = 16,
        EmCntHmBallista = 17,
        EmCntHmCannon = 18,
        EmCntHmGatling = 19,
        EmCntHmTrap = 20,
        EmCntHmFlameThrower = 21,
        EmCntHmNpc = 22,
        EmCntHmDragnator = 23,
        ExtraEmRunaway = 24,
    }
}

// snow.QuestManager.BossSetCondition
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum BossSetCondition {
        None = 0,
        Default = 1,
        Free1 = 2,
        Free2 = 3,
        Free3 = 4,
        Timer1 = 5,
        Timer2 = 6,
        Em1Hp = 7,
        Em2Hp = 8,
        Em3Hp = 9,
        Em4Hp = 10,
        Em5Hp = 11,
        HpEmx1 = 12,
        HpEmx2 = 13,
        InitRandom = 14,
        SwapRandom = 15,
        FsmControl = 16,
        EntryTime = 17,
    }
}

// snow.QuestManager.SwapSetCondition
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum SwapSetCondition {
        None = 0,
        QuestTimer = 1,
    }
}

// snow.QuestManager.SwapStopType
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum SwapStopType {
        None = 0,
        LowerHp = 1,
    }
}

// snow.QuestManager.SwapExecType
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum SwapExecType {
        None = 0,
        FreeExtra = 1,
    }
}

// snow.quest.BattleBGMType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum BattleBgmType {
        Default = 0,
        C01 = 1,
        C02 = 2,
        C03 = 3,
        C04 = 4,
        C05 = 5,
        C06 = 6,
        C07 = 7,
        Sp01 = 8,
    }
}

// snow.quest.ClearBGMType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum ClearBgmType {
        Default = 0,
        SpClear01 = 1,
    }
}

// snow.enemy.EnemyDef.EmTypes
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum EmTypes {
        Em(u32) = 0x0000..=0x0FFF,
        Ems(u32) = 0x1000..=0x1FFF,
    }
}

impl EmTypes {
    fn order_index(&self) -> u32 {
        match *self {
            EmTypes::Em(i) => (i & 0xFF) << 16 | (i & 0xF00),
            EmTypes::Ems(i) => (i & 0xFF) << 16 | (i & 0xF00) | 0x80000000,
        }
    }
}

impl PartialOrd<EmTypes> for EmTypes {
    fn partial_cmp(&self, other: &EmTypes) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EmTypes {
    fn cmp(&self, other: &EmTypes) -> Ordering {
        self.order_index().cmp(&other.order_index())
    }
}

rsz_struct! {
    #[rsz("snow.quest.NormalQuestData.Param",
        0x708b71d8 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct NormalQuestDataParam {
        pub quest_no: i32,
        pub dbg_name: String,
        pub quest_type: QuestType,
        pub quest_level: QuestLevel,
        pub enemy_level: EnemyLevel,
        pub map_no: i32, // snow.QuestMapManager.MapNoType
        pub base_time: u32,
        pub time_variation: u32,
        pub time_limit: u32,
        pub quest_life: u32,
        pub order_type: Vec<QuestOrderType>,
        pub target_type: Vec<QuestTargetType>,
        pub tgt_em_type: Vec<EmTypes>,
        pub tgt_item_id: Vec<ItemId>,
        pub tgt_num: Vec<u32>,
        pub boss_em_type: Vec<EmTypes>,
        pub init_extra_em_num: u8,
        pub swap_em_rate: Vec<u8>,
        pub boss_set_condition: Vec<BossSetCondition>,
        pub boss_set_param: Vec<u32>,
        pub swap_set_condition: Vec<SwapSetCondition>,
        pub swap_set_param: Vec<u8>,
        pub swap_exit_time: Vec<u8>,
        pub is_swap_exit_marionette: bool,
        pub swap_stop_type: SwapStopType,
        pub swap_stop_param: u8,
        pub swap_exec_type: SwapExecType,
        pub rem_money: u32,
        pub rem_village_point: u32,
        pub rem_rank_point: u32,
        pub supply_tbl: u32,
        pub icon: Vec<i32>, // TODO: snow.gui.SnowGuiCommonUtility.Icon.EnemyIconFrameForQuestOrder
        pub is_from_npc: bool,
        pub is_tutorial: bool,
        pub fence_active_sec: u16,
        pub fence_default_active: bool,
        pub fence_default_wait_sec: u16,
        pub fence_reload_sec: u16,
        pub is_use_pillar: Vec<bool>,
        pub battle_bgm_type: BattleBgmType,
        pub clear_bgm_type: ClearBgmType,
        pub auto_match_hr: u16,
        pub dbg_client:String,
        pub dbg_content: String,
    }
}

impl NormalQuestDataParam {
    pub fn has_target(&self, em_type: EmTypes) -> bool {
        if self.target_type.get(0) == Some(&QuestTargetType::AllMainEnemy) {
            let count = if let Some(&count) = self.tgt_num.get(0) {
                count as usize
            } else {
                eprintln!("tgt_num empty for quest {}", self.quest_no);
                return false;
            };

            let targets = if let Some(targets) = self.boss_em_type.get(0..count) {
                targets
            } else {
                eprintln!(
                    "tgt_num is larget than boss_em_type len for quest {}",
                    self.quest_no
                );
                return false;
            };

            return targets.contains(&em_type);
        }

        self.tgt_em_type.contains(&em_type)
    }
}

rsz_struct! {
    #[rsz("snow.quest.NormalQuestData",
        0x299bae19 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct NormalQuestData {
        pub param: Vec<NormalQuestDataParam>,
    }
}

rsz_with_singleton! {
    #[path("Quest/QuestData/NormalQuestData.user")]
    pub struct BaseNormalQuestDataLrHr(NormalQuestData);

    #[path("Quest/QuestData/NormalQuestData_MR.user")]
    pub struct BaseNormalQuestDataMr(NormalQuestData);

    #[path("Quest/QuestData/DlQuestData.user")]
    pub struct DlNormalQuestData(NormalQuestData);
}

/*rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum EmsSetNo {
        None = 0,
        M01Base = 1,
        M01Tour = 2,
        M01Tutorial = 3,
        M02Base = 4,
        M02Tour = 5,
        M02Qn000313 = 6,
        M02Qn000412 = 7,
        M02Qn010212 = 8,
        M02Qn010516 = 9,
        M03Base  = 10,
        M03Tour  = 11,
        M03Qn000315  = 12,
        M03Qn010213  = 13,
        M03Qn010517  = 14,
        M04Base  = 15,
        M04Tour  = 16,
        M04Qn000205  = 17,
        M04Qn000209  = 18,
        M04Qn010112  = 19,
        M04Qn010412  = 20,
        M05Base  = 21,
        M05Tour  = 22,
        M01Qn000104  = 23,
        M01Qn000105  = 24,
        M01Qn000208  = 25,
        M01Qn00310  = 26,
        M05Qn000410  = 27,
        M05Qn000414  = 28,
        M01Qn010111  = 29,
        M01Qn010418  = 30,
        M05Qn010617  = 31,
    }
}*/

// snow.enemy.EnemyDef.NandoYuragi
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum NandoYuragi {
        False = 0,
        True1 = 1,
        True2 = 2,
    }
}

// snow.enemy.EnemyDef.EnemyIndividualType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum EnemyIndividualType {
        Normal = 0,
        Mystery = 1,
    }
}

pub trait EnemyParam {
    fn sub_type(&self, i: usize) -> Option<u8>;
    fn vital_tbl(&self, i: usize) -> Option<u16>;
    fn attack_tbl(&self, i: usize) -> Option<u16>;
    fn parts_tbl(&self, i: usize) -> Option<u16>;
    fn other_tbl(&self, i: usize) -> Option<u16>;
    fn stamina_tbl(&self, i: usize) -> Option<u8>;
    fn scale(&self, i: usize) -> Option<u8>;
    fn scale_tbl(&self, i: usize) -> Option<i32>;
    fn difficulty(&self, i: usize) -> Option<NandoYuragi>;
    fn boss_multi(&self, i: usize) -> Option<u8>;
}

macro_rules! impl_enemy_param {
    ($t:ty) => {
        impl EnemyParam for $t {
            fn sub_type(&self, i: usize) -> Option<u8> {
                self.sub_type.get(i).copied()
            }
            fn vital_tbl(&self, i: usize) -> Option<u16> {
                self.vital_tbl.get(i).copied().map(Into::into)
            }
            fn attack_tbl(&self, i: usize) -> Option<u16> {
                self.attack_tbl.get(i).copied().map(Into::into)
            }
            fn parts_tbl(&self, i: usize) -> Option<u16> {
                self.parts_tbl.get(i).copied().map(Into::into)
            }
            fn other_tbl(&self, i: usize) -> Option<u16> {
                self.other_tbl.get(i).copied().map(Into::into)
            }
            fn stamina_tbl(&self, i: usize) -> Option<u8> {
                self.stamina_tbl.get(i).copied()
            }
            fn scale(&self, i: usize) -> Option<u8> {
                self.scale.get(i).copied()
            }
            fn scale_tbl(&self, i: usize) -> Option<i32> {
                self.scale_tbl.get(i).copied()
            }
            fn difficulty(&self, i: usize) -> Option<NandoYuragi> {
                self.difficulty.get(i).copied()
            }
            fn boss_multi(&self, i: usize) -> Option<u8> {
                self.boss_multi.get(i).copied()
            }
        }
    };
}

rsz_struct! {
    #[rsz("snow.quest.NormalQuestDataForEnemy.Param",
        0x7E1E92C8 = 10_00_02
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct NormalQuestDataForEnemyParam {
        pub quest_no: i32,
        pub ems_set_no: i32, //EmsSetNo,
        pub zako_vital: u8,
        pub zako_attack: u8,
        pub zako_parts: u8,
        pub zako_other: u8,
        pub zako_multi: u8,
        pub route_no: Vec<u8>,
        pub init_set_name: Vec<String>,
        pub individual_type: Vec<EnemyIndividualType>,
        pub sub_type: Vec<u8>,
        pub vital_tbl: Vec<u16>,
        pub attack_tbl: Vec<u16>,
        pub parts_tbl: Vec<u16>,
        pub other_tbl: Vec<u16>,
        pub stamina_tbl: Vec<u8>,
        pub scale: Vec<u8>,
        pub scale_tbl: Vec<i32>, // snow.enemy.EnemyDef.BossScaleTblType
        pub difficulty: Vec<NandoYuragi>,
        pub boss_multi: Vec<u8>,
    }
}

impl_enemy_param!(NormalQuestDataForEnemyParam);

rsz_struct! {
    #[rsz("snow.quest.NormalQuestDataForEnemy",
        path = "Quest/QuestData/NormalQuestDataForEnemy.user",
        0xd1f4bc61 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct NormalQuestDataForEnemy {
        pub param: Vec<NormalQuestDataForEnemyParam>,
    }
}

rsz_with_singleton! {
    #[path("Quest/QuestData/NormalQuestDataForEnemy.user")]
    pub struct BaseNormalQuestDataForEnemyLrHr(NormalQuestDataForEnemy);

    #[path("Quest/QuestData/NormalQuestDataForEnemy_MR.user")]
    pub struct BaseNormalQuestDataForEnemyMr(NormalQuestDataForEnemy);

    #[path("Quest/QuestData/DlQuestDataForEnemy.user")]
    pub struct DlNormalQuestDataForEnemy(NormalQuestDataForEnemy);
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.VitalRateTableData",
        0x11ea886d = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct VitalRateTableData {
        pub vital_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.AttackRateTableData",
        0xa9a1e8ba = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct AttackRateTableData {
        pub attack_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.PartsRateTableData",
        0x0c501c3d = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct PartsRateTableData {
        pub parts_vital_rate: f32,
        pub mystery_core_vital_rate: f32
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.OtherRateTableData",
        0x334ea69a = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct OtherRateTableData {
        pub defense_rate: f32,
        pub damage_element_rate_a: f32,
        pub damage_element_rate_b: f32,
        pub stun_rate: f32,
        pub tired_rate: f32,
        pub paralyze_rate: f32,
        pub sleep_rate: f32,
        pub marionette_rate: f32,
        pub damage_element_first_rate_a: f32,
        pub damage_element_first_rate_b: f32,
        pub stun_first_rate: f32,
        pub tired_first_rate: f32,
        pub paralyze_first_rate: f32,
        pub sleep_first_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.MultiRateTableData.MultiData",
        0x4efdc07a = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MultiData {
        pub two: f32,
        pub three: f32,
        pub four: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.MultiRateTableData",
        0xe9130b0b = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct MultiRateTableData {
        pub multi_data_list: [MultiData; 13],
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData",
        path = "enemy/user_data/system_difficulty_rate_data.user",
        0xC776EEC0 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct SystemDifficultyRateData {
        pub vital_rate_table_list: Vec<VitalRateTableData>,
        pub attack_rate_table_list: Vec<AttackRateTableData>,
        pub parts_rate_table_list: Vec<PartsRateTableData>,
        pub other_rate_table_list: Vec<OtherRateTableData>,
        pub multi_rate_table_list: Vec<MultiRateTableData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossRandomScaleData.ScaleAndRateData",
        0x72b0bab3 = 0
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct ScaleAndRateData {
        pub scale: f32,
        pub rate: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossRandomScaleData.RandomScaleTableData",
        0xB0D72295 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomScaleTableData {
        pub type_: i32, // snow.enemy.EnemyDef.BossScaleTblType
        pub scale_and_rate_data: Vec<ScaleAndRateData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossRandomScaleData",
        path = "enemy/user_data/system_boss_random_scale_data.user",
        0xc45db706 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyBossRandomScaleData {
        pub random_scale_table_data_list: Vec<RandomScaleTableData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemEnemySizeListData.SizeInfo",
        0xB66C1F4D = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct SizeInfo {
        pub em_type: EmTypes,
        pub base_size: f32,
        pub small_boarder: f32,
        pub big_boarder: f32,
        pub king_boarder: f32,
        pub no_size_scale: bool,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemEnemySizeListData",
        path = "enemy/user_data/system_enemy_sizelist_data.user",
        0xab121e9c = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemySizeListData {
        pub size_info_list: Vec<SizeInfo>,
    }
}

rsz_struct! {
    #[rsz("snow.quest.DiscoverEmSetData.Param",
        0xa9f8ec2d = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct DiscoverEmSetDataParam {
        pub em_type: EmTypes,
        pub cond_village: VillageProgress,
        pub cond_low: i32, // snow.progress.HallProgress
        pub cond_high: i32, // snow.progress.HallProgress
        pub cond_master: i32, // snow.progress.MasterRankProgress
        pub map_flag: [bool; 7],
        pub route_no: [u8; 7],
        pub init_set_name: [String; 7],
        pub sub_type: [u8; 4],
        pub vital_tbl: [u8; 4],
        pub attack_tbl: [u8; 4],
        pub parts_tbl: [u8; 4],
        pub other_tbl: [u8; 4],
        pub stamina_tbl: [u8; 4],
        pub scale: [u8; 4],
        pub scale_tbl:[i32; 4], // snow.enemy.EnemyDef.BossScaleTblType
        pub difficulty: [NandoYuragi; 4],
        pub boss_multi: [u8; 4],
    }
}

impl_enemy_param!(DiscoverEmSetDataParam);

rsz_struct! {
    #[rsz("snow.quest.DiscoverEmSetData",
        path = "Quest/QuestData/DiscoverEmSetData.user",
        0x250dcb35 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct DiscoverEmSetData {
        pub param: Vec<DiscoverEmSetDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.data.MainTargetRewardLotNumDefineUserData.Param",
        0x266cce0b = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MainTargetRewardLotNumDefineUserDataParam {
        pub target_num: u32,
        pub base_lot_num: u32,
        pub max_lot_num: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.MainTargetRewardLotNumDefineUserData",
        path = "data/Define/Quest/System/QuestRewardSystem/MainTargetLotNumDefineData.user",
        0x360c1a50 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MainTargetRewardLotNumDefineUserData {
        pub param: Vec<MainTargetRewardLotNumDefineUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.QuestDataForRewardUserData.Param",
        0x5a13ba06 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestDataForRewardUserDataParam {
        pub quest_numer: i32,
        pub target_reward_add_num: u32,
        pub additional_target_reward_table_index: u32,
        pub common_material_add_num: u32,
        pub common_material_reward_table_index: u32,
        pub additional_quest_reward_table_index: Vec<u32>,
        pub cloth_ticket_index: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.QuestDataForRewardUserData",
        path = "data/Define/Quest/System/QuestRewardSystem/QuestDataForRewardData.user",
        0x424e2f4b = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestDataForRewardUserData {
        pub param: Vec<QuestDataForRewardUserDataParam>
    }
}

// snow.data.ItemLotTable.LotRule
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum LotRule {
        Random = 0,
        RandomOut1 = 1,
        RandomOut2 = 2,
        RandomOut3 = 3,
        FirstFix = 4,
    }
}

rsz_struct! {
    #[rsz("snow.data.RewardIdLotTableUserData.Param",
        0x214bfede = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RewardIdLotTableUserDataParam {
        pub id: u32,
        pub lot_rule: LotRule,
        pub item_id_list: Vec<ItemId>,
        pub num_list: Vec<u32>,
        pub probability_list: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.RewardIdLotTableUserData",
        path = "data/Define/Quest/System/QuestRewardSystem/RewardIdLotTableData.user",
        0xdb631ed5 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RewardIdLotTableUserData {
        pub param: Vec<RewardIdLotTableUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.quest.HyakuryuQuestData.WaveData",
        0xc4e69cfb = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuQuestDataWaveData {
        pub boss_em: EmTypes,
        pub boss_sub_type: u32,
        pub order_table_no: i32,
        pub em_table: [EmTypes; 4],
        pub boss_em_nando_tbl_no: i32,
        pub wave_em_nando_tbl_no: i32,
    }
}

// snow.quest.nHyakuryuQuest.Attr
rsz_bitflags! {
    pub struct HyakuryuQuestAttr: u8 {
        const FIX_WAVE_ORDER = 1;
        const LOT_HIGH_EX = 2;
        const LOT_TRUE_ED = 4;
        const FINAL_BOSS_KILL = 8;
    }
}

// snow.quest.nHyakuryuQuest.Category
rsz_enum! {
    #[rsz(u8)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum HyakuryuQuestCategory {
        Normal = 0,
        Nushi = 1,
    }
}

rsz_struct! {
    #[rsz("snow.quest.HyakuryuQuestData",
        0x48e96a40 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuQuestData {
        pub quest_no: i32,
        pub random_seed: i32,
        pub attr: HyakuryuQuestAttr,
        pub wave_data: [HyakuryuQuestDataWaveData; 3],
        pub quest_lv: QuestLevel,
        pub map_no: i32, // snow.QuestMapManager.MapNoType
        pub category: HyakuryuQuestCategory,
        pub is_village: bool,
        pub base_time: u8,
        pub start_block_no: u8,
        pub end_block_no: u8,
        pub extra_em_wave_no: u8,
        pub extra_em_nando_tbl_no: i8,
        pub nushi_order_tbl_no: u8,
        pub hm_unlock_tbl_no: u8,
        pub sub_target: [QuestTargetType; 6],
        pub sub_target5_wave_no: u8,
    }
}

rsz_struct! {
    #[rsz("snow.quest.HyakuryuQuestDataTbl",
        path = "Quest/Hyakuryu/QuestData/FixHyakuryuQuestData.user",
        0x9b163063 = 0,
        0xB0022BC2 = 2,
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuQuestDataTbl {
        pub data_list: Vec<HyakuryuQuestData>,
        pub data_list_310: Vec<HyakuryuQuestData>,
        pub data_list_320: Vec<HyakuryuQuestData>,
        pub data_list_350: Vec<HyakuryuQuestData>,
        pub data_list_370: Versioned<Vec<HyakuryuQuestData>, 2, {u32::MAX}>,
        pub data_list_380: Versioned<Vec<HyakuryuQuestData>, 2, {u32::MAX}>,
        pub data_list_390: Versioned<Vec<HyakuryuQuestData>, 2, {u32::MAX}>,
    }
}
