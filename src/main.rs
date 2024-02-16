use blog::Post;
fn main() {
    let mut post = Post::new();
    post.add_text("Hello, world!");

    let post = post.request_review();

    println!("The initial post is {:?}", post);

    let post = post.reject();
    println!("The post is rejected {:?}", post);

    let mut new_post = Post::new();

    new_post.add_text("The content is revised now for review");
    let new_post = new_post.request_review();
    println!("The post now is {:?}", new_post);

    let new_post = new_post.approve();
    println!("The approved post is {:?}", new_post);

    println!("The final post content is {}", new_post.content());


}