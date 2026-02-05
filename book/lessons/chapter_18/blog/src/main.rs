use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.request_approval();
    assert_eq!("I ate lunch today", post.content());
}
