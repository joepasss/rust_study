// THE HEAP
// It's a region of the process memory that is NOT automatically managed
// we should manually allocate and free
// it has no size restrictions
// It's accessible by any function, anywhere in the program.
// Heap allocations are expensive and we should avoid them when possible

fn main() {
    let a = 2;
    stack_only(a);
}

fn stack_only(i32 b) {
    let c = 3;
}

fn stack_and_heap(i32 a) {
    let d = 5;
    // SOME HEAP VAR V
}


// heap

// 0xf123123 => some addr
// 7 => some value

// STACK
// V = 0xf123123 => in stack we store only address
