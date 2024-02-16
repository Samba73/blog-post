use blog::Post;
fn main() {
    let mut post = Post::new();
    post.add_text("Hello, world!");

    post.request_review();
    // println!("The state of the post is {:?}", post.state);
    println!("The post was created {}", post.content());              

    post.approve();
    println!("The post was approved {}", post.content()); 


}