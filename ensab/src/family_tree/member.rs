use std::collections::{HashMap, HashSet};

use contracts::member::{RawMember, SonlessRawMember};
use leptos::{RwSignal, SignalGet, SignalSet, SignalUpdate};
use uuid::Uuid;

#[derive(Clone, Copy, Default)]
pub enum Action {
    #[default]
    Preview,
    Add,
    Remove,
    Update,
}

#[derive(Clone, Copy, Default)]
pub struct Member {
    pub id: Uuid,
    pub name: RwSignal<String>,
    pub is_male: RwSignal<bool>,
    pub sons: RwSignal<Vec<Member>>,
    pub action: RwSignal<Action>,
}

#[derive(Clone, Copy)]
struct Updates {
    created: RwSignal<HashMap<Uuid, Vec<RawMember>>>,
    updates: RwSignal<Vec<SonlessRawMember>>,
    deleted: RwSignal<HashSet<Uuid>>,
}

impl Updates {
    fn is_dirty(&self) -> bool {
        !self.deleted.get().is_empty()
            || !self.updates.get().is_empty()
            || !self.created.get().is_empty()
    }

    fn record_update(&self, member: SonlessRawMember) {
        let old_member = self.updates.get().into_iter().find(|x| x.id == member.id);
        // TODO : check if the fields are matching original member and if so cancel the update
        match old_member {
            Some(mut old_member) => {
                old_member.name = member.name;
                old_member.is_male = member.is_male;
                self.updates.update(|xs| {
                    xs.retain(|x| x.id != old_member.id);
                    xs.push(old_member);
                })
            }
            None => {
                self.updates.update(|xs| xs.push(member));
            }
        };
    }

    fn record_create(&self, parent_id: Uuid, member: RawMember) {
        let old_parent_sons = self.created.get();
        let old_parent_sons = old_parent_sons.get(&parent_id);
        match old_parent_sons {
            Some(old_parent_sons) => {
                let mut siblings = old_parent_sons
                    .into_iter()
                    .filter(|x| x.name != member.name)
                    .cloned()
                    .collect::<Vec<_>>();
                self.created.update(|xs| {
                    siblings.push(member);
                    xs.insert(parent_id, siblings);
                });
            }
            None => self.created.update(|xs| {
                xs.insert(parent_id, vec![member]);
            }),
        }
    }

    fn record_delete(&self, id: Uuid) {
        if self.deleted.get().contains(&id) {
            self.deleted.update(|xs| xs.retain(|x| *x != id));
        } else {
            self.deleted.update(|xs| {
                xs.insert(id);
            });
        }
    }
}

impl Member {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: RwSignal::new(name),
            is_male: RwSignal::new(true),
            sons: Default::default(),
            action: Default::default(),
        }
    }
    pub fn from_raw(
        RawMember {
            id,
            name,
            is_male,
            sons,
        }: RawMember,
    ) -> Self {
        Self {
            id,
            name: RwSignal::new(name),
            is_male: RwSignal::new(is_male),
            sons: RwSignal::new(sons.into_iter().map(|x| Member::from_raw(x)).collect()),
            action: Default::default(),
        }
    }
    pub fn raw(self) -> RawMember {
        RawMember {
            id: self.id,
            name: self.name.get(),
            is_male: self.is_male.get(),
            sons: self.sons.get().into_iter().map(|x| x.raw()).collect(),
        }
    }
    pub fn sonless_raw(self) -> SonlessRawMember {
        SonlessRawMember {
            id: self.id,
            name: self.name.get(),
            is_male: self.is_male.get(),
        }
    }

    fn with_sons(&self, names: &mut Vec<String>) {
        let name = names.pop();
        let Some(name) = name else {
            return;
        };
        let son = Member::new(name);
        son.with_sons(names);
        self.sons.set(vec![son]);
    }

    pub fn create_from_name(name: &str) -> Self {
        let mut names = name.split("->").map(|x| x.to_string()).collect::<Vec<_>>();
        let name = names.pop().unwrap();
        let person = Member::new(name);
        person.with_sons(&mut names);
        person
    }

    pub fn add_son(&self, member: Member) {
        let sons = self.sons.get();
        let same_person = sons.iter().find(|x| x.name.get() == member.name.get());
        match same_person {
            Some(same_person) => {
                for person in member.sons.get() {
                    same_person.add_son(person)
                }
            }
            None => {
                self.sons.update(|xs| xs.push(member));
                // UPDATES.record_create(self.id, member.raw());
            }
        }
    }
}
