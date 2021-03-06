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

use std::rc::Rc;
use std::cell::RefCell;

use sulis_rules::HitKind;
use {script::ScriptEntitySet, ScriptCallback};
use {animation::Anim, EntityState, GameState, area_feedback_text::ColorKind};

pub (in animation) fn update(attacker: &Rc<RefCell<EntityState>>, model: &mut MeleeAttackAnimModel, frac: f32) {
    if !model.has_attacked && frac > 0.5 {
        let cb_def_targets = ScriptEntitySet::new(&model.defender, &vec![Some(Rc::clone(attacker))]);
        let cb_att_targets = ScriptEntitySet::new(attacker, &vec![Some(Rc::clone(&model.defender))]);

        for cb in model.callbacks.iter() {
            cb.before_attack(&cb_def_targets);
        }

        let area_state = GameState::area_state();

        let defender_cbs = model.defender.borrow().callbacks();
        let attacker_cbs = attacker.borrow().callbacks();

        attacker_cbs.iter().for_each(|cb| cb.before_attack(&cb_def_targets));
        defender_cbs.iter().for_each(|cb| cb.before_defense(&cb_att_targets));

        let (hit_kind, damage, text, color) = (model.attack_func)(attacker, &model.defender);

        area_state.borrow_mut().add_feedback_text(text, &model.defender, color);
        model.has_attacked = true;

        for cb in model.callbacks.iter() {
            cb.after_attack(&cb_def_targets, hit_kind, damage);
        }

        attacker_cbs.iter().for_each(|cb| cb.after_attack(&cb_att_targets, hit_kind, damage));
        defender_cbs.iter().for_each(|cb| cb.after_defense(&cb_def_targets, hit_kind, damage));
    }

    let mut attacker = attacker.borrow_mut();
    if frac > 0.5 {
        attacker.sub_pos = ((1.0 - frac) * model.vector.0, (1.0 - frac) * model.vector.1);
    } else {
        attacker.sub_pos = (frac * model.vector.0, frac * model.vector.1);
    }
}

pub (in animation) fn cleanup(owner: &Rc<RefCell<EntityState>>) {
    owner.borrow_mut().sub_pos = (0.0, 0.0);
}

pub (in animation) struct MeleeAttackAnimModel {
    defender: Rc<RefCell<EntityState>>,
    callbacks: Vec<Box<ScriptCallback>>,
    vector: (f32, f32),
    pub (in animation) has_attacked: bool,
    attack_func: Box<Fn(&Rc<RefCell<EntityState>>, &Rc<RefCell<EntityState>>) ->
        (HitKind, u32, String, ColorKind)>,
}

pub fn new(attacker: &Rc<RefCell<EntityState>>,
           defender: &Rc<RefCell<EntityState>>,
           duration_millis: u32,
           callbacks: Vec<Box<ScriptCallback>>,
           attack_func: Box<Fn(&Rc<RefCell<EntityState>>, &Rc<RefCell<EntityState>>) ->
                (HitKind, u32, String, ColorKind)>) -> Anim {

    let x = defender.borrow().location.x + defender.borrow().size.width / 2
        - attacker.borrow().location.x - attacker.borrow().size.width / 2;
    let y = defender.borrow().location.y + defender.borrow().size.height / 2
        - attacker.borrow().location.y - attacker.borrow().size.height / 2;
    let vector = (x as f32, y as f32);

    let model = MeleeAttackAnimModel {
        defender: Rc::clone(defender),
        callbacks,
        has_attacked: false,
        attack_func,
        vector,
    };

    Anim::new_melee_attack(attacker, duration_millis, model)
}
