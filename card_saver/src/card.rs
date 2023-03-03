use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct YuGiOhCard {
    pub id: i32,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "frameType")]
    pub frame_type: Option<String>,
    pub desc: Option<String>,
    pub race: Option<String>,
    pub archetype: Option<String>,
    pub card_images: Vec<CardImages>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CardImages {
    pub id: i32,
    pub image_url: Option<String>,
    pub image_url_small: Option<String>,
    pub image_url_cropped: Option<String>,
}
