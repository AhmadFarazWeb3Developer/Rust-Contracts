# 📚 Arrays vs Vectors in Rust

## Rust provides **arrays** and **vectors** to store collections of elements. They are similar but have important differences.

## 1️⃣ Arrays (`[T; N]`)

### Definition

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
```

- **Fixed size**: Must know the number of elements at compile time (`5` in this case).
- **Homogeneous**: All elements must have the same type (`i32`).
- **Stack-allocated**: Stored on the stack.
- **Immutable size**: You cannot add or remove elements after creation.

### Properties

| Feature                   | Array                 |
| ------------------------- | --------------------- |
| Size                      | Fixed at compile time |
| Mutable size              | ❌ No                 |
| Memory location           | Stack                 |
| Can hold dynamic elements | ❌ No                 |
| Access speed              | Fast, direct indexing |

### When to Use

- When the **number of elements is known and fixed**.
- When you want **fast, predictable memory usage**.
- For **small collections** where size won’t change.

---

## 2️⃣ Vectors (`Vec<T>`)

### Definition

```rust
let mut vec: Vec<i32> = Vec::new(); // empty vector
vec.push(1);
vec.push(2);

let vec2: Vec<i32> = vec![1, 2, 3, 4, 5]; // initialized with elements
```

- **Dynamic size**: Can grow or shrink at runtime using methods like `push()`, `pop()`.
- **Homogeneous**: All elements must have the same type (`i32`).
- **Heap-allocated**: Stored on the heap.
- **Resizing**: Rust automatically reallocates memory when needed.

### Properties

| Feature         | Vector                                                                |
| --------------- | --------------------------------------------------------------------- |
| Size            | Dynamic (can grow/shrink)                                             |
| Mutable size    | ✅ Yes (`push`, `pop`)                                                |
| Memory location | Heap                                                                  |
| Access speed    | Slightly slower than array due to heap indirection                    |
| Methods         | Many helper methods (`push`, `pop`, `insert`, `remove`, `iter`, etc.) |

### When to Use

- When the **number of elements is not known** at compile time.
- When you need to **add or remove elements dynamically**.
- For **large collections** where stack space might be insufficient.

---

## 3️⃣ Summary Comparison

| Feature        | Array                                | Vector                                |
| -------------- | ------------------------------------ | ------------------------------------- |
| Size           | Fixed                                | Dynamic                               |
| Mutable length | ❌                                   | ✅                                    |
| Memory         | Stack                                | Heap                                  |
| Initialization | `[1, 2, 3]`                          | `vec![1, 2, 3]` or `Vec::new()`       |
| Performance    | Slightly faster (no heap allocation) | Slightly slower (heap allocation)     |
| Use case       | Known-size small collections         | Unknown-size or resizable collections |

---

### Example

```rust
// Array: fixed size
let arr: [i32; 3] = [10, 20, 30];

// Vector: dynamic size
let mut vec = Vec::new();
vec.push(10);
vec.push(20);
vec.push(30);
```

- `arr` cannot grow beyond 3 elements.
- `vec` can grow as needed with `push()` or shrink with `pop()`.

---

✅ **Rule of Thumb:**

- Use **array** for **fixed-size, small collections**.
- Use **vector** for **dynamic, resizable collections** or **larger datasets**.

---

If you want, I can make a **diagram showing stack vs heap memory** for arrays and vectors to visualize it.

Do you want me to do that?
