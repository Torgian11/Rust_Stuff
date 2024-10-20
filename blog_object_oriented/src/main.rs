use blog_object_oriented::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a sandwich today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a sandwich today", post.content());
}
