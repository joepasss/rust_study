fn main() {
    println!("What is ownership");
    println!("------------------------------");
    what_is_ownership();

    println!("");

    println!("References and Borrowing");
    println!("------------------------------");
    ref_and_borrowing();

    println!("");

    println!("QUIZ");
    println!("------------------------------");
    quiz();
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
    // Multiple variables can interact with the same data in different ways in Rust.
    {
        let x = 5;
        let _y = x;

        // bind the value 5 to x; then make a copy of the value in x and bind it to y;
        // integers are simple values with a known, fixed size, and these two 5 values are pushed onto stack.

        let s1 = String::from("Hello");
        let _s2 = s1;
        
        // a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. memory on the heap that holds the contents

        // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to.

        // both data pointers pointing to the same location. when s2 and s1 go out of scope, they will both try to free the same memory. This is known as double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

        // To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid, Therefore, Rust doesn't need to free anything when s1 goes out of scope.

        // the concept of coping the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of calling it a shallow copy. It's known as a move.

        // With only s2 valid, when it goes out of scope, it alone will free the memory
        // Rust will never automatically create "deep" copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance
    }


    /*      Ways Variables and Data Interact: Clone     */
    // if we do want to deeply copy the data of the string, not just the stack data, we can use a common method called clone.
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        
        println!("s1 = {}, s2 = {}", s1, s2);

        // When you se call to clone, you know that some arbitrary code being executed and that code may be expensive. It's a visual indicator that something different is going on.
    }


    /*      Stack-Only Data: Copy       */
    {
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);

        // that types such as integers that have a known size at compiler time are stored entires on the stack, so copies of the actual values are quick to make. That means there's no reason we would want to prevent x from being valid after we create the variable y. In other words, there's no difference between deep and shallow copying here, so calling clone wouldn't do anything different from the usual shallow copying and we can leave it out.

        // some of the types that implement copy:
        // All the integer types, such as u32.
        // The boolean type, bool, with values true and false
        // All the floating point types, such as f64
        // the character type, char.
        // Tuples, if they only contain types also implement Copy. (i32, i32) is but (i32, String) does not
    }


    /*      Ownership and Functions     */
    
}

fn quiz() {
    let mut s = String::from("hello");
    let s2 = &s;
    println!("{s2}");

    let s3 = &mut s;
    s3.push_str(" world");
    println!("{s3}");
    println!("{s}");
}

fn ref_and_borrowing() {
    // The issue with the tuple code in Listing 4-5 is that we have to return the String to the calling function so we can still use the String after the call to calculate_length, because the String was moved into calculate_length. Instead, we can provide a reference to the String value. A reference is like a pointer in that it's and address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

    fn calculate_length(s: &String) -> usize {
        s.len()
    }   // s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // The &s1 syntax lets us create a reference that refers to the value of s1. but does not own it. Because it does not own it, the value it points to will not be dropped when the reference stops being used.

    // Likewise, the signature of the functions uses & to indicate that the type of the parameter s is a reference.

    println!("The length of '{}' is {}. ", s1, len);

    // first notice that all the tuple code in the variable declaration and the function return value is gone. Second, not that we pass &s1 into calculate_length and, in its definition, we take &String rather than String. These ampersands represent references, and they allow you to refer to some value without taking ownership of it

    // The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

    // What happens if we try to modify something we're borrowing?
    fn _change(_some_string: &String) {
        // some_string.push_str(", world");
        // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    }
    // Just as variables are immutable by default, so are references. We're not allowed to modify something we have a reference to.


    /*          MUTABLE REFERENCES           */
    // We can fix the code from _change function to allow us to modify a borrowed value with just a few small tweaks that use, instead, a mutable reference:
    fn change_mut(some_string: &mut String) {
        some_string.push_str(", world");
    }

    let mut s2 = String::from("hello");
    change_mut(&mut s2);
    println!("{}", s2);

    // Mutable references have one big restriction: if you have mutable references to a value, you can have no other references to that value.

    let r1 = &mut s2;
    // ERROR
    // let r2 = &mut s2;
    // "second mutable borrow occurs here"

    println!("{}", r1);

    // we cannot borrow s2 as mutable more than once at a time. The first mutable borrow is in r1 and must last until it's used in the println!, but between in the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.

    // The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in very controlled fashion. It's something that new Rustaceans struggle with, because most languages let you mutate whenever you'd like. The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:
        
        // Tow or more pointers access the same data at the same time.
        // At least one of the pointers is begin use to write to the data.
        // There's no mechanism being used to synchronize access to the data
    
    // Data races cause undefined behavior and can be difficult to diagnose and fix when you're trying to track them down at runtime;
    // Rust prevents this problem by refusing to compile code with data races

    // We can use curly brackets to create new scope, allowing for multiple mutable references, just not simultaneous ones:

    let mut s3 = String::from("HELLO no3");

    {
        let _ref1 = &mut s3;
    }

    let _ref2 = &mut s3;

    // Rust enforces a similar rule for combination mutable and immutable references.
    /*
    result in an error:

    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;    // problem occurs
    */


    /*          IMPLICIT BORROWING          */
    // Borrows and references are used everywhere in Rust. So to make Rust programs less verbose, the Rust compiler has a number of strategies for implicitly creating borrows and converting references
    // mutable references can be moved by direct assignment
    {
        let mut s = String::from("Hello world");
        let s2 = &mut s;
        let _s3 = s2;

        // println!("{}", s2); // not valid because s2 is moved
    }

    // but mutable references are not moved by function calls
    {
        fn consume(_s: &mut String) {}
        let mut s = String::from("mutable references are not moved by function calls");
        let s2 = &mut s;
        consume(s2);
        println!("{}", s2);

        // this program works because Rust automatically reborrows mutable references when passed as input to a function call.
        // That way, Rust programmers don't have to keep creating new mutable references on every call. Inside the complier the call to consume is transformed to look like this
            // consume(&mut *s2);
        // therefore s2 is not moved by consume, but rather borrowed by consume 
    }


    /*          Dangling References         */
    // In language with pointers, it's easy to erroneously create a dangling pointer--a pointer that references a location in memory that may have been given to someone else--by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
    /*
    {
        let reference_to_nothing = dangle();
        
        fn dangle() -> &String {
            let s = String::from("hello");
            
            &s  // created inside of function
        }   // s goes out of scope, and is dropped. It memory goes away
    }

    this function's return type contains a borrowed value, but there is no value for it to be borrowed from

    */
}