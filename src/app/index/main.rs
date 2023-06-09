use tiny_web::sys::action::{Answer, Action};

pub async fn index(this: &mut Action) -> Answer {
    this.render("index").await
}

pub async fn head(this: &mut Action) -> Answer {
    this.render("head").await
}

pub async fn foot(this: &mut Action) -> Answer {
    this.render("foot").await
}

pub async fn not_found(this: &mut Action) -> Answer {
    this.render("not_found").await
}