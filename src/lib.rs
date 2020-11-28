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
/// that returns FSM\<T\>.
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
/// struct ExampleMachine{
///     x: i32,
/// }
///
/// // States
/// impl ExampleMachine {
///     fn a(&mut self) -> FSM<ExampleMachine> {
///         self.x += 1;
///         println!("A: {}", self.x);
///         FSM::transition(*self, Self::b)
///     }
///     fn b(&mut self) -> FSM<ExampleMachine> {
///         self.x += 1;
///         println!("B: {}", self.x);
///         FSM::transition(*self, Self::c)
///     }
///     fn c(&mut self) -> FSM<ExampleMachine> {
///         self.x += 1;
///         println!("C: {}", self.x);
///         FSM::transition(*self, Self::a)
///     }
/// }
///
/// fn test_fsm() {  
///     // Initialize
///     // state is fn a(...) -> ...
///     let mut state = FSM::transition(ExampleMachine{x: 0}, ExampleMachine::a);
///     
///     // state is now fn b(...) -> ...
///     state.update();
///
///     //state is now fn c(...) -> ...
///     state.update();
///
///     // output:
///     // A: 1
///     // B: 2
///     // C: 3
/// }
/// ```
#[derive(Copy)]
pub struct FSM<T>
where
    T: Copy + Clone,
{
    machine: T,
    state: State<T>,
}

impl<T> FSM<T>
where
    T: Copy + Clone,
{
    /// transition one step.
    pub fn update(&mut self) {
        let fsm = self(&mut self.machine);
        self.state = fsm.state;
        self.machine = fsm.machine;
    }
    /// Move to new state.
    pub fn transition(machine: T, func: State<T>) -> FSM<T> {
        FSM {
            machine: machine,
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
        self.machine = source.machine;
    }
}
