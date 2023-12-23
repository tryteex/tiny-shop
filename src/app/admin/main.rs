use tiny_web::sys::action::{Answer, Action};

pub async fn index(_: &mut Action) -> Answer {
    Answer::String("admin/main/index".to_owned())
}