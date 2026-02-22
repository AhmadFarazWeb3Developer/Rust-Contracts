# 🔹 What Are Macros in Rust?

1. **Macros are metaprogramming tools** — they let you write code that writes other code.
2. They are **expanded at compile-time**, not runtime.
   - This is different from functions, which execute at runtime.

3. Macros are used to:
   - Reduce **repetition / boilerplate**
   - Generate code dynamically
   - Increase flexibility

---

# 🔹 Two Main Types of Macros

### 1️⃣ Declarative Macros (`macro_rules!`)

- Easier to write and most common.
- Pattern-based: you define patterns and placeholders.
- Examples: `println!()`, `panic!()`, `vec![]`

### 2️⃣ Procedural Macros

- More advanced, used for **custom code generation**.
- Three kinds:
  1. **Custom derive macros**: e.g., `#[derive(Debug)]`, `#[derive(Serialize)]`
  2. **Attribute-like macros**: e.g., `#[route("/path")]` in web frameworks
  3. **Function-like macros**: e.g., `sql!()` that processes input like a function

---

# 🔹 Your Code Explained

### 1. Simple Declarative Macro

```rust
macro_rules! say_hello {
    () => {
        println!("Hello world")
    };
}
```

- When you call `say_hello!()`, the compiler **replaces it with `println!("Hello world")`**.
- **Benefit:** no runtime cost, reduces typing for repeated code.

---

### 2. Parameterized Declarative Macro

```rust
macro_rules! repeat_message {
    ($msg:expr, $times:expr) => {
        for _ in 0..$times {
            println!("{}", $msg);
        }
    };
}
```

- `$msg` and `$times` are **placeholders**.
- `$expr` means **expression**.
- Usage:

```rust
repeat_message!("Rust is awesome!", 3);
```

- Compiler generates the `for` loop **3 times** at compile-time.

---

### 3. Type-based Macro

```rust
macro_rules! create_vector {
    ($type:ty) => {
        fn new_vector() -> Vec<$type> {
            Vec::new()
        }
    };
}
```

- `$type:ty` is a **type placeholder**.
- You can generate a function that returns a **vector of any type**.
- Usage:

```rust
create_vector!(i32);
let mut my_vec = new_vector(); // returns Vec<i32>
```

- This avoids writing a separate `new_vector_i32()` or `new_vector_string()` function.

---

### 4. Procedural Macro Example

```rust
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}
```

- `#[derive(Serialize)]` is a **procedural macro**.
- It **auto-generates code** to convert `User` into JSON.
- Example usage:

```rust
println!("{}", serde_json::to_string(&user).unwrap());
```

- Without this macro, you would need to manually write serialization logic for `User`.

---

# 🔹 Why Macros Are Important in Rust

1. **Zero runtime cost** – code is generated at compile-time.
2. **Eliminates repetitive boilerplate** – e.g., `println!`, `vec![]`.
3. **Enables generic code generation** – functions, structs, or traits dynamically.
4. **Powerful procedural macros** allow advanced features like `#[derive(Serialize)]`, web routing, and more.

---

# 🔹 Summary of Key Concepts

| Feature                | Example / Concept               | Notes                                       |
| ---------------------- | ------------------------------- | ------------------------------------------- |
| Declarative macro      | `macro_rules! say_hello`        | Simple pattern-based macros                 |
| Parameterized macro    | `repeat_message!($msg, $times)` | Accepts expressions, repeats code           |
| Type macro             | `create_vector!($type)`         | Generates functions/types dynamically       |
| Procedural macro       | `#[derive(Serialize)]`          | Generates code for traits, attributes, etc. |
| Compile-time expansion | –                               | No runtime overhead                         |
| Reduces boilerplate    | –                               | Saves time, increases productivity          |

---

### 🔹 Mental Model:

- Think of macros as **templates or blueprints**:
  - You tell Rust: _“Whenever I call this macro, generate this code for me.”_

- Example analogy:
  - `repeat_message!("Hello", 3)` → Rust writes the `for` loop **three times automatically**.

---

If you want, I can create a **diagram showing declarative vs procedural macros**, and how they expand into actual Rust code **under the hood**, which makes it extremely visual and easy to understand.

Do you want me to do that?
