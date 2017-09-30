Rust macro that implements a Brainfuck interpreter (not a translator) at macro-expansion time.

This was presented in [a talk at RustFest Zurich 2017](http://zurich.rustfest.eu/sessions/alex)!

Example usage (full demo in `demo.sh` + `src/demo.rs`):

```rust
const MACHINE: types::Machine<'static> =
    bf!({
        ++++++++++
        [>+++++++>++++++++++>+++>+<<<<-]
        >++.>+.+++++++..+++.>++
        [.[-],]
        >[>+++<-]>+++.
    } {
        [8 2] [1 1 7] [1 1 5] [1 1 6] [7 0] [1 0 1] [1 1 5] [1 1 6]
        [3 2]
        [9 0] [1 9 5] [1 8 8] [1 1 4] [1 0 5] [9 9] [1 0 4]
    });

println!("{:#?}", MACHINE);
```

Output:

```rust
Machine {
    memory: [ 0 72 111 0 0 33 ],
    output:
    Hello RustFest Zürich!
}
```

