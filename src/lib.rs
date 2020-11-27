//! # CFSM
//!
//! `cfsm`, also called Celezt's Finite State Machine is a
//! library that implements FSM to rust.
//! It uses function pointers as state and struct as machine.

type State<T> = fn(&mut T) -> FSM<T>;

/// Finite State Machine using fn-pointer
///
/// Parsing unique struct that has `copy` + `clone` derived. To implement a new
/// machine, create an impl for that struct that contains all states as fn
/// that returns FSM\<T\>. It is required to call ::new before using it and
/// can be overwritten.
///
/// # Examples
///
/// Basic usage with ExampleMachine, a struct that implements `Copy` + `Clone`:
///
/// ```
/// use cfsm::FSM;
///
/// // Machine
/// #[derive(Debug, Copy, Clone)]
/// struct ExampleMachine;
///
/// // States
/// impl ExampleMachine {
///     fn a(&mut self) -> FSM<ExampleMachine> {
///         println!("A");
///         FSM::transition(Self::b)
///     }
///     fn b(&mut self) -> FSM<ExampleMachine> {
///         println!("B");
///         FSM::transition(Self::c)
///     }
///     fn c(&mut self) -> FSM<ExampleMachine> {
///         println!("C");
///         FSM::transition(Self::a)
///     }
/// }
///
/// fn test_fsm() {  
///     // Initialize
///     // state is fn a(...) -> ...
///     let mut state = FSM::new(ExampleMachine::a, &ExampleMachine);
///     
///     // state is now fn b(...) -> ...
///     state.update();
///
///     //state is now fn c(...) -> ...
///     state.update();
///     
///     // it is possible to overwrite with a new struct and fn pointer
///     // state is fn a(...) -> ...
///     state = FSM::new(ExampleMachine::a, &ExampleMachine);
///     
///     // state is now fn b(...) -> ...
///     state.update();
///
///     // output:
///     // A
///     // B
///     // C
///     // A
///     // B
/// }
/// ```
#[derive(Copy)]
pub struct FSM<T>
where
    T: Copy + Clone,
{
    state_type: Option<T>,
    state: State<T>,
}

impl<T> FSM<T>
where
    T: Copy + Clone,
{
    /// Return state.
    pub fn get(&self) -> State<T> {
        self.state
    }
    /// transition one step.
    pub fn update(&mut self) {
        // If machine exist
        if self.state_type.is_some() {
            self.state = self(&mut self.state_type.unwrap()).state;
        } else {
            panic!("FSM was None");
        }
    }
    /// Initialize FSM.
    pub fn new(func: State<T>, state_type: &T) -> FSM<T> {
        FSM {
            state_type: Some(*state_type),
            state: func,
        }
    }
    /// Move to new state.
    pub fn transition(func: State<T>) -> FSM<T> {
        FSM {
            state_type: None,
            state: func,
        }
    }
}

impl<T> std::ops::Deref for FSM<T>
where
    T: Copy + Clone,
{
    type Target = State<T>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T> Clone for FSM<T>
where
    T: Copy + Clone,
{
    fn clone(&self) -> Self {
        *self
    }

    fn clone_from(&mut self, source: &Self) {
        self.state = source.state;
        // Copy only if source machine is not none
        if source.state_type.is_some() {
            self.state_type = source.state_type;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Copy, Clone)]
    struct TestState;

    /// For test purposes
    impl TestState {
        fn a(&mut self) -> FSM<TestState> {
            println!("A");
            FSM::transition(Self::b)
        }
        fn b(&mut self) -> FSM<TestState> {
            println!("B");
            FSM::transition(Self::c)
        }
        fn c(&mut self) -> FSM<TestState> {
            println!("C");
            FSM::transition(Self::a)
        }
    }

    #[test]
    fn test_fsm() {
        // Initialize
        let mut state = FSM::new(TestState::a, &TestState);

        for _ in 0..20 {
            state.update();
        }

        state = FSM::new(TestState::a, &TestState);
        state.update();
    }
}
