use blog::{Post, RustPost};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut rust_post = RustPost::new();

    rust_post.add_text("I ate a salad for lunch today");
    let rust_post = rust_post.request_review();
    let rust_post = rust_post.approve();

    assert_eq!("I ate a salad for lunch today", rust_post.content());
}
