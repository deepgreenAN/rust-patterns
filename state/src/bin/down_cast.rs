use downcast::{downcast, Any};

trait Post: Any {}

downcast!(dyn Post);

struct DraftPost {
    content: String,
}

impl DraftPost {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }
    fn request_review(self: Box<Self>) -> Box<PendingReviewPost> {
        Box::new(PendingReviewPost {
            content: self.content,
        })
    }
}

impl Post for DraftPost {}

struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    fn approve(self: Box<Self>) -> Box<PublishedPost> {
        Box::new(PublishedPost {
            content: self.content,
        })
    }
}

impl Post for PendingReviewPost {}

struct PublishedPost {
    content: String,
}

impl PublishedPost {
    fn content(&self) -> &str {
        &self.content
    }
}

impl Post for PublishedPost {}

fn main() {
    use downcast::Downcast;

    let mut post: Option<Box<dyn Post>> = Some(Box::new(DraftPost::new()));

    {
        if let Some(post_inner) = post.take() {
            let mut draft_post = Downcast::<DraftPost>::downcast(post_inner).unwrap();
            draft_post.add_text("I ate a salad for lunch today");
            post = Some(draft_post.request_review())
        }
    }

    {
        if let Some(post_inner) = post.take() {
            let pending_review_post = Downcast::<PendingReviewPost>::downcast(post_inner).unwrap();
            post = Some(pending_review_post.approve())
        }
    }

    {
        if let Some(post_inner) = post.take() {
            let published_post = Downcast::<PublishedPost>::downcast(post_inner).unwrap();
            assert_eq!(published_post.content(), "I ate a salad for lunch today")
        }
    }
}
