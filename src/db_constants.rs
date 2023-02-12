pub const TABLE_NAME: &str = "StatsDB";
pub const LSI_NAME: &str = "LSI";
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
                LSI_NAME => KeyName {
                    pk: "PK".to_owned(),
                    sk: "LSI-SK".to_owned(),
                },
                GSI1_NAME => KeyName {
                    pk: "GSI1_PK".to_owned(),
                    sk: "GSI1_SK".to_owned(),
                },
                GSI2_NAME => KeyName {
                    pk: "GSI2_PK".to_owned(),
                    sk: "GSI2_SK".to_owned(),
                },
                _ => panic!("Provided unknown index name"),
            },
        }
    }

    pub fn as_tuple(&self) -> (&str, &str) {
        (&self.pk, &self.sk)
    }
}
