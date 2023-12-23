use tiny_web::sys::action::{Answer, Action, Redirect};

pub async fn index(this: &mut Action) -> Answer {
    this.response.redirect = Some(Redirect { url: "/admin".to_owned(), permanently: false });
    Answer::None
}

pub async fn not_found(_: &mut Action) -> Answer {
    Answer::String("not_found".to_owned())
}