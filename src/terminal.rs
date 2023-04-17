use crate::state::State;

pub struct Terminal {
    state: State,
}

impl Terminal {
    pub async fn new(state: State) -> Self {
        Self {
            state,
        }
    }
}
