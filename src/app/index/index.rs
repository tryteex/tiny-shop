use tiny_web::sys::action::{Answer, Action};

pub async fn index(this: &mut Action) -> Answer {
    Answer::String("index".to_owned())
}

pub async fn not_found(this: &mut Action) -> Answer {
    Answer::String("not_found".to_owned())
}