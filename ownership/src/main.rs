fn main() {
    println!("What is ownership");
    println!("------------------------------");
    what_is_ownership();
}

fn what_is_ownership() {
    /*      What is Ownership?      */
    // set of rules that governs how a Rust program manages memory.

    // memory is manages through a system of ownership with a set of rules that the compiler checks. if any of the rules are violated, the program won't compile. None of the features of ownership will slow down your program while it's running.


    /*      The Stack and the Heap      */
    // systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions. Parts of ownership will be described in relation to the stack and the heap

    // STACK
    // Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. The stack stored values in order it gets them of a stack of plates: whe you add more plates, you put them on top of the pile, and when you need a plate, you take on off the top.(FIFO structure). Adding or removing plates from the middle or bottom wouldn't work as well
    // Adding data called pushing onto the stack must have a know, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead

    // HEAP
    // The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considers allocating). Because the pointer to the heap is a known fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the staff finds and empty table that fits everyone and leads you there. If someone in your group comes late, the can ask where you've been seated to find you.

    // Pushing to the stack is faster then allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
    // Comparatively, allocating space on the heap requires more work, because allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

    // Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. Continuing the analogy, consider a server at restaurant taking orders from many tables. It's most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, the one from A again, and then one from B again would be a much slower process. By the same token, a processor can do its job better if it works on data that's close to other data (as it is on the stack) rather than farther away(as it can be on the heap)

    // When your code calls a function, the values passed into the function (including, potentiality, pointers to data on the heap) and the functions's local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

    // Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of space are all problems that ownership addresses.

    // STACK => FIXED SIZE, FIFO structure
    // HEAP => "allocated", Various size, return pointer


    /*      OWNERSHIP RULES     */
    // Each value in Rust has owner
    // There can only be one owner at time
    // When the owner goes out of scope, the value will be dropped.


    /*      Variable Scope      */
    {
                            // s is not valid here, it's not yet declared
        let _s = "hello!";   // s is valid from ths point forward
        // do stuff with s ...
    }                       // this scope is now over, and s is no longer valid
    

    // The variable s refers to string literal, where the value of the string is hardcoded into the text of our program. The variable is valid from the point at which it's declared until the end of the current scope.

    // When s comes into scope, it is valid
    // it remains valid until it goes out of scope


    /*      THE String Type     */
    {
        let _s = String::from("Hello");
        // the double colon "" operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.
        
        // This kind of string can be mutated:
        let mut s = String::from("hello");
        s.push_str(", world!");    // push_str() appends a literal to a String 
        println!("{}", s);           // This will print `hello world!`

        // Why can String be mutated but literals cannot? The difference is how these two types deal with memory
    }

    
    /*      Memory and Allocation       */
    // In the case of string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fact and efficient. But these properties only come from the string literal's immutability. Unfortunately, we can't put a blob of memory into the binary for each piece of text whose size is unknown at compiler time and whose size might change while running the program.

    // With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.

        // The memory must be requested from the memory allocator at runtime.
        // We need a way of returning this memory to the allocator when we're done with our String.

    // That first part is done by us; when we call "String::from". its implementation requests the memory it needs. This is pretty much universal in programming languages.

    // However, the second part is different. In languages with a garbage collector (GC), the GC Keeps track of and cleans up memory that isn't begin used anymore, and we don't need to think about it. In most languages without a GC, it's our responsibility to identify when memory is no longer being used and call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we'll waste memory. If we do it too early, we'll have an invalid variable. If we do it twice, that's a bug too. We need to pair exactly one allocate with exactly one free.

    // Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
    {
        let _s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }   // this scope is now over, and s is no longer valid

    // There is a natural point at which we can return the memory our String need to the allocator: whe s goes out of scope. Whe a variable goes out of scope, Rust calls a special function for us. This function called drop, and it's where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.
    
        /*      Ways Variables and Data Interact: Move      */
        
} 
