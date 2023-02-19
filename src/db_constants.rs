pub const TABLE_NAME: &str = "StatsDB-Test";
pub const LSI1_NAME: &str = "LSI1";
pub const GSI1_NAME: &str = "GSI1";
pub const GSI2_NAME: &str = "GSI2";

#[derive(Clone)]
pub struct KeyName {
    pk: String,
    sk: String,
}

impl KeyName {
    pub fn from_index_name(index_name: Option<&str>) -> Self {
        match index_name {
            None => KeyName {
                pk: "PK".to_owned(),
                sk: "SK".to_owned(),
            },
            Some(k) => match k {
                LSI1_NAME => KeyName {
                    pk: "PK".to_owned(),
                    sk: "LSI1-SK".to_owned(),
                },
                GSI1_NAME => KeyName {
                    pk: "GSI1-PK".to_owned(),
                    sk: "GSI1-SK".to_owned(),
                },
                GSI2_NAME => KeyName {
                    pk: "GSI2-PK".to_owned(),
                    sk: "GSI2-SK".to_owned(),
                },
                _ => panic!("Provided unknown index name"),
            },
        }
    }

    pub fn as_tuple(&self) -> (&str, &str) {
        (&self.pk, &self.sk)
    }
}
