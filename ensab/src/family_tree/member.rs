use contracts::member::{RawMember, SonlessRawMember};
use leptos::{RwSignal, SignalGetUntracked, SignalSet, SignalUpdate};
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
pub struct Updates {
    origin: RwSignal<RawMember>,
    pub copy: RwSignal<Member>,
}

impl Updates {
    pub fn init(member: Member) -> Self {
        Self {
            origin: RwSignal::from(member.raw()),
            copy: RwSignal::new(member),
        }
    }

    pub fn updates(&self) -> Vec<SonlessRawMember> {
        let origin = self.origin.get_untracked();
        let copy = self.copy.get_untracked().raw();

        pub fn compare(origin: RawMember, copy: RawMember) -> Vec<SonlessRawMember> {
            let first = if origin.name != copy.name || origin.is_male != copy.is_male {
                vec![copy.clone().sonless()]
            } else {
                vec![]
            };
            let rest = origin
                .sons
                .into_iter()
                .filter(|x| copy.sons.iter().any(|y| x.id == y.id))
                .zip(copy.sons.clone())
                .map(|(origin, copy)| compare(origin, copy))
                .flatten();
            rest.chain(first).collect()
        }
        compare(origin, copy)
    }

    pub fn created(&self) -> Vec<RawMember> {
        let origin = self.origin.get_untracked();
        let copy = self.copy.get_untracked().raw();

        pub fn compare(origin: RawMember, copy: RawMember) -> Vec<RawMember> {
            let first = copy
                .sons
                .iter()
                .filter(|son| origin.sons.iter().all(|x| x.id != son.id))
                .cloned()
                .collect::<Vec<_>>();

            origin
                .sons
                .into_iter()
                .zip(copy.sons)
                .map(|(origin, copy)| compare(origin, copy))
                .flatten()
                .chain(first)
                .collect::<Vec<_>>()
        }
        compare(origin, copy)
    }

    pub fn deleted(&self) -> Vec<Uuid> {
        let origin = self.origin.get_untracked();
        let copy = self.copy.get_untracked().raw();

        pub fn compare(origin: RawMember, copy: RawMember) -> Vec<RawMember> {
            let first = origin
                .sons
                .iter()
                .filter(|son| copy.sons.iter().all(|x| x.id != son.id))
                .cloned()
                .collect::<Vec<_>>();

            origin
                .sons
                .into_iter()
                .zip(copy.sons)
                .map(|(origin, copy)| compare(origin, copy))
                .flatten()
                .chain(first)
                .collect::<Vec<_>>()
        }
        compare(origin, copy).into_iter().map(|x| x.id).collect()
    }
}

//     pub fn record_update(&self, new_updates: SonlessRawMember) {
//         let previously_updated_member = self
//             .updates
//             .get_untracked()
//             .into_iter()
//             .find(|x| x.id == new_updates.id);

//         match previously_updated_member {
//             Some(_) => self.updates.update(|xs| {
//                 xs.retain(|x| x.id != new_updates.id);
//                 xs.push(new_updates);
//             }),
//             None => {
//                 self.updates.update(|xs| xs.push(new_updates));
//             }
//         };
//         logging::log!("{:#?}", self.updates.get_untracked());
//     }

//     pub fn remove_update(&self, id: Uuid) {
//         self.updates.update(|xs| xs.retain(|x| x.id != id));
//         logging::log!("{:#?}", self.updates.get_untracked());
//     }

//     pub fn record_create(&self, parent_id: Uuid, member: RawMember) {
//         let old_parent_sons = self.created.get();
//         let old_parent_sons = old_parent_sons.get(&parent_id);
//         match old_parent_sons {
//             Some(old_parent_sons) => {
//                 let mut siblings = old_parent_sons
//                     .iter()
//                     .filter(|x| x.name != member.name)
//                     .cloned()
//                     .collect::<Vec<_>>();
//                 self.created.update(|xs| {
//                     siblings.push(member);
//                     xs.insert(parent_id, siblings);
//                 });
//             }
//             None => self.created.update(|xs| {
//                 xs.insert(parent_id, vec![member]);
//             }),
//         }
//         //BUG : sons are not mounted correctly
//         logging::log!("{:#?}", self.created.get_untracked());
//     }

//     pub fn record_delete(&self, id: Uuid) {
//         self.deleted.update(|xs| {
//             xs.insert(id);
//         });
//         logging::log!("{:#?}", self.deleted.get_untracked());
//     }
// }

pub trait Rm {
    fn find_son(&self, id: Uuid) -> Option<&RawMember>;
    fn sonless(self) -> SonlessRawMember;
}

impl Rm for RawMember {
    fn find_son(&self, id: Uuid) -> Option<&RawMember> {
        if self.id == id {
            return Some(self);
        } else {
            for son in self.sons.iter() {
                if let Some(son) = son.find_son(id) {
                    return Some(son);
                };
            }
        }
        None
    }

    fn sonless(self) -> SonlessRawMember {
        SonlessRawMember {
            id: self.id,
            name: self.name,
            is_male: self.is_male,
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
            sons: RwSignal::new(sons.into_iter().map(Member::from_raw).collect()),
            action: Default::default(),
        }
    }
    pub fn raw(self) -> RawMember {
        RawMember {
            id: self.id,
            name: self.name.get_untracked(),
            is_male: self.is_male.get_untracked(),
            sons: self
                .sons
                .get_untracked()
                .into_iter()
                .map(|x| x.raw())
                .collect::<Vec<_>>(),
        }
    }
    // pub fn sonless_raw(self) -> SonlessRawMember {
    //     SonlessRawMember {
    //         id: self.id,
    //         name: self.name.get_untracked(),
    //         is_male: self.is_male.get_untracked(),
    //     }
    // }

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
        let sons = self.sons.get_untracked();
        let same_person = sons
            .iter()
            .find(|x| x.name.get_untracked() == member.name.get_untracked());
        match same_person {
            Some(same_person) => {
                for person in member.sons.get_untracked() {
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
