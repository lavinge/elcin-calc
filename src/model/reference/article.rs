use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Article {
    id: String,
    title: String,
    text: String,
}

impl Article {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            text: String::new(),
        }
    }

    pub fn with_id(&mut self, id: impl ToString) -> &mut Self {
        self.id = id.to_string();

        self
    }

    pub fn with_title(&mut self, title: impl ToString) -> &mut Self {
        self.title = title.to_string();

        self
    }

    pub fn with_text(&mut self, text: impl ToString) -> &mut Self {
        self.text = text.to_string();

        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }
}
