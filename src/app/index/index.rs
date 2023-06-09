use tiny_web::sys::action::{Answer, Action};

pub async fn index(this: &mut Action) -> Answer {
    this.load("head", "index", "main", "head", None).await;
    this.load("index", "index", "main", "index", None).await;
    this.load("foot", "index", "main", "foot", None).await;
    this.render("index").await
}

pub async fn not_found(this: &mut Action) -> Answer {
    this.load("head", "index", "main", "head", None).await;
    this.load("index", "index", "main", "not_found", None).await;
    this.load("foot", "index", "main", "foot", None).await;
    this.render("index").await
}