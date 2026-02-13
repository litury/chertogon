use bevy::prelude::*;

/// XP орб — зелёная светящаяся сфера, дропается при смерти врага
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct XpOrb {
    pub xp_value: f32,
    /// Фаза полёта к игроку (true = летит)
    pub magnetized: bool,
    /// Время жизни (для bobbing анимации)
    pub age: f32,
    /// Начальное смещение для разброса при спавне
    pub spawn_offset: Vec3,
}

/// HP орб — красная сфера, хилит при сборе
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HpOrb {
    pub heal_amount: f32,
    pub magnetized: bool,
    pub age: f32,
    pub spawn_offset: Vec3,
}

/// Ресурс: опыт и уровень игрока
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerXp {
    pub current_xp: f32,
    pub level: u32,
    pub xp_to_next: f32,
    /// Базовый радиус магнита XP/HP орбов
    pub magnet_radius: f32,
    /// Флаг: требуется показать экран level-up
    pub pending_level_up: bool,
}

impl Default for PlayerXp {
    fn default() -> Self {
        Self {
            current_xp: 0.0,
            level: 1,
            xp_to_next: 100.0,
            magnet_radius: 3.0,
            pending_level_up: false,
        }
    }
}

impl PlayerXp {
    /// XP кривая: Level N → (N * 100) XP, cap 1000 при level 10+
    pub fn calculate_xp_to_next(level: u32) -> f32 {
        ((level) * 100).min(1000) as f32
    }

    /// Добавляет XP, проверяет level-up
    pub fn add_xp(&mut self, amount: f32) {
        self.current_xp += amount;
        while self.current_xp >= self.xp_to_next {
            self.current_xp -= self.xp_to_next;
            self.level += 1;
            self.xp_to_next = Self::calculate_xp_to_next(self.level);
            self.pending_level_up = true;
        }
    }
}

/// ID апгрейда
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum UpgradeId {
    RunePeruna,
    RuneVetra,
    OberegSvaroga,
    SlezaLady,
    ZnakVolka,
}

/// Категория апгрейда
#[derive(Clone, Copy, Debug)]
pub enum UpgradeCategory {
    Attack,
    Defense,
    Path,
}

/// Статическое описание апгрейда
pub struct UpgradeDef {
    pub id: UpgradeId,
    pub name: &'static str,
    pub description: &'static str,
    pub category: UpgradeCategory,
    pub max_level: u32,
}

/// Инвентарь апгрейдов игрока
#[derive(Resource, Default)]
pub struct UpgradeInventory {
    pub upgrades: Vec<(UpgradeId, u32)>,
}

impl UpgradeInventory {
    pub fn get_level(&self, id: &UpgradeId) -> u32 {
        self.upgrades.iter()
            .find(|(uid, _)| uid == id)
            .map(|(_, level)| *level)
            .unwrap_or(0)
    }

    pub fn increment(&mut self, id: UpgradeId) {
        if let Some((_, level)) = self.upgrades.iter_mut().find(|(uid, _)| *uid == id) {
            *level += 1;
        } else {
            self.upgrades.push((id, 1));
        }
    }
}

/// Состояние level-up экрана
#[derive(Resource, Default)]
pub struct LevelUpState {
    pub is_active: bool,
    pub offered_upgrades: Vec<UpgradeId>,
}
