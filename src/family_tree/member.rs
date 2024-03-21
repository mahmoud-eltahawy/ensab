use leptos::{logging, RwSignal, SignalGet, SignalUpdateUntracked};

#[derive(Debug, Clone, Copy)]
pub struct FamilyMember {
    pub name: RwSignal<String>,
    pub is_male: bool,
    pub generation: i32,
    pub sibling_order: i32,
    pub sons: RwSignal<Vec<RwSignal<FamilyMember>>>,
}

impl Default for FamilyMember {
    fn default() -> Self {
        FamilyMember {
            name: RwSignal::new(String::new()),
            generation: 1,
            sibling_order: 1,
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
            generation: self.generation + 1,
            ..Default::default()
        };
        son.with_sons(names);
        self.sons = RwSignal::new(vec![RwSignal::new(son)]);
    }

    pub fn key(&self) -> String {
        self.name.get() + &self.generation.to_string() + &self.sibling_order.to_string()
    }

    pub fn add_son(&self, name: String, is_male: bool) {
        let person = Self::create_from_name(name, self.generation + 1);
        self.sons.update_untracked(|sons| {
            if sons.iter().any(|x| x.get().name.get() == person.name.get()) {
                logging::log!("{} already exists", person.name.get());
            } else {
                sons.push(RwSignal::new(FamilyMember {
                    is_male,
                    sibling_order: sons.len() as i32 + 1,
                    ..person
                }));
            }
        });
    }

    pub fn create_from_name(name: String, generation: i32) -> Self {
        let mut names = name
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let Some(name) = names.pop() else {
            return Self::default();
        };
        let mut person = FamilyMember {
            name: RwSignal::new(name),
            generation,
            ..Default::default()
        };
        person.with_sons(&mut names);
        person
    }
}
