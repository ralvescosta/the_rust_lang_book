mod lib;
use lib::Post;

fn main() {
    let mut post = Post::new();
    let post_text = "I ate a salad for lunch today";

    post.add_text(post_text);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(post_text, post.content());
}
