

#[derive(Clone, Debug, Default)]
pub struct GameState {
    last_message: String,
}


pub fn update(state: &mut GameState, input: String) -> String {
    let ret = state.last_message.clone();
    state.last_message = input;
    ret
}
