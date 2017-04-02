extern crate scrappy;

#[test]
fn it_should_return_none_on_invalid_url() {
    assert_eq!(scrappy::get_content("google"), None);
}

#[test]
fn it_should_return_content() {
    assert_eq!(scrappy::get_content("www.google.com").unwrap().find("google").is_some(), true);
    assert_eq!(scrappy::get_content("www.google.com").unwrap().find("adasdasdadasdvcxvxvcx"), None);
}
