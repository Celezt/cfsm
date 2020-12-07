 # CFSM

 `cfsm`, also called Celezt's Finite State Machine is a
 library that implements FSM to rust.
 It uses function pointers as state and struct as machine.

 Finite State Machine using fn-pointer

 Parsing unique struct that has `copy` + `clone` derived. To implement a new
 machine, create an impl for that struct that contains all states as fn
 that returns FSM\<T\>.

 # Examples

 Basic usage with ExampleMachine, a struct that implements `Copy` + `Clone`:

 ```
 use cfsm::FSM;

 // Machine
 #[derive(Copy, Clone)]
 struct ExampleMachine{
     x: i32,
 }

 // States
 impl ExampleMachine {
     fn a(&mut self) -> FSM<ExampleMachine> {
         self.x += 1;
         println!("A: {}", self.x);
         FSM::transition(*self, Self::b)
     }
     fn b(&mut self) -> FSM<ExampleMachine> {
         self.x += 1;
         println!("B: {}", self.x);
         FSM::transition(*self, Self::c)
     }
     fn c(&mut self) -> FSM<ExampleMachine> {
         self.x += 1;
         println!("C: {}", self.x);
         FSM::transition(*self, Self::a)
     }
 }

 fn test_fsm() {  
     // Initialize
     // state is fn a(...) -> ...
     let mut state = FSM::transition(ExampleMachine{x: 0}, ExampleMachine::a);
     
     // state is now fn b(...) -> ...
     state.update();

     //state is now fn c(...) -> ...
     state.update();

     println!("Expose: {}", state.get().x);

     state.get_mut().x = 0;

     println!("Expose: {}", state.get().x);

     // output:
     // A: 1
     // B: 2
     // C: 3
     // Expose: 3
     // Expose: 0
 }
 ```