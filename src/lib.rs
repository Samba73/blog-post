// states: Draft, Pending Review, Published



// New Post -> Draft, In `Draft state` add text to post
// Draft with text -> Move to `Pending Review` state
// Approve the post -> Post moved to `Published` state
#[allow(dead_code)]
trait State {
   fn request_review(self: Box<Self>) -> Box<dyn State>;
   fn approve(self: Box<Self>) -> Box<dyn State>;
}

pub struct Post {
    state: Option<Box<dyn State>>,
    pub content: String,
}

impl Post {

    // function to create a new post
    pub fn new() -> Post {
        Post{
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }

    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
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
    // fn content(&mut self) -> &str {
    //     self.state.as_ref().unwrap().content(self)
    // }
    
}
struct Draft {

}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        Box::new(PendingReview{})
    }
    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }
}

struct PendingReview {

}
impl State for PendingReview {
    fn request_review(self: Box<Self>)-> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
 }

 struct Published {

}

impl State for Published {
    fn request_review(self: Box<Self>)-> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}