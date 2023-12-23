use tiny_web::sys::action::{Answer, Action, Redirect};

pub async fn index(this: &mut Action) -> Answer {
    if this.session.user_id != 0 {
        this.response.redirect = Some(Redirect { url: "/admin/main".to_owned(), permanently: false });
        return Answer::None;
    }
    this.load("head", "admin", "index", "head", None).await;
    this.load("foot", "admin", "index", "foot", None).await;
    this.render("login")
}