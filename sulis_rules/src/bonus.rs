//  This file is part of Sulis, a turn based RPG written in Rust.
//  Copyright 2018 Jared Stephen
//
//  Sulis is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  Sulis is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with Sulis.  If not, see <http://www.gnu.org/licenses/>

use sulis_core::util::ExtInt;
use {Attribute, Damage, DamageKind, ArmorKind, Slot, WeaponKind, WeaponStyle};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum BonusKind {
    Attribute { attribute: Attribute, amount: i8 },
    ActionPoints(i32),
    Armor(i32),
    ArmorKind { kind: DamageKind, amount: i32 },
    Damage(Damage),
    ArmorProficiency(ArmorKind),
    WeaponProficiency(WeaponKind),
    Reach(f32),
    Range(f32),
    Initiative(i32),
    HitPoints(i32),
    MeleeAccuracy(i32),
    RangedAccuracy(i32),
    SpellAccuracy(i32),
    Defense(i32),
    Fortitude(i32),
    Reflex(i32),
    Will(i32),
    Concealment(i32),
    ConcealmentIgnore(i32),
    CritThreshold(i32),
    HitThreshold(i32),
    GrazeThreshold(i32),
    CritMultiplier(f32),
    HitMultiplier(f32),
    GrazeMultiplier(f32),
    MovementRate(f32),
    AttackCost(i32),
    FlankingAngle(i32),
    CasterLevel(i32),
    MoveDisabled,
    AttackDisabled,
    Hidden,
    GroupUsesPerEncounter { group: String, amount: ExtInt },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub enum Contingent {
    /// Bonuses that should always be applied
    Always,

    /// Bonuses that should only be applied to the parent if they have the given
    /// WeaponKind equipped
    WeaponEquipped(WeaponKind),

    /// Bonuses that should only be applied to the parent if they have a shield of
    /// the specified type equipped
    ArmorEquipped { kind: ArmorKind, slot: Slot },

    /// Bonus that should only be applied if the parent's attack is of the given
    /// weapon style
    WeaponStyle(WeaponStyle),

    /// For bonuses applied to attacks, only Damage, MeleeAccuracy, RangedAccuracy,
    /// SpellAccuracy, CritThreshold, HitThreshold,
    /// GrazeThreshold, CritMultiplier, HitMultiplier, and GrazeMultiplier are valid

    /// Bonuses that should only be applied to an attack using the given WeaponKind
    AttackWithWeapon(WeaponKind),

    /// Bonuses that are only applied to attacks when the attacker is hidden
    AttackWhenHidden,

    /// Bonuses that are only applied to attacks when the attacker is flanking
    AttackWhenFlanking,

    /// Bonuses that are only applied to attacks with the specified base damage kind
    AttackWithDamageKind(DamageKind),
}

impl Default for Contingent {
    fn default() -> Contingent {
        Contingent::Always
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Bonus {
    #[serde(default)]
    pub when: Contingent,
    pub kind: BonusKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BonusList(Vec<Bonus>);

impl BonusList {
    /// An iterator through all standard bonuses held in this list.  These bonuses
    /// should always be applied to the parent entity
    pub fn iter(&self) -> impl Iterator<Item=&Bonus> {
        self.0.iter()
    }

    pub fn add(&mut self, bonus: Bonus) {
        self.0.push(bonus);
    }

    pub fn add_kind(&mut self, kind: BonusKind) {
        self.0.push(Bonus { when: Contingent::Always, kind });
    }
}

impl Default for BonusList {
    fn default() -> BonusList {
        BonusList(Vec::new())
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct AttackBonuses {
    pub damage: Option<Damage>,
    pub melee_accuracy: i32,
    pub ranged_accuracy: i32,
    pub spell_accuracy: i32,
    pub crit_threshold: i32,
    pub hit_threshold: i32,
    pub graze_threshold: i32,
    pub crit_multiplier: f32,
    pub hit_multiplier: f32,
    pub graze_multiplier: f32,
}

impl Default for AttackBonuses {
    fn default() -> AttackBonuses {
        AttackBonuses {
            damage: None,
            melee_accuracy: 0,
            ranged_accuracy: 0,
            spell_accuracy: 0,
            crit_threshold: 0,
            hit_threshold: 0,
            graze_threshold: 0,
            crit_multiplier: 0.0,
            hit_multiplier: 0.0,
            graze_multiplier: 0.0,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AttackBuilder {
    pub damage: Damage,
    pub kind: AttackKindBuilder,
    pub bonuses: AttackBonuses,
}

impl AttackBuilder {
    pub fn distance(&self) -> f32 {
        match self.kind {
            AttackKindBuilder::Melee { reach } => reach,
            AttackKindBuilder::Ranged { range, .. } => range,
        }
    }

    pub fn mult(&mut self, multiplier: f32) -> AttackBuilder {
        AttackBuilder {
            damage: self.damage.mult_f32(multiplier),
            kind: self.kind.clone(),
            bonuses: self.bonuses.clone(),
        }
    }

    pub fn is_melee(&self) -> bool {
        match self.kind {
            AttackKindBuilder::Melee { .. } => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, untagged)]
pub enum AttackKindBuilder {
    Melee {
        reach: f32,
    },
    Ranged {
        range: f32,
        projectile: String,
    },
}
