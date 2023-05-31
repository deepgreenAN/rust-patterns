#[derive(Debug)]
pub struct StateError;

pub enum Post {
    Draft(String),
    Pending(String),
    Publish(String),
}

impl Default for Post {
    fn default() -> Self {
        Post::Draft(String::new())
    }
}

impl Post {
    pub fn new() -> Self {
        Post::Draft(String::new())
    }
    pub fn request_review(&mut self) -> Result<(), StateError> {
        match self {
            Post::Draft(content) => {
                *self = Post::Pending(std::mem::take(content));
                Ok(())
            }
            _ => Err(StateError),
        }
    }
    pub fn add_text(&mut self, text: &str) -> Result<(), StateError> {
        match self {
            Post::Draft(content) => {
                content.push_str(text);
                Ok(())
            }
            _ => Err(StateError),
        }
    }
    pub fn approve(&mut self) -> Result<(), StateError> {
        match self {
            Post::Pending(content) => {
                *self = Post::Publish(std::mem::take(content));
                Ok(())
            }
            _ => Err(StateError),
        }
    }
    pub fn article(&self) -> Result<&str, StateError> {
        match self {
            Post::Publish(content) => Ok(content),
            _ => Err(StateError),
        }
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today").unwrap();

    assert!(matches!(post.article(), Err(StateError)));

    post.request_review().unwrap();

    post.approve().unwrap();

    assert!(matches!(post.article(), Ok(_)));
}
