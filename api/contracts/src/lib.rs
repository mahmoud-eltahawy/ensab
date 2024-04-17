pub mod member {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Debug)]
    pub struct RawMember {
        pub id: Uuid,
        pub name: String,
        pub is_male: bool,
        pub sons: Vec<RawMember>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, Default)]
    pub struct SonlessRawMember {
        pub id: Uuid,
        pub name: String,
        pub is_male: bool,
    }
}

pub mod users {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    pub struct User {
        pub id: Uuid,
        pub username: String,
        pub password: String,
        pub member_id: Option<Uuid>,
    }
}
