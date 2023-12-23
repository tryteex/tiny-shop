use tiny_web::sys::action::{Answer, Action, Redirect};

pub async fn index(this: &mut Action) -> Answer {
    if this.session.user_id != 0 {
        this.response.redirect = Some(Redirect { url: "/admin/main".to_owned(), permanently: false });
    } else {
        this.response.redirect = Some(Redirect { url: "/admin/login".to_owned(), permanently: false });
    }
    Answer::None
}

pub async fn head(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect { url: "/admin/main".to_owned(), permanently: false });
    }
    this.render("head")
}

pub async fn foot(this: &mut Action) -> Answer {
    if !this.internal {
        this.response.redirect = Some(Redirect { url: "/admin/main".to_owned(), permanently: false });
    }
    this.render("foot")
}