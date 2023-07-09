/*
- reject 메소드를 추가하여 게시물의 상태를 PendingReview에서 Draft로 변경하기
- Published로 상태 변경이 가능해지기 전에 approve가 두 번 호출되도록 요구하기
- 게시물이 Draft 상태일 때는 사용자들에게 글 내용의 추가만 허용하기. 힌트: 상태 객체가 내용에 관한 변경에는 역할을 하지만 Post를 수정하기 위한 역할은 하지 않게 하기
 */

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// enum?

trait State {
    // 이 문법은 메소드가 오직 그 타입을 보유한 Box 상에서 호출될 경우에만 유효함
    // 이전 상태를 무효화 시키기 위해, 상태의 소유권을 취할 필요가 있다.
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview;

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published;
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
