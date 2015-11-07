---
title: Hello world!
layout: post
---

Hello world!

This repository and blog is about a horrendous abuse of the Rust macro system in order to create a Brainfuck interpreter which runs at compile time.

Okay, back to the beginning. My name is Alex and I'm a robotics graduate student working on haptic sensing. I got interested in Rust during the run-up to the 1.0 stable release, and the timing of the release lined up with the beginning of a hardware/software project where I was the main software developer. I seized the opportunity to learn Rust while injecting it into my research environment, and that was successful.

Unsurprisingly Rust crept into my side projects as well. Coming from a bit of a Lisp (actually Clojure) background, I was excited to hear that Rust had a macro system, though I was a bit disappointed by its limitations. On the other hand, they say restrictions breed creativity.

This project is my largest and most recursive macro to date, by far. It tricks `rustc` into interpreting a Brainfuck program during the macro-expansion phase. No unstable features are used. The Brainfuck program is given in the usual syntax, and input can be provided using modified decimal literals. The interpreter produces is a few arrays of trivial arithmetic expressions (e.g. ```1u8 + 1u8```) representing the ending state of the machine's memory and the output. At Rust runtime (needs to be specified, since Brainfuck runtime is during Rust compile time), this data is printed (with the Brainfuck program's output converted to a UTF-8 string). There is also a rudimentary profiler which counts how many times the Brainfuck machine executes each instruction, etc. However, this requires a bit of Rust runtime execution for reasons that I will explain later.

If you just came here for the macro, by all means [proceed](https://github.com/durka/brainmunch/blob/master/src/bf.pre.rs). Otherwise, read on for some background and explanation.

Background
==========

First, influences. There are three main influences to cite:

1. The Rust book briefly delves into advanced, macros showing an implementation of [Bitwise Cyclic Tag](https://esolangs.org/wiki/Bitwise_Cyclic_Tag) in a section called [The deep end](https://doc.rust-lang.org/book/macros.html#the-deep-end). We will see that this deep end was, in fact, rather shallow.
2. Many thanks are due to the inimitable [Quxxy](https://github.com/DanielKeep) and their [TLBORM](http://danielkeep.github.io/tlborm/), both for the tricks that form the building blocks of the macros shown here, and for general encouragement on the path to completely macro insanity.
3. Quxxy again, plus Manishearth, for the [implementation](https://www.reddit.com/r/rust/comments/39wvrm/hodor_esolang_as_a_rust_macro/cs769ip) of a Hodor-to-Rust compiler in a macro. Hodor is the same as [Ook!](http://esolangs.org/wiki/Ook), which is the same as Brainfuck but with a syntax that is harder for a Rust macro to parse (due to the need for manual bracket counting). However, note that this was a compiler, not an interpeter -- the Brainfuck program is translated at Rust compile time to Rust code, which then runs at Rust runtime.

Progression
===========

These will be example-heavy mini-articles, starting small and building up to the final Brainmunch macro.

1. Recursive macros and token-tree munchers
    - Macros can call themselves
    - Digression: continuation passing style
    - Recursion limit
    - TT Munching
    - TT Bundling
2. Numbers: representations and parsing
    - Digression: when is the macro-expansion phase?
    - Zermelo numerals
    - Unary numerals
    - Output
    - Input: "Parsing" "literals"
3. Brainmunch: first steps
    - Digression: Brainfuck syntax and semantics
    - Machine representation
    - inc & dec instructions
    - left & right instructions
4. Brainmunch: loops
    - Marking the end of the instruction stream
    - Leveraging the Rust parser
    - The loop stack and the tail stack
    - Putting loops together
5. Brainmunch: input and output
    - Output, which is easy
    - Input, which is hard
6. Brainmunch: profiling
    - Goals and non-goals of profiling
    - Necessity of cheating
    - Implementation of the profiler
7. Demo time
    - Digression: stack size
    - Ex1. Hello world
    - Ex2. Rectangles
    - Ex3. Incat
    - Ex4. Awib???

