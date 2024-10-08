use tiny_web::sys::action::{Action, Answer};

pub async fn index(_this: &mut Action) -> Answer {
    Answer::None
}

pub async fn not_found(_this: &mut Action) -> Answer {
    Answer::None
}

pub async fn err(_this: &mut Action) -> Answer {
    Answer::None
}
