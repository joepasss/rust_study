#### GETTING STARTED

---

##### installing rustup on macOS

```
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

##### HELLO WORLD!

```
// in main.rs

fn main() {
  println!("Hello World!")
}

// console
// build
$ rustc main.rs

// run
$ ./main
```

##### HELLO CARGO!
```
// in console
$ cargo --version
```

1. creating a project with Cargo
```
$ cargo new hello_cargo
$ cd hello_cargo
```
