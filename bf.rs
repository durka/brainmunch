#![recursion_limit="5000"]
//#![feature(trace_macros)] trace_macros!(true);

macro_rules! bf {
    // these rules handle the end of the instruction stream -- either the real end of the program,
    // or the end of a loop body
    
    // no loop stack and no tail stack => goodbye
    (@run () [$cur_:tt $left_:tt $right_:tt] [] [] [] $out_:tt $_in:tt) => {
        bf!(@out $cur_ [] $left_ $right_ $out_)
    };
    // counter is zero, pop the loop stack to exit the loop
    (@run () [() $left_:tt $right_:tt] [] [$loops_head:tt $($loops_tail:tt)*] [[$head:tt $($tail:tt)*] $($tails_:tt)*] $out_:tt $in_:tt) => {
        bf!(@run $head [() $left_ $right_] [$($tail)*] [$($loops_tail)*] [$($tails_)*] $out_ $in_)
    };
    // counter is nonzero, restart the loop
    (@run () $state_:tt [] [[$head:tt $($tail:tt)*] $($loops:tt)*] $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run $head $state_ [$($tail)*] [[$head $($tail)*] $($loops)*] $tails_ $out_ $in_)
    };

    // the next few rules deal with multi-character tokens which are parsed together, but we want
    // the individual tokens, so just split them up and push the pieces back onto the program

    // >> is actually > >
    (@run >> $state_:tt [$($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run > $state_ [> $($tail)*] $loops_ $tails_ $out_ $in_)
    };
    // << is actually < <
    (@run << $state_:tt [$($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run < $state_ [< $($tail)*] $loops_ $tails_ $out_ $in_)
    };
    // .. is actually . .
    (@run .. $state_:tt [$($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run . $state_ [. $($tail)*] $loops_ $tails_ $out_ $in_)
    };
    // ... is actually . . .
    (@run ... $state_:tt [$($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run . $state_ [. . $($tail)*] $loops_ $tails_ $out_ $in_)
    };
    // <- is actually < -
    (@run <- $state_:tt [$($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run < $state_ [- $($tail)*] $loops_ $tails_ $out_ $in_)
    };
    // -> is actually - >
    (@run -> $state_:tt [$($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $out_:tt) => {
        bf!(@run - $state_ [> $($tail)*] $loops_ $tails_ $out_ $in_)
    };

    // now the instructions themselves!
    
    // add one to the current cell: $cur => ($cur)
    (@run + [$cur:tt $left_:tt $right_:tt] [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run $head [($cur) $left_ $right_] [$($tail)*] $loops_ $tails_ $out_ $in_)
    };
    
    // subtract one from the current cell: ($cur) => (), () => () (that is, underflow is a no-op)
    (@run - [() $left_:tt $right_:tt] [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {{
        bf!(@run $head [() $left_ $right_] [$($tail)*] $loops_ $tails_ $out_ $in_)
    }};
    (@run - [($cur:tt) $left_:tt $right_:tt] [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run $head [$cur $left_ $right_] [$($tail)*] $loops_ $tails_ $out_ $in_)
    };
    
    // move the cell pointer to the right
    // the memory expands if necessary (new cell initialized to zero)
    (@run > [$cur:tt [$($lmore:tt)*] [$right:tt $($rmore:tt)*]] [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run $head [$right [$cur $($lmore)*] [$($rmore)*]] [$($tail)*] $loops_ $tails_ $out_ $in_)
    };
    (@run > [$cur:tt [$($lmore:tt)*] []] [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run $head [() [$cur $($lmore)*] []] [$($tail)*] $loops_ $tails_ $out_ $in_)
    };
    
    // move the cell pointer to the left
    // the memory expands if necessary (new cell initialized to zero)
    (@run < [$cur:tt [$left:tt $($lmore:tt)*] [$($rmore:tt)*]] [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run $head [$left [$($lmore)*] [$cur $($rmore)*]] [$($tail)*] $loops_ $tails_ $out_ $in_)
    };
    (@run < [$cur:tt [] [$($rmore:tt)*]] [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run $head [() [] [$cur $($rmore)*]] [$($tail)*] $loops_ $tails_ $out_ $in_)
    };
    
    // loops!
    
    // counter is zero, skip the loop
    (@run [$inner_head:tt $($inner_tail:tt)*] [() $left_:tt $right_:tt] [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run $head [() $left_ $right_] [$($tail)*] $loops_ $tails_ $out_ $in_)
    };
    // counter is nonzero, push the current tail + loop and enter the loop
    (@run [$inner_head:tt $($inner_tail:tt)*] $state_:tt $tail_:tt [$($loops:tt)*] [$($tails:tt)*] $out_:tt $in_:tt) => {
        bf!(@run $inner_head $state_ [$($inner_tail)* ()] [[$inner_head $($inner_tail)* ()] $($loops)*] [$tail_ $($tails)*] $out_ $in_)
    };
    
    // output the current cell: just push $cur onto $out
    //  note: the Rust "program" will crash at runtime if the BF program outputs invalid UTF-8
    (@run . [$cur:tt $left_:tt $right_:tt] [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt [$($out:tt)*] $in_:tt) => {
        bf!(@run $head [$cur $left_ $right_] [$($tail)*] $loops_ $tails_ [$($out)* $cur] $in_)
    };

    // input to the current cell: do nothing on EOF, otherwise pop $in and parse it
    (@run , [$cur:tt $left_:tt $right_:tt] [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt $out_:tt []) => {
        bf!(@run $head [$cur $left_ $right_] [$($tail)*] $loops_ $tails_ $out_ [])
    };
    (@run , [$cur:tt $left_:tt $right_:tt] [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt $out_:tt [$in_head:tt $($in_tail:tt)*]) => {
        bf!(@in @revconv $in_head [] [[$head $left_ $right_] [[$($tail)*] $loops_ $tails_ $out_ [$($in_tail)*]]])
    };

    (@in @revconv [] [$($digit:tt)*] $stuff:tt) => { bf!(@in @unary [] [] [()] [$($digit)*] $stuff) };
    (@in @revconv [0 $($tail:tt)*] [$($digit:tt)*] $stuff:tt) => { bf!(@in @revconv [$($tail)*] [() $($digit)*] $stuff) };
    (@in @revconv [1 $($tail:tt)*] [$($digit:tt)*] $stuff:tt) => { bf!(@in @revconv [$($tail)*] [(()) $($digit)*] $stuff) };
    (@in @revconv [2 $($tail:tt)*] [$($digit:tt)*] $stuff:tt) => { bf!(@in @revconv [$($tail)*] [((())) $($digit)*] $stuff) };
    (@in @revconv [3 $($tail:tt)*] [$($digit:tt)*] $stuff:tt) => { bf!(@in @revconv [$($tail)*] [(((()))) $($digit)*] $stuff) };
    (@in @revconv [4 $($tail:tt)*] [$($digit:tt)*] $stuff:tt) => { bf!(@in @revconv [$($tail)*] [((((())))) $($digit)*] $stuff) };
    (@in @revconv [5 $($tail:tt)*] [$($digit:tt)*] $stuff:tt) => { bf!(@in @revconv [$($tail)*] [(((((()))))) $($digit)*] $stuff) };
    (@in @revconv [6 $($tail:tt)*] [$($digit:tt)*] $stuff:tt) => { bf!(@in @revconv [$($tail)*] [((((((())))))) $($digit)*] $stuff) };
    (@in @revconv [7 $($tail:tt)*] [$($digit:tt)*] $stuff:tt) => { bf!(@in @revconv [$($tail)*] [(((((((()))))))) $($digit)*] $stuff) };
    (@in @revconv [8 $($tail:tt)*] [$($digit:tt)*] $stuff:tt) => { bf!(@in @revconv [$($tail)*] [((((((((())))))))) $($digit)*] $stuff) };
    (@in @revconv [9 $($tail:tt)*] [$($digit:tt)*] $stuff:tt) => { bf!(@in @revconv [$($tail)*] [(((((((((()))))))))) $($digit)*] $stuff) };
    (@in @unary $fullacc:tt $digitacc:tt $pv:tt [] $stuff:tt) => { bf!(@in @zem () $fullacc $stuff) };
    (@in @unary        [$($fullacc:tt)*]            [$($digitacc:tt)*] [$($pv:tt)*]                              [() $($digits:tt)*] $stuff:tt) => {
        bf!(@in @unary [$($fullacc)* $($digitacc)*] []                 [$($pv)* $($pv)* $($pv)* $($pv)* $($pv)*
                                                                        $($pv)* $($pv)* $($pv)* $($pv)* $($pv)*] [$($digits)*]       $stuff)
    };
    (@in @unary        $fullacc:tt [$($digitacc:tt)*]      [$($pv:tt)*] [($digit:tt) $($digits:tt)*] $stuff:tt) => {
        bf!(@in @unary $fullacc    [$($digitacc)* $($pv)*] [$($pv)*]    [$digit      $($digits)*]    $stuff)
    };
    (@in @zem $acc:tt []                      [[$head:tt $left:tt $right:tt] [$($other_stuff:tt)*]]) => { bf!(@run $head [$acc $left $right] $($other_stuff)*) };
    (@in @zem $acc:tt [$head:tt $($tail:tt)*] $stuff:tt)                                             => { bf!(@in @zem ($acc) [$($tail)*] $stuff) };

    // invalid instruction is a no-op
    (@run $_instr:tt $state_:tt [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt $out_:tt $in_:tt) => {
        bf!(@run $head $state_ [$($tail)*] $loops_ $tails_ $out_ $in_)
    };

    // the program is over! we now output the memory state and the output to Rust
    // first we need to unzip the memory, which involves reversing the left-hand side

    // the reversal is finished: convert each memory cell and output character to arithmetic and
    // output them into arrays
    (@out $cur:tt [$($left:tt)*] [] [$($right:tt)*] [$($out:tt)*]) => {
        (&[$(bf!(@count $left),)* bf!(@count $cur), $(bf!(@count $right),)*], &[$(bf!(@count $out),)*])
    };
    // reverse the left-hand side of the memory zipper for output
    (@out $cur_:tt [$($left_:tt)*] [$left_head:tt $($left_tail:tt)*] $right_:tt $out_:tt) => {
        bf!(@out $cur_ [$left_head $($left_)*] [$($left_tail)*] $right_ $out_)
    };
    
    // recursively convert a Zermelo numeral to an arithmetic expression
    //  note: only the first and last line should be necessary, but the compiler has a stack
    //  overflow on a too-long chain of additions, so we do a little unrolling
    (@count ()) => { 0u8 };
    (@count ((($inner:tt)))) => { 3u8 + bf!(@count $inner) };
    (@count (($inner:tt))) => { 2u8 + bf!(@count $inner) };
    (@count ($inner:tt)) => { 1u8 + bf!(@count $inner) };

    // entry point: given a sequence of instructions, launch the machine
    ({$head:tt $($tail:tt)*} {$($input:tt)*}) => { bf!(@run $head [() [] []] [$($tail)* ()] [] [] [] [$($input)*]) }
    //                                                      |     |           |         |   |  |  |  ^ input
    //                                                      |     |           |         |   |  |  ^ output
    //                                                      |     |           |         |   |  ^ tail stack
    //                                                      |     |           |         |   ^ loop stack
    //                                                      |     |           |         ^ sentinel added to mark the end of the program
    //                                                      |     |           ^ remainder of program
    //                                                      |     ^ memory zipper [cur [left cells, adjacent first] [right cells, adjacent first]]
    //                                                      ^ current instruction
}

// the BF program is interpreted by the macro system at compile time and turned into a constant
// arithmetic expression (see expanded.rs)
// MACHINE is a tuple where MACHINE.0 is the memory state and MACHINE.1 is the output
const MACHINE: (&'static [u8], &'static [u8]) =
    bf!({ ,[+.[-],] } { [6 4] [6 5] [6 6] [6 7] [6 8] [6 9] });
    //bf!({ ++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>. } {});
    //bf!({ + } { });

fn main() {
    // all we do at runtime is print the memory state and the output (converted to UTF-8)
    println!("memory={:?}", MACHINE.0);
    println!("output:");
    println!("{}", std::str::from_utf8(MACHINE.1).unwrap());
}
