#![recursion_limit="5000"]
#![cfg_attr(debug_assertions, feature(trace_macros))]

#[macro_use] mod bf;
mod types;

// the BF program is interpreted by the macro system at compile time and turned into a constant arithmetic expression (see expanded.rs)
// example run: unifdef -t -UPROFILE bf.rs.pre >bf.rs; [ $? != 2 ] && time RUST_MIN_STACK=16777216 mr ru nightly cargo run --bin bf
const MACHINE: types::Machine<'static> =
    /*bf!({ // incrementing cat
        first some useless instructions to exercise the profiler
            this one triggers an overflow: +++++[>+++<-]>[>+++++ +++++ +++++ ++<-]>+
            this one triggers an underflow: >-
            now grow the memory in both directions: <<<<<>>>>>
        here is the real program
            ,[+.[-],]
        one last useless eof: ,
    } {
        [6 4]   [6 5]   [6 6]   [6 7]
        [6 8]   [6 9]   [7 0]   [7 1]
        [7 2]   [7 3]   [7 4]   [7 5]
        [7 6]   [7 7]   [7 8]   [7 9]
        [8 0]   [8 1]   [8 2]   [8 3]
        [8 4]   [8 5]   [8 6]   [8 7]
        [8 8]   [8 9]   [9 6]   [9 7]
        [9 8]   [9 9]   [1 0 0] [1 0 1]
        [1 0 2] [1 0 3] [1 0 4] [1 0 5]
        [1 0 6] [1 0 7] [1 0 8] [1 0 9]
        [1 1 0] [1 1 1] [1 1 2] [1 1 3]
        [1 1 4] [1 1 5] [1 1 6] [1 1 7]
        [1 1 8] [1 1 9] [1 2 0] [1 2 1]
        [4 7]   [4 8]   [4 9]   [5 0]
        [5 1]   [5 2]   [5 4]   [5 5]
        [5 6]
    });*/
    //bf!({ ++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>. } {}); // hello world
    bf!({ // hello $input!
        ++++++++++                        $0 = 10
        [>+++++++>++++++++++>+++>+<<<<-]  ($1, $2, $3, $4) = (70, 100, 30, 10)
        >++.>+.+++++++..+++.>++           output "Hello "
        [.[-],]                           cat input
        >[>+++<-]>+++.                    output "!"
    } {
        [8 2] [1 1 7] [1 1 5] [1 1 6] [7 0] [1 0 1] [1 1 5] [1 1 6] // R u s t F e s t
        [3 2] // space
        [9 0] [1 9 5] [1 8 8] [1 1 4] [1 0 5] [9 9] [1 0 4] // Z ü r i c h
    });
    //bf!({ ++[>++[>++<-].<-] } { }); // nested loop test
    //bf!({ + } { });
    //bf!({ ,------------------------------------------------>,------------------------------------------------<[>[- >>>>>>>>>> >+>+<< <<<<<<<<<<]>>>>>>>>>> >>[-<< <<<<<<<<<<+>>>>>>>>>> >>]<[<<<<<<<<<< <> ++++++ [ > +++++++ < - ] > . [-] <<>>>>>>>>>> >-]<<<<<<<<<< <> +++++ [ > ++ < - ] > . [-] <<<-] } { [5 0] [5 0] }); // draw a rectangle (credit: Aceeri on #rust)
    //bf!({ ,>,<[>[- >>>>>>>>>> >+>+<< <<<<<<<<<<]>>>>>>>>>> >>[-<< <<<<<<<<<<+>>>>>>>>>> >>]<[<<<<<<<<<< <> ++++++ [ > +++++++ < - ] > . [-] <<>>>>>>>>>> >-]<<<<<<<<<< <> +++++ [ > ++ < - ] > . [-] <<<-] } { [3] [2] }); // draw a rectangle (modified to take integers instead of ASCII)
    //bf!({ +++>++  <[>[- >>>>>>>>>> >+>+<< <<<<<<<<<<]>>>>>>>>>> >>[-<< <<<<<<<<<<+>>>>>>>>>> >>]<[<<<<<<<<<< <> ++++++ [ > +++++++ < - ] > . [-] <<>>>>>>>>>> >-]<<<<<<<<<< <> +++++ [ > ++ < - ] > . [-] <<<-] } { }); // draw a rectangle (modified to set the first two tape entries instead of taking input)

fn main() {
    // all we do at runtime is print the memory state and the output (converted to UTF-8)
    println!("{:#?}", MACHINE);
}

