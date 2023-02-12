use crate::dto;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddStatRequest {
    user: String,
    game: String,
    stat: String,
    value: f64,
    day: Option<u128>,
}

impl AddStatRequest {
    pub fn as_stat_view(&self) -> dto::stat_view::StatView {
        dto::stat_view::StatView::new(&self.user, &self.game, &self.stat, self.value, self.day)
    }
}
