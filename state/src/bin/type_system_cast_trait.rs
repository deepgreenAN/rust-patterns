pub struct Post {
    content: String,
}

impl Post {
    fn content(&self) -> &str {
        &self.content
    }
}
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

pub struct PendingReviewPost {
    content: String,
}

// -------------------------------------------------------------------------------------------------
// キャスト用のトレイト

trait Transition<T> {
    fn transition(self) -> T;
}

impl Transition<PendingReviewPost> for DraftPost {
    fn transition(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl Transition<Post> for PendingReviewPost {
    fn transition(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

fn main() {
    let mut post = DraftPost::new();

    post.add_text("I ate a salad for lunch today");

    let post: PendingReviewPost = post.transition();

    let post: Post = post.transition();

    assert_eq!(post.content(), "I ate a salad for lunch today");
}
