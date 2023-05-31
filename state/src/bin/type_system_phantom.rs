use std::marker::PhantomData;

pub struct Draft;
pub struct PendingReview;
pub struct Published;

pub struct Post<T> {
    content: String,
    _state: PhantomData<T>,
}

#[allow(clippy::new_without_default)]
impl Post<Draft> {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            _state: PhantomData::<Draft>,
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> Post<PendingReview> {
        Post {
            content: self.content,
            _state: PhantomData::<PendingReview>,
        }
    }
}

impl Post<PendingReview> {
    pub fn approve(self) -> Post<Published> {
        Post {
            content: self.content,
            _state: PhantomData::<Published>,
        }
    }
}

impl Post<Published> {
    pub fn content(&self) -> &str {
        &self.content
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
