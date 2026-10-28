use serde::{
    Deserialize,
    Serialize
};

// #region content
#[derive(Serialize, Deserialize)]
pub struct BlogPost {
    title: String,
    content: String,
}

impl BlogPost {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
// #endregion content
