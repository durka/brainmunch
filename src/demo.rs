#![recursion_limit="5000"]

#[macro_use] mod bf;
mod types;

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

fn main() {
    println!("{:#?}", MACHINE);
}

