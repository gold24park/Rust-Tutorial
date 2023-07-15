pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

pub struct PublishedPost {
    content: String,
}

impl Post {
    pub fn new() -> Self {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        ""
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> PublishedPost {
        PublishedPost { content }
    }
}

impl PublishedPost {
    pub fn content(self) -> &str {
        &self.content
    }
}
