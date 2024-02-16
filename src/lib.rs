#[allow(dead_code)]
#[allow(unused)]
// states: Draft, Pending Review, Published
// New Post -> Draft, In `Draft state` add text to post
// Draft with text -> Move to `Pending Review` state
// Approve the post -> Post moved to `Published` state
#[allow(dead_code)]
// trait State {
//    fn request_review(self: Box<Self>) -> Box<dyn State>;
//    fn approve(self: Box<Self>) -> Box<dyn State>;
//    fn content<'a>(&self, post: &'a Post) -> &'a str {
//     ""
//    } 
// }
#[derive(Debug)]
pub struct Post {
    content: String,
}

impl Post {

    // function to create a new post
    pub fn new() -> DraftPost {
        DraftPost{
            content: String::new(),
        }

    }

    pub fn content(&self) -> &str {
        &self.content
    }
    
}
#[derive(Debug)]
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost{
            content: self.content,
        }
    }
}
#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {

    pub fn approve(self) -> Post {
        Post{
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost{
            content: self.content,
        }
    }
}
// struct Draft {

// }
// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State>{
//         Box::new(PendingReview{})
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State>{
//         self
//     }

// }

// struct PendingReview {

// }
// impl State for PendingReview {
//     fn request_review(self: Box<Self>)-> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published{})
//     }

//  }

//  struct Published {

// }

// impl State for Published {
//     fn request_review(self: Box<Self>)-> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }