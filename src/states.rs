struct MyMove;
struct TheirMove;
struct WrapUp;
struct Done;

impl From<TheirMove> for MyMove {
    fn from(val: TheirMove) -> MyMove {
        MyMove
    }
}

impl From<MyMove> for TheirMove {
    fn from(val: MyMove) -> TheirMove {
        TheirMove
    }
}

impl From<MyMove> for WrapUp {
    fn from(val: MyMove) -> WrapUp {
        WrapUp
    }
}

impl From<TheirMove> for WrapUp {
    fn from(val: TheirMove) -> WrapUp {
        WrapUp
    }
}

impl From<MyMove> for Done {
    fn from(val: MyMove) -> Done {
        Done
    }
}

impl From<TheirMove> for Done {
    fn from(val: TheirMove) -> Done {
        Done
    }
}

impl From<WrapUp> for Done {
    fn from(val: WrapUp) -> Done {
        Done
    }
}

#[cfg(test)]
mod tests {
    use crate::states::TheirMove;
    use crate::states::MyMove;
    use crate::states::WrapUp;
    use crate::states::Done;

    #[test]
    fn my_move_to_their_move() {
        let my_move = MyMove;
        let their_move: TheirMove = my_move.into();
    }

    #[test]
    fn their_move_to_my_move() {
        let their_move = TheirMove;
        let my_move: MyMove = their_move.into();
    }

    #[test]
    fn my_move_to_wrap_up() {
        let my_move = MyMove;
        let wrap_up: WrapUp = my_move.into();
    }

    #[test]
    fn their_move_to_wrap_up() {
        let their_move = TheirMove;
        let wrap_up: WrapUp = their_move.into();
    }

    #[test]
    fn my_move_to_done() {
        let my_move = MyMove;
        let done: Done = my_move.into();
    }

    #[test]
    fn their_move_to_done() {
        let their_move = TheirMove;
        let done: Done = their_move.into();
    }

    #[test]
    fn wrap_up_to_done() {
        let wrap_up = WrapUp;
        let wrap_up: Done = wrap_up.into();
    }
}