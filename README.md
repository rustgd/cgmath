# OMath: a linear algebra library written in Rust

OMath (Odyssey Math) is linear algebra I've been working on to help teach myself 3D mathematics. Learning by doing, y'know. I've uploaded it in the hope that somebody might find it useful, you never know.

## Todo:

- Matrix Inversion: ugh
- Matrix rotation
- Euler and Axial rotations

## Disclaimer:

I'm new to Rust and a novice at linear algebra, so I may have made mistake, and I may not have done things in the most efficient way. So use this at your own risk!

## Why Rust?

I've been on the lookout for a nice system programming language for a while now because I'd like to work on real-time 3D without having to deal with the complexity and inelegance of C++.

Over the last few months I've spent some time playing with [D](http://dlang.org/). It has plenty of attractive qualities such as powerful compile-time generics, a friendly community on IRC and an aesthetically pleasing syntax. I do have concerns however that may cause me to shelve it for now. These include a lack of a defined mission, [performance issues with the GC](http://www.reddit.com/r/programming/comments/ze4cx/real_world_comparison_gc_vs_manual_memory/), and a lack of focus in the community in terms of creating a single solid compiler (there are currently three, DMD, GDC and LDC).

Over this time I've also been following [Rust](https://github.com/mozilla/rust/), a language currently under heavy development at Mozilla. It's built on LLVM, has a self-hosting compiler and has a clear mission: to be fast, concurrent and safe.

> It's like C++ grew up, went to grad school, started dating ML, and is sharing an office with Erlang.

There are downsides: the different pointer types are confusing at first, it still has those dreaded semicolons and it's missing some features I love (properties, default arguments, method overloading...). These are minor annoyances however, and the language as it stands feels really nice to work with. :)

~B☼