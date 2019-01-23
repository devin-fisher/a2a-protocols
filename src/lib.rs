pub mod api;
mod aggregate;
mod event;
mod command;

enum Roles {
    Player
}

enum MachineStates {
    MyMove,
    TheirMove,
    WrapUp,
    Done
}



#[cfg(test)]
mod protocol_tests {
    #[test]
    fn noop() {

    }

    #[test]
    fn test_create() {
    }
}
