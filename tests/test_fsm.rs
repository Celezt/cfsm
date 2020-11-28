#[cfg(test)]
mod tests {
    use cfsm::FSM;

    #[derive(Debug, Copy, Clone)]
    struct TestState {
        x: i32,
    }

    /// For test purposes
    impl TestState {
        fn a(&mut self) -> FSM<TestState> {
            self.x += 1;
            println!("A: {:?}", &self);
            FSM::transition(*self, Self::b)
        }
        fn b(&mut self) -> FSM<TestState> {
            self.x += 1;
            println!("B: {:?}", &self);
            FSM::transition(*self, Self::c)
        }
        fn c(&mut self) -> FSM<TestState> {
            self.x += 1;
            println!("C: {:?}", &self);
            FSM::transition(*self, Self::a)
        }
    }

    #[test]
    fn test_fsm() {
        // Initialize
        let mut state = FSM::transition(TestState { x: 0 }, TestState::a);

        for _ in 0..20 {
            state.update();
        }

        println!("Expose: {}", state.get().x);

        state.get_mut().x = 0;

        println!("Expose: {}", state.get().x);

        state.update();
    }
}
