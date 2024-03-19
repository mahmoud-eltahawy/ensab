use leptos::RwSignal;

#[derive(Debug, Clone)]
pub struct FamilyMember {
    pub name: String,
    pub is_male: bool,
    pub generation: i32,
    pub sibling_order: i32,
    pub sons: Vec<RwSignal<FamilyMember>>,
}

impl Default for FamilyMember {
    fn default() -> Self {
        FamilyMember {
            name: String::new(),
            generation: 1,
            sibling_order: 1,
            is_male: true,
            sons: vec![],
        }
    }
}

impl FamilyMember {
    pub fn with_sons(&mut self, names: &mut Vec<String>) {
        let Some(name) = names.pop() else {
            return;
        };
        let mut son = FamilyMember {
            name,
            generation: self.generation + 1,
            ..Default::default()
        };
        son.with_sons(names);
        self.sons = vec![RwSignal::new(son)];
    }

    pub fn key(&self) -> String {
        self.name.clone() + &self.generation.to_string() + &self.sibling_order.to_string()
    }

    pub fn add_son(&mut self, name: String, is_male: bool) {
        let person = Self::create_from_name(name, self.generation + 1);
        self.sons.push(RwSignal::new(FamilyMember {
            is_male,
            sibling_order: self.sons.len() as i32 + 1,
            ..person
        }));
    }

    pub fn create_from_name(name: String, generation: i32) -> Self {
        let mut names = name
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let mut person = FamilyMember {
            name: names.pop().unwrap(),
            generation,
            ..Default::default()
        };
        person.with_sons(&mut names);
        person
    }
}
