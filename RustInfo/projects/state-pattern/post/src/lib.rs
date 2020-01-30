//! # post
//!
//! A library for the workflow of post publishing.
//! Use state pattern design.
//!
//! ## The flow
//!
//! 1. A new post from blank `Draft`
//! 2. After `Draft` done, request to review
//! 3. After review done, post will be published
//! 4. Only the published post could be printed


/// The basic state trait, different states will cause
/// different actions.
trait State {
    // 使用Self，方法调用支队该类型的Box有效
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn contents<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

/// Basic post struct, store post content and current state
pub struct Post {
    // use Box<dyn trait> as dynamic type
    // 使用 Option 类型，因为struct的字段不可为空,同时在改变状态时
    // 需要取出 state的所有权，然后变为None, 再赋予新的状态
    state: Option<Box<dyn State>>,
    contents: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            contents: String::new(),
        }
    }

    pub fn contents(&self) -> &str {
        self.state.as_ref().unwrap().contents(self)
    }

    pub fn add_text(&mut self, text: &str) {
        self.contents.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

/// The `Draft` state, a new post's state
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

/// The `PendingReview` state, after `Draft`
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

/// The `Published` state
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn contents<'a>(&self, post: &'a Post) -> &'a str {
        &post.contents
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_return_spefic_contents_in_spefic_state() {
        let mut post = Post::new();

        post.add_text("Just a simple post with none title");
        assert_eq!("", post.contents());

        post.request_review();
        assert_eq!("", post.contents());

        post.approve();
        assert_eq!("Just a simple post with none title", post.contents());
    }
}
