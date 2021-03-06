#[cfg(debug_assertions)] trace_macros!(true);

macro_rules! bf {
    // these rules handle the end of the instruction stream -- either the real end of the program,
    // or the end of a loop body
    
    // no loop stack and no tail stack => goodbye
    (@run ()
     [$cur_:tt $left_:tt $right_:tt]
     [] [] []
     $out_:tt $_in:tt
#ifdef PROFILE
     $prof_:tt
#endif
    ) => {
        bf!(@out
            $cur_ [] $left_ $right_
            $out_
#ifdef PROFILE
            $prof_
#endif
           )
    };

    // counter is zero, pop the loop stack to exit the loop
    (@run ()
     [[] $left_:tt $right_:tt]
     [] [$loops_head:tt $($loops_tail:tt)*] [[$head:tt $($tail:tt)*] $($tails_:tt)*]
     $out_:tt $in_:tt
#ifdef PROFILE
     $prof_:tt
#endif
    ) => {
        bf!(@run $head
            [[] $left_ $right_]
            [$($tail)*] [$($loops_tail)*] [$($tails_)*]
            $out_ $in_
#ifdef PROFILE
            $prof_
#endif
           )
    };

    // counter is nonzero, restart the loop
    (@run ()
     $state_:tt
     [] [[$head:tt $($tail:tt)*] $($loops:tt)*] $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     $prof_:tt
#endif
    ) => {
        bf!(@run $head
            $state_
            [$($tail)*] [[$head $($tail)*] $($loops)*] $tails_
            $out_ $in_
#ifdef PROFILE
            $prof_
#endif
           )
    };

    // the next few rules deal with multi-character tokens which are parsed together, but we want
    // the individual tokens, so just split them up and push the pieces back onto the program

    // >> is actually > >
    (@run >>
     $state_:tt
     [$($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     $prof_:tt
#endif
    ) => {
        bf!(@run >
            $state_
            [> $($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            $prof_
#endif
           )
    };
    // << is actually < <
    (@run <<
     $state_:tt
     [$($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     $prof_:tt
#endif
    ) => {
        bf!(@run <
            $state_
            [< $($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            $prof_
#endif
           )
    };
    // .. is actually . .
    (@run ..
     $state_:tt
     [$($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     $prof_:tt
#endif
    ) => {
        bf!(@run .
            $state_
            [. $($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            $prof_
#endif
           )
    };
    // ... is actually . . .
    (@run ...
     $state_:tt
     [$($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     $prof_:tt
#endif
    ) => {
        bf!(@run .
            $state_
            [. . $($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            $prof_
#endif
           )
    };
    // <- is actually < -
    (@run <-
     $state_:tt
     [$($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     $prof_:tt
#endif
    ) => {
        bf!(@run <
            $state_
            [- $($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            $prof_
#endif
           )
    };
    // -> is actually - >
    (@run ->
     $state_:tt
     [$($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     $prof_:tt
#endif
    ) => {
        bf!(@run -
            $state_
            [> $($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            $prof_
#endif
           )
    };

    // now the instructions themselves!
    
    // add one to the current cell (overflow is a no-op)
    (@run +
     [[() () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
       () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
       () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
       () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
       () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
       () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
       () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
       () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()]
      $left_:tt $right_:tt]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [[() () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
              () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
              () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
              () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
              () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
              () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
              () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()
              () () () () () () () () () () () () () () () () () () () () () () () () () () () () () () ()]
             $left_ $right_]
            [$($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* instructions=1 increments=1 overflows=1]
#endif
           )
    };

    (@run +
     [[$($cur:tt)*] $left_:tt $right_:tt]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [[$($cur)* ()] $left_ $right_]
            [$($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* instructions=1 increments=1]
#endif
           )
    };
    
    // subtract one from the current cell (underflow is a no-op)
    (@run -
     [[] $left_:tt $right_:tt]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [[] $left_ $right_]
            [$($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* instructions=1 decrements=1 underflows=1]
#endif
           )
    };

    (@run -
     [[$chead:tt $($ctail:tt)*] $left_:tt $right_:tt]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [[$($ctail)*] $left_ $right_]
            [$($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* instructions=1 decrements=1]
#endif
           )
    };
    
    // move the cell pointer to the right
    // the memory expands if necessary (new cell initialized to zero)
    (@run >
     [$cur:tt [$($lmore:tt)*] [$right:tt $($rmore:tt)*]]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [$right [$cur $($lmore)*] [$($rmore)*]]
            [$($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* instructions=1 rights=1]
#endif
           )
    };

    (@run >
     [$cur:tt [$($lmore:tt)*] []]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [[] [$cur $($lmore)*] []]
            [$($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* instructions=1 rights=1 right_grows=1]
#endif
           )
    };
    
    // move the cell pointer to the left
    // the memory expands if necessary (new cell initialized to zero)
    (@run <
     [$cur:tt [$left:tt $($lmore:tt)*] [$($rmore:tt)*]]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [$left [$($lmore)*] [$cur $($rmore)*]]
            [$($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* instructions=1 lefts=1]
#endif
           )
    };

    (@run <
     [$cur:tt [] [$($rmore:tt)*]]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [[] [] [$cur $($rmore)*]]
            [$($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* instructions=1 lefts=1 left_grows=1]
#endif
           )
    };
    
    // loops!
    

    // special case for [-]
    (@run [-]
     [[$($cur:tt)*] $left_:tt $right_:tt]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [[] $left_ $right_]
            [$($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* instructions=[$($cur),*].len() as u32 clears=1 decrements=[$($cur),*].len() as u32]
#endif
           )
    };
    
    // counter is zero, skip the loop
    (@run [$inner_head:tt $($inner_tail:tt)*]
     [[] $left_:tt $right_:tt]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [[] $left_ $right_]
            [$($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* instructions=1 loops=1]
#endif
           )
    };

    // counter is nonzero, push the current tail + loop and enter the loop
    (@run [$inner_head:tt $($inner_tail:tt)*]
     $state_:tt
     $tail_:tt [$($loops:tt)*] [$($tails:tt)*]
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $inner_head
            $state_
            [$($inner_tail)* ()] [[$inner_head $($inner_tail)* ()] $($loops)*] [$tail_ $($tails)*]
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* instructions=1 loops=1]
#endif
           )
    };
    
    // output the current cell: just push $cur onto $out
    //  note: the Rust "program" will crash at runtime if the BF program outputs invalid UTF-8
    (@run .
     [$cur:tt $left_:tt $right_:tt]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     [$($out:tt)*] $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [$cur $left_ $right_]
            [$($tail)*] $loops_ $tails_
            [$($out)* $cur] $in_
#ifdef PROFILE
            [$($prof_var)* instructions=1 outs=1]
#endif
           )
    };

    // input to the current cell: do nothing on EOF, otherwise pop $in and parse it
    (@run ,
     [$cur:tt $left_:tt $right_:tt]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt []
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            [$cur $left_ $right_]
            [$($tail)*] $loops_ $tails_
            $out_ []
#ifdef PROFILE
            [$($prof_var)* instructions=1 ins=1 eofs=1]
#endif
           )
    };
    (@run ,
     [$cur:tt $left_:tt $right_:tt]
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt [$in_head:tt $($in_tail:tt)*]
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@in @revconv
#ifdef PROFILE
            [$($prof_var)* instructions=1 ins=1]
#endif
            $in_head []
            [[$head $left_ $right_] [[$($tail)*] $loops_ $tails_ $out_ [$($in_tail)*]]]
           )
    };

    // input helper subroutine for converting from decimal to unary
    // step 1 "revconv": reverse the digits and convert them to zermelo
    // step 2 "unary": iterate through the digits and construct a unary number
    (@in @revconv
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [] [$($digit:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @unary
#ifdef PROFILE
            [$($prof_var)* in_revconvs=1]
#endif
            [] [] [()] [$($digit)*]
            $stuff
           )
    };
    (@in @revconv
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [0 $($tail:tt)*] [$($digit:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @revconv
#ifdef PROFILE
            [$($prof_var)* in_revconvs=1]
#endif
            [$($tail)*] [() $($digit)*]
            $stuff
           )
    };
    (@in @revconv
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [1 $($tail:tt)*] [$($digit:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @revconv
#ifdef PROFILE
            [$($prof_var)* in_revconvs=1]
#endif
            [$($tail)*] [(()) $($digit)*]
            $stuff
           )
    };
    (@in @revconv
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [2 $($tail:tt)*] [$($digit:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @revconv
#ifdef PROFILE
            [$($prof_var)* in_revconvs=1]
#endif
            [$($tail)*] [((())) $($digit)*]
            $stuff
           )
    };
    (@in @revconv
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [3 $($tail:tt)*] [$($digit:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @revconv
#ifdef PROFILE
            [$($prof_var)* in_revconvs=1]
#endif
            [$($tail)*] [(((()))) $($digit)*]
            $stuff
           )
    };
    (@in @revconv
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [4 $($tail:tt)*] [$($digit:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @revconv
#ifdef PROFILE
            [$($prof_var)* in_revconvs=1]
#endif
            [$($tail)*] [((((())))) $($digit)*]
            $stuff
           )
    };
    (@in @revconv
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [5 $($tail:tt)*] [$($digit:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @revconv
#ifdef PROFILE
            [$($prof_var)* in_revconvs=1]
#endif
            [$($tail)*] [(((((()))))) $($digit)*]
            $stuff
           )
    };
    (@in @revconv
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [6 $($tail:tt)*] [$($digit:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @revconv
#ifdef PROFILE
            [$($prof_var)* in_revconvs=1]
#endif
            [$($tail)*] [((((((())))))) $($digit)*]
            $stuff
           )
    };
    (@in @revconv
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [7 $($tail:tt)*] [$($digit:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @revconv
#ifdef PROFILE
            [$($prof_var)* in_revconvs=1]
#endif
            [$($tail)*] [(((((((()))))))) $($digit)*]
            $stuff
           )
    };
    (@in @revconv
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [8 $($tail:tt)*] [$($digit:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @revconv
#ifdef PROFILE
            [$($prof_var)* in_revconvs=1]
#endif
            [$($tail)*] [((((((((())))))))) $($digit)*]
            $stuff
           )
    };
    (@in @revconv
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [9 $($tail:tt)*] [$($digit:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @revconv
#ifdef PROFILE
            [$($prof_var)* in_revconvs=1]
#endif
            [$($tail)*] [(((((((((()))))))))) $($digit)*]
            $stuff
           )
    };

    // how step 2 works:
    // - $fullacc is the number being built up, $digitacc is used within digits (both are unary numbers)
    // - $pv is the place value counter, multiplied by 10 each iteration
    // - within a digit, count down the zermelo layers and inc $digitacc by $pv
    // - at the end of a digit, just add $digitacc to $fullacc (and multiply $pv by 10)
    (@in @unary
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     $fullacc:tt $digitacc:tt $pv:tt []
     [[$head:tt $left:tt $right:tt] [$($other_stuff:tt)*]]
    ) => {
        bf!(@run $head
            [$fullacc $left $right]
            $($other_stuff)*
#ifdef PROFILE
            [$($prof_var)* in_unaries=1]
#endif
           )
    };
    (@in @unary
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     [$($fullacc:tt)*] [$($digitacc:tt)*] [$($pv:tt)*] [() $($digits:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @unary
#ifdef PROFILE
            [$($prof_var)* in_unaries=1]
#endif
            [$($fullacc)* $($digitacc)*] [] [$($pv)* $($pv)* $($pv)* $($pv)* $($pv)*
                                             $($pv)* $($pv)* $($pv)* $($pv)* $($pv)*] [$($digits)*]
            $stuff
           )
    };
    (@in @unary
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
     $fullacc:tt [$($digitacc:tt)*] [$($pv:tt)*] [($digit:tt) $($digits:tt)*]
     $stuff:tt
    ) => {
        bf!(@in @unary
#ifdef PROFILE
            [$($prof_var)* in_unaries=1]
#endif
            $fullacc [$($digitacc)* $($pv)*] [$($pv)*] [$digit $($digits)*]
            $stuff
           )
    };

    // invalid instruction is a no-op
    (@run $_instr:tt
     $state_:tt
     [$head:tt $($tail:tt)*] $loops_:tt $tails_:tt
     $out_:tt $in_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@run $head
            $state_
            [$($tail)*] $loops_ $tails_
            $out_ $in_
#ifdef PROFILE
            [$($prof_var)* noops=1]
#endif
           )
    };

    // the program is over! we now output the memory state and the output to Rust
    // first we need to unzip the memory, which involves reversing the left-hand side

    // the reversal is finished: convert each memory cell and output character to arithmetic and
    // output them into arrays, also generate the profile shim
    (@out
     $cur:tt [$($left:tt)*] [] [$($right:tt)*]
     [$($out:tt)*]
#ifdef PROFILE
     [$($prof_var:ident = $prof_expr:expr)*]
#endif
    ) => {
        types::Machine {
            memory: types::CambridgeArray(&[$(bf!(@count $left),)* bf!(@count $cur), $(bf!(@count $right),)*]),
            output: types::UTF8Wrapper(&[$(bf!(@count $out),)*]),
#ifdef PROFILE
            trace:  {
                fn profile() -> types::Profile {
                    let mut p = types::Profile::default();
                    $(p.$prof_var += $prof_expr;)*
                    p
                }
                types::ProfileShim(profile)
            },
#endif
        }
    };
    // reverse the left-hand side of the memory zipper for output
    (@out
     $cur_:tt [$($left_:tt)*] [$left_head:tt $($left_tail:tt)*] $right_:tt
     $out_:tt
#ifdef PROFILE
     [$($prof_var:tt)*]
#endif
    ) => {
        bf!(@out
            $cur_ [$left_head $($left_)*] [$($left_tail)*] $right_
            $out_
#ifdef PROFILE
            [$($prof_var)* out_revs=1]
#endif
           )
    };
    
    // unconditionally replace a tt with a given expression
    (@replace $_from:tt $to:expr) => { $to };

    // convert a unary number to an arithmetic expression
    (@count [$($thing:tt)*]) => { (0u8 $(+ bf!(@replace $thing 1u8))*) };

    // entry point: given a sequence of instructions, launch the machine
    ({$head:tt $($tail:tt)*} {$($input:tt)*}) => { bf!(@run $head
    //                                                      ^ current instruction
                                                       [[] [] []]
    //                                                 ^ memory zipper [cur [left cells, adjacent first] [right cells, adjacent first]]
                                                       [$($tail)* ()] [] []
    //                                                  |         |   |  ^ tail stack
    //                                                  |         |   ^ loop stack
    //                                                  |         ^ sentinel added to mark the end of the program
    //                                                  ^ remainder of program
                                                       [] [$($input)*]
    //                                                 |  ^ unparsed input
    //                                                 ^ output
#ifdef PROFILE
                                                       []
    //                                                 ^ profiling data
#endif
                                                      )
    }
}

// TODO
// - change profiling to use individual counters, for speed
// - examine decisions for EOF and underflow/overflow wrapping
// - make things (profiling, EOF, wrapping) configurable using something like CPP
// - test all #ifdef configurations
// - use CPP to generate rules? cheating?

