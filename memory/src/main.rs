// STACK
// special region of the process memory that stores variables created by each function
// for every function call a new stack frame is allocated on top of the current one.
// the size of every variable on the stack has to be known at compile time

fn main() {
    let a = 2;
    stack_only(a);
}

fn stack_only(i32 b) {
    let c = 3;
}

// stack

// stack_only
// b = 2
// c = 3
// has to store two value 

// main
// a = 2

// stack has limited size!