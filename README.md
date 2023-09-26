
<div align="center">

# Autumn School of Solana 2023
</div>

## Task 2
This is Task 2 of the course. Its primary objective is to introduce you to the Rust programming language. We'll explore fundamental Rust principles, including borrowing, objects, traits, mutability, and more. This part of the course is crucial because Rust is a key programming language in the Solana ecosystem. It's worth noting that Rust is sometimes compared to C++ and considered a competitor, making it valuable to learn.

The central concept of this task revolves around creating a calculator and implementing multiple shapes with corresponding functions to calculate properties like area and circumference. However, since this task serves as a gateway into the Solana world, it's essential that your calculator also handles potential overflow and underflow issues commonly encountered in arithmetic operations within Solana programs.

Regarding the Shapes, the goal is to familiarize yourself with Rust's concepts of structs, traits, and effectively applying object-oriented programming (OOP) principles in this language.

-----

### Task details
Inside **main.rs**, let's walk through the code structure from top to bottom:

- Traits **Shape** and **Display**: You will find two traits defined, **Shape** and **Display**.
    - The **Display** trait is already implemented, and you are not supposed to modify it. It handles how the objects are displayed when printed.

    - The **Shape** trait defines several functions, each corresponding to specific properties of geometric shapes (**Rectangle**, **Circle**) that are further defined in the source code. The function names are mostly self-explanatory by their names. Your task is to implement these trait functions for the given geometric objects .

- **Structures** as **Geometric Objects**: Further you'll find structures representing shapes, such as **Rectangle** and **Circle**, with self-explanatory content (e.g., **Rectangle** has sides a and b). **<u>Implement the Shape trait appropriately for these objects, as properties like area differ between shapes.</u>**

- **Calculator** Struct: The Calculator is a struct that contains two operands, **x** and **y**. These operands are set when you create an instance of the **Calculator**, and mathematical functions are computed on them. **<u>Your task is to implement mathematical functions based on the provided template.</u>** Additionally, it's crucial to handle **underflow** and **overflow** appropriately in your calculator, as these cases are tested in the provided tests.


## Submission Process
- Implement the necessary functions and logic in **main.rs**; they are marked as **TODO!**
- Use the provided tests in **tests.rs** to validate your code.
- Push your work to the GitHub - Same as for Tasks before.

## Deadline
The deadline for this task is **Tuesday, September 3rd, at 23:59 UTC**. Note that we will not accept submissions after the deadline.

## Evaluation
We will evaluate your submission using the same test suite provided in this task. Therefore, the requirements for this task are to pass **100%** of the provided tests.

## Setup
For this Task you need to have [Rust Installed](https://www.rust-lang.org/tools/install). You don't need to worry about the installed version, as the specified Rust version inside rust-toolchain will handle that.

### Commands
After cloning the repository, you should be able to execute the following commands:

1. To compile the project, run:
```
cargo build
```

2. To run the project and start the main function, use:
```
cargo run
```

3. To test your implementation, run:
```
cargo test
```

If you encounter any questions or issues during the installation process or have any inquiries related to the task, please feel free to initiate a discussion on Discord within the Issues Forum.

## Hints and Useful Links
[Primitive Type i64](https://doc.rust-lang.org/std/primitive.i64.html)

[Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

[References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html?highlight=borrow#references-and-borrowing)

[Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

[Options](https://doc.rust-lang.org/std/option/)

-----

## What's next?
If you're interested in our company, don't hesitate to visit our website, [Ackee Blockchain](https://ackeeblockchain.com), or reach out to us on [Discord](https://discord.gg/x7qXXnGCsa). You can also follow us on [X(Twitter)](https://twitter.com/ackeeblockchain?lang=en) for updates.\
For the most recent [Solana News](https://solana.com/news) or [Solana Twitter](https://twitter.com/solana).
