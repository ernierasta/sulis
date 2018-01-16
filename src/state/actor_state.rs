use module::{item, Actor, Module};
use state::{ChangeListenerList, EntityState, Inventory};
use rules::{AttributeList, StatList};

use std::rc::Rc;
use std::cell::{RefCell};

pub struct ActorState {
    pub actor: Rc<Actor>,
    inventory: Inventory,
    pub attributes: AttributeList,
    pub stats: StatList,
    pub listeners: ChangeListenerList<ActorState>,
    hp: u32,
    ap: u32,
}

impl PartialEq for ActorState {
    fn eq(&self, other: &ActorState) -> bool {
        Rc::ptr_eq(&self.actor, &other.actor)
    }
}

impl ActorState {
    pub fn new(actor: Rc<Actor>) -> ActorState {
        trace!("Creating new actor state for {}", actor.id);
        let inventory = Inventory::new(&actor);
        ActorState {
            actor,
            inventory,
            attributes: AttributeList::default(),
            stats: StatList::default(),
            listeners: ChangeListenerList::default(),
            hp: 0,
            ap: 0,
        }
    }

    /// this method is only to be called by `EntityState#can_attack`
    pub(in state) fn can_attack(&self, _target: &Rc<RefCell<EntityState>>, dist: f32) -> bool {
        trace!("Checking can attack for '{}' with reach of {}.  Distance to target is {}",
               self.actor.name, self.stats.reach, dist);
        dist < self.stats.reach
    }

    /// this method is only to be called by `EntityState#attack`
    pub(in state) fn attack(&mut self, target: &Rc<RefCell<EntityState>>) {
        let amount = self.stats.damage.roll();
        info!("'{}' attacks '{}' for {} damage", self.actor.name,
              target.borrow().actor.actor.name, amount);
        target.borrow_mut().remove_hp(amount);
    }

    pub fn equip(&mut self, index: usize) -> bool {
        let result = self.inventory.equip(index);
        self.compute_stats();

        result
    }

    pub fn unequip(&mut self, slot: item::Slot) -> bool {
        let result = self.inventory.unequip(slot);
        self.compute_stats();

        result
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn hp(&self) -> u32 {
        self.hp
    }

    pub fn ap(&self) -> u32 {
        self.ap
    }

    pub(in state) fn remove_hp(&mut self, hp: u32) {
        if hp > self.hp {
            self.hp = 0;
        } else {
            self.hp -= hp;
        }

        self.listeners.notify(&self);
    }

    pub fn init(&mut self) {
        self.hp = self.stats.max_hp;
        self.init_turn();
    }

    pub fn init_turn(&mut self) {
        let rules = Module::rules();

        self.ap = rules.base_ap;
    }

    pub fn compute_stats(&mut self) {
        let rules = Module::rules();

        if let Some(ref item_state) = self.inventory.get(item::Slot::HeldMain) {
            if let Some(equippable) = item_state.item.equippable {
                if let Some(damage) = equippable.damage {
                    self.stats.damage = damage;
                }

                if let Some(reach)= equippable.reach {
                    self.stats.reach = reach;
                }
            }
        }

        // for item_state in self.inventory.borrow().equipped_iter() {
        //     let equippable = match item_state.item.equippable {
        //         None => continue,
        //         Some(equippable) => equippable,
        //     };
        // }

        let mut max_hp: u32 = 0;
        for &(ref class, level) in self.actor.levels.iter() {
            max_hp += class.hp_per_level * level;
        }
        self.stats.max_hp = max_hp;

        self.stats.initiative = rules.base_initiative;

        self.listeners.notify(&self);
    }
}
