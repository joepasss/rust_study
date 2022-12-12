/* Using Structs to Structure Related Data */
// struct is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

fn main() {
    println!("QUIZ!");
    quiz();
    divider();

    println!("\n");

    println!("Defining And Instantiating Structs");
    defining_and_instantiating();
    divider();

    println!("\n");

    println!("An Example Program Using Structs");
    structs_program();
    divider();

    println!("\n");

    println!("Method Syntax");
    method_syntax();
    divider();

}

fn divider() {
    println!("----------------");
}

fn quiz() {

}

fn defining_and_instantiating() {
struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user = User {
        email: String::from("some_email@example.com"),
        username: String::from("usernames"),
        active: true,
        sign_in_count: 1
    };

    println!("User structs active: {} username: {} email: {} sign_in_count: {}", user.active, user.username, user.email, user.sign_in_count);

    {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        println!("init user");
        println!("{}", user1.email);

        println!("change user1 information (email)");
        user1.email = String::from("anotheremail@example.com");
        println!("{}", user1.email);
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    {
        let user2: User = build_user(String::from("joepasss@gmail.com"), String::from("joepasss"));

        println!("init user using function");
        println!("{}", user2.email);
    }

    {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
    
        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };

        println!("using ... op");
        println!("{}", user2.email);
    }

    {
        // Using Tuple Structs without Named Fields to Create Different Types

        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }

}

fn structs_program() {
    // calculates the area of a rectangle.

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    {
        let width1 = 30;
        let height1 = 50;

        println!("The area of the rectangle is {} square pixels", area(width1, height1))
    }

    // Refactoring with Tuples
    fn area2(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
        
        // width is the tuple index 0 and height is the tuple index 1.
    }

    // Refactoring with Structs: Adding More Meaning
    
    struct Rectangle {
        width: u32,
        height: u32,
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("The area of the rectangle is {} square pixels", area3(&rect1));
    }

    fn area3(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    {
        #[derive(Debug)]
        struct Rectangle2 {
            width: u32,
            height: u32,
        }

        let rect = Rectangle2 {
            width: 30,
            height: 50,
        };

        println!("rect is {:?}", rect);
        println!("rect is {:#?}", rect);        // style the output
        dbg!(&rect);        // returns ownership of the expression's value
    }
}

fn method_syntax() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    {
        
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        let rect2 = Rectangle {
            width: 20,
            height: 30,
        };

        println!("The area of the rectangle is {} square pixels", rect1.area());
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        
        let square1 = Rectangle::square(32);

        println!("square1: {:?}", square1);
    }
}