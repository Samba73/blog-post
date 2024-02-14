use blog::Post;
fn main() {
    let mut post = Post::new();
    post.add_text("Hello, world!");

    println!("The post was created {}", post.content)
}
