---
title: Hello world!
layout: post
---

Hello world!

This repository and blog is about a horrendous abuse of the Rust macro system in order to create a Brainfuck interpreter which runs at compile time.

Okay, back to the beginning. My name is Alex and I'm a robotics graduate student working on haptic sensing. I got interested in Rust during the run-up to the 1.0 stable release, and the timing of the release lined up with the beginning of a hardware/software project where I was the main software developer. I seized the opportunity to learn Rust while injecting it into my research environment, and that was successful.

Unsurprisingly Rust crept into my side projects as well. Coming from a bit of a Lisp (actually Clojure) background, I was excited to hear that Rust had a macro system, though I was a bit disappointed by its limitations. On the other hand, they say restrictions breed creativity.

This project is my largest and most recursive macro to date, by far. It tricks ```rustc``` into interpreting a Brainfuck program during the macro-expansion phase. The Brainfuck program is given in the usual syntax, and input can be provided using modified decimal literals. The interpreter's output is a few arrays of trivial arithmetic expressions (e.g. {% ihighlight rust %}1u8 + 1u8{% endihighlight %}) which are printed at Rust runtime.

{% highlight rust %}
fn foo<T, U>(x: i32) -> u32 {}
{% endhighlight %}

