/// 状態の変更に失敗したときのエラー．今回は分かりやすさのため用いているが，状態遷移を何もしない実装にすべき場合も多い
#[derive(Debug)]
pub struct StateError;

trait PostState {
    fn request_review(self: Box<Self>) -> Result<Box<dyn PostState>, StateError> {
        Err(StateError)
    }
    fn approve(self: Box<Self>) -> Result<Box<dyn PostState>, StateError> {
        Err(StateError)
    }
    fn content<'a>(&self, _post: &'a Post) -> Result<&'a str, StateError> {
        Err(StateError)
    }
    /// あくまでもスタック領域のPostを使うことで(Stateを実装した型に状態を持たせない)コストを最小限にする．
    fn add_text<'a>(&self, _post: &'a mut Post, _text: &str) -> Result<(), StateError> {
        Err(StateError)
    }
}

pub struct Post {
    pub(crate) state: Option<Box<dyn PostState>>,
    pub(crate) content: String,
}

#[allow(clippy::new_without_default)]
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft)),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) -> Result<(), StateError> {
        if let Some(post_state) = self.state.take() {
            post_state.add_text(self, text)?;
            self.state = Some(post_state);
            Ok(())
        } else {
            panic!("state does not have value") // 簡単のため．実際はエラーを伝播
        }
    }
    pub fn request_review(&mut self) -> Result<(), StateError> {
        if let Some(post_state) = self.state.take() {
            self.state = Some(post_state.request_review()?);
            Ok(())
        } else {
            panic!("state does not have value") // 簡単のため．実際はエラーを伝播
        }
    }
    pub fn approve(&mut self) -> Result<(), StateError> {
        if let Some(post_state) = self.state.take() {
            self.state = Some(post_state.approve()?);
            Ok(())
        } else {
            panic!("state does not have value") // 簡単のため．実際はエラーを伝播
        }
    }
    pub fn content(&self) -> Result<&str, StateError> {
        if let Some(post_state) = self.state.as_ref() {
            post_state.content(self)
        } else {
            panic!("state does not have value") // 簡単のため．実際はエラーを伝播
        }
    }
}

// -------------------------------------------------------------------------------------------------
// トレイトオブジェクトの型の定義

struct Draft;
struct PendingReview;
struct Published;

impl PostState for Draft {
    fn add_text<'a>(&self, post: &'a mut Post, text: &str) -> Result<(), StateError> {
        post.content.push_str(text);
        Ok(())
    }
    fn request_review(self: Box<Self>) -> Result<Box<dyn PostState>, StateError> {
        Ok(Box::new(PendingReview))
    }
}

impl PostState for PendingReview {
    fn approve(self: Box<Self>) -> Result<Box<dyn PostState>, StateError> {
        Ok(Box::new(Published))
    }
}

impl PostState for Published {
    fn content<'a>(&self, post: &'a Post) -> Result<&'a str, StateError> {
        Ok(&post.content)
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today").unwrap();
    assert!(matches!(post.content(), Err(_)));

    post.request_review().unwrap();
    assert!(matches!(post.content(), Err(_)));

    post.approve().unwrap();
    assert_eq!("I ate a salad for lunch today", post.content().unwrap());
}
