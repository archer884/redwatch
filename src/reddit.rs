use hashbrown::HashMap;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "postIds")]
    pub ids: Vec<String>,
    posts: HashMap<String, Post>,
}

impl Response {
    pub fn posts<'a>(&'a self) -> impl Iterator<Item = &Post> + 'a {
        self.ids.iter().filter_map(move |id| self.posts.get(id))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub permalink: String,
}

impl Post {
    pub fn keywords<'a>(&'a self) -> impl Iterator<Item = String> + 'a {
        self.title
            .split_whitespace()
            .map(|x| x.to_ascii_lowercase())
    }
}
