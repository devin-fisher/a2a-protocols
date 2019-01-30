extern crate eventsourcing;
extern crate eventsourcing_derive;

extern crate serde;
extern crate serde_derive;

pub mod api;
mod aggregate;
mod events;
mod commands;
mod roles;
mod states;
mod board;

use crate::aggregate::TTTAggregateState;
use crate::roles::Roles;
use crate::events::TTTEvents;

fn create(role: Roles) -> TTTAggregateState {
    TTTAggregateState{
        role
    }
}

fn event(state: TTTAggregateState, event: TTTEvents) -> TTTAggregateState  {
    state
}

#[cfg(test)]
mod protocol_tests {
    use crate::*;
    use crate::aggregate::*;
    use crate::roles::*;
    use crate::events::*;

    #[test]
    fn noop() {

    }

    #[test]
    fn test_create() {
        let state: TTTAggregateState = create(Roles::Player);
        assert!(state.role == Roles::Player)
    }

    #[test]
    fn test_receive_event() {
        let e = TTTEvents::ReceiveMove(Move{});
        let state: TTTAggregateState = create(Roles::Player);
        let state = event(state, e);
    }
}