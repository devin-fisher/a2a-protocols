use crate::eventsourcing_derive::*;
use crate::serde_derive::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Move {

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Outcome {

}

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("0.1")]
#[event_source("did:sov:SLfEi9esrjzybysFxQZbfq;spec/tictactoe/1.0")]
pub enum TTTEvents {
    SendMove(Move),
    ReceiveMove(Move),
    SendOutcome(Move),
    SendReceive(Move)
}

#[cfg(test)]
mod tests {
    #[test]
    fn noop() {

    }
}