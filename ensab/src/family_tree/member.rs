use contracts::member::{RawMember, SonlessRawMember};
use leptos::{server, RwSignal, ServerFnError, SignalGetUntracked, SignalSet, SignalUpdate};
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

    fn updates(&self) -> Vec<SonlessRawMember> {
        let origin = self.origin.get_untracked();
        let copy = self.copy.get_untracked().raw();

        fn compare(origin: RawMember, copy: RawMember) -> Vec<SonlessRawMember> {
            let first = if origin.name != copy.name || origin.is_male != copy.is_male {
                Some(copy.clone().sonless())
            } else {
                None
            };
            let mut rest = Vec::new();
            if let Some(son) = first {
                rest.push(son);
            }
            for origin_son in origin.sons {
                let copy_son = copy.sons.iter().find(|x| x.id == origin_son.id);
                if let Some(copy_son) = copy_son {
                    rest.extend(compare(origin_son, copy_son.clone()));
                }
            }
            rest
        }
        compare(origin, copy)
    }

    fn created(&self) -> Vec<(Uuid, RawMember)> {
        let origin = self.origin.get_untracked();
        let copy = self.copy.get_untracked().raw();

        fn compare(origin: RawMember, copy: RawMember) -> Vec<(Uuid, RawMember)> {
            let mut rest = copy
                .sons
                .iter()
                .filter(|son| origin.sons.iter().all(|x| x.id != son.id))
                .cloned()
                .map(|x| (copy.id, x))
                .collect::<Vec<_>>();
            for origin_son in origin.sons {
                let copy_son = copy.sons.iter().find(|x| x.id == origin_son.id);
                if let Some(copy_son) = copy_son {
                    rest.extend(compare(origin_son, copy_son.clone()));
                }
            }
            rest
        }
        compare(origin, copy)
    }

    fn deleted(&self) -> Vec<Uuid> {
        let origin = self.origin.get_untracked();
        let copy = self.copy.get_untracked().raw();

        fn compare(origin: RawMember, copy: RawMember) -> Vec<RawMember> {
            let mut rest = origin
                .sons
                .iter()
                .filter(|son| copy.sons.iter().all(|x| x.id != son.id))
                .cloned()
                .collect::<Vec<_>>();
            for origin_son in origin.sons {
                let copy_son = copy.sons.iter().find(|x| x.id == origin_son.id);
                if let Some(copy_son) = copy_son {
                    rest.extend(compare(origin_son, copy_son.clone()));
                }
            }
            rest
        }
        compare(origin, copy).into_iter().map(|x| x.id).collect()
    }

    pub async fn commit(&self) -> Result<(), ServerFnError> {
        for member in self.updates() {
            update_member(member).await?;
        }
        for (parent_id, member) in self.created() {
            save_member_son(parent_id, member).await?;
        }
        for id in self.deleted() {
            delete_member(id).await?;
        }
        self.origin.set(self.copy.get_untracked().raw());
        Ok(())
    }
    pub fn discard(&self) {
        self.copy.set(Member::from_raw(self.origin.get_untracked()));
    }
}
#[server(encoding = "Cbor")]
async fn save_member_son(parent_id: Uuid, son: RawMember) -> Result<(), ServerFnError> {
    println!("parent id : {}\nson : {:#?}", parent_id, son);
    Ok(())
}
#[server(encoding = "Cbor")]
async fn delete_member(id: Uuid) -> Result<(), ServerFnError> {
    println!("deleted id : {}", id);
    Ok(())
}
#[server(encoding = "Cbor")]
async fn update_member(member: SonlessRawMember) -> Result<(), ServerFnError> {
    println!("update member : {:#?}", member);
    Ok(())
}

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
            }
        }
    }
}
