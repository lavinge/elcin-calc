use crate::controller::reference::{Topic, TopicId};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ReferenceBrowser {
    topics: HashMap<String, Topic>,
}

impl ReferenceBrowser {
    pub fn new(topics: HashMap<String, Topic>) -> Self {
        Self { topics }
    }

    pub fn empty() -> Self {
        Self {
            topics: HashMap::new(),
        }
    }

    pub fn summary(&self, topic_id: TopicId) -> Topic {
        match self.topics.get(&topic_id.to_string()) {
            Some(topic) => topic.clone(),
            None => Topic::new()
                .with_id(topic_id)
                .with_title("Заголовок отсутствует")
                .with_text("Текст отсутствует"),
        }
    }
}
