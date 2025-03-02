// Tutorial - https://www.youtube.com/watch?v=-6cnnNlAvNk&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=9
// Docs - https://doc.rust-lang.org/rust-by-example/std/box.html

/*
* # Stack vs Heap in Rust
*
* ## Stack:
* - The stack is a **fast and efficient** memory allocation method.
* - It stores data in a **last-in, first-out (LIFO)** order.
* - Memory is automatically allocated and deallocated when a function scope ends.
* - Works best for **fixed-size** values (e.g., integers, booleans, etc.).
*
* ## Heap:
* - The heap is a **dynamically allocated** memory region.
* - It is used when **size is unknown at compile time** or when data needs to persist beyond a function scope.
* - Memory is manually allocated (e.g., with `Box<T>`, `Vec<T>`).
* - The **ownership system in Rust** ensures heap memory is safely managed.
 */

fn main() {
    // 🟢 Example of Stack Allocation
    let x = 2; // `x` is stored in the stack because it's a fixed-size integer (i32).
    let y = x; // `y` is also stored in the stack, and a **copy** of `x` is made.

    println!("Stack Values: x = {}, y = {}", x, y);

    // 🔴 Example of Heap Allocation
    let heap_num = Box::new(10);
    // `Box::new(10)` stores the integer `10` on the heap,
    // while `heap_num` (the pointer) is on the stack.

    println!("Heap Value: {}", heap_num);

    {
        // 🟡 Scope Example: Stack vs Heap
        let scoped_var = 5; // Stored in the stack (temporary)
        let heap_var = Box::new(50); // Allocated on the heap

        println!("Inside Scope: stack = {}, heap = {}", scoped_var, heap_var);
        // `scoped_var` is dropped when scope ends (stack memory freed).
        // `heap_var` is dropped as well, but Rust automatically deallocates heap memory.
    }

    // 🔄 Example of Heap Data Transfer
    let heap_data1 = Box::new(100); // Stored in heap
    let heap_data2 = heap_data1; // Moves ownership to `heap_data2`

    // println!("{}", heap_data1); // ❌ Error! `heap_data1` is no longer valid.

    println!("Heap Data after Move: {}", heap_data2);

    // ✅ Example of Heap Allocation in a Vector
    let mut v = vec![1, 2, 3]; // Vectors are heap-allocated
    v.push(4); // Dynamic memory allocation

    println!("Heap Vector: {:?}", v);
}

/*
 * 📌 Key Takeaways
 * Stack is for small, known-size data (fast access, no manual memory management).
 * Heap is for large, dynamically allocated data (slower, requires memory management).
 * Rust ensures memory safety using ownership, borrowing, and lifetimes.
 * Box<T> allows storing data on the heap while keeping ownership on the stack.
 */
