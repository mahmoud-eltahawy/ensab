use leptos::{RwSignal, SignalGet, SignalGetUntracked, SignalUpdate};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct FamilyMember {
    pub id: Uuid,
    pub name: RwSignal<String>,
    pub is_male: bool,
    pub sons: RwSignal<Vec<FamilyMember>>,
}

impl Default for FamilyMember {
    fn default() -> Self {
        FamilyMember {
            id: Uuid::new_v4(),
            name: RwSignal::new(String::new()),
            is_male: true,
            sons: RwSignal::new(vec![]),
        }
    }
}

impl FamilyMember {
    pub fn new(name: RwSignal<String>) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
    pub fn with_sons(&mut self, names: &mut Vec<String>) {
        let Some(name) = names.pop() else {
            return;
        };
        let mut son = FamilyMember {
            name: RwSignal::new(name),
            ..Default::default()
        };
        son.with_sons(names);
        self.sons = RwSignal::new(vec![son]);
    }

    pub fn add_son(&self, name: String, is_male: bool) {
        let person = Self::create_from_name(name);
        self.sons.update(|sons| {
            if let Some(son) = sons.iter().find(|x| x.name.get() == person.name.get()) {
                let persons = person.sons.get_untracked().into_iter().collect::<Vec<_>>();
                son.sons.update(|sons| {
                    for person in persons {
                        sons.push(FamilyMember { is_male, ..person })
                    }
                });
            } else {
                sons.push(FamilyMember { is_male, ..person });
            }
        });
    }

    pub fn create_from_name(name: String) -> Self {
        let mut names = name
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let Some(name) = names.pop() else {
            return Self::default();
        };
        let mut person = FamilyMember {
            name: RwSignal::new(name),
            ..Default::default()
        };
        person.with_sons(&mut names);
        person
    }
}
