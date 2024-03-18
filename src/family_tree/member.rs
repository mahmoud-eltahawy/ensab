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
    pub fn with_sons(&mut self, names: &mut Vec<String>, generation: i32) {
        let Some(name) = names.pop() else {
            return;
        };
        let mut son = FamilyMember {
            name,
            generation,
            ..Default::default()
        };
        son.with_sons(names, generation + 1);
        self.sons = vec![RwSignal::new(son)];
    }

    pub fn key(&self) -> String {
        self.name.clone() + &self.generation.to_string() + &self.sibling_order.to_string()
    }
}
