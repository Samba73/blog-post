use blog::Post;
fn main() {
    let mut post = Post::new();
    post.add_text("Hello, world!");

    let post = post.request_review();

    let post = post.approve();

    println!("The post content is {}", post.content());


}