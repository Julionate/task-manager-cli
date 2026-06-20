use crate::enums::AppState;

pub fn exit() -> AppState {
    println!("Closing...");
    AppState::Exit
}
