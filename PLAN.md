---
title: "RES: a NES emulator written in Rust"
subtitle: "Design document"
author: Karl H. Totland
---

Contents
--------

1. [Purpose](#purpose)
2. [Design](#design)
    1. [Components](#components)
    2. [Memory layout](#memory)
    3. [CPU](#cpu)
    4. [PPU](#ppu)
    5. [APU](#apu)
3. [Rust](#rust)

Purpose (#purpose)
-------

I started this project with two objectives in mind: to learn Rust, and to learn
about emulation. Since I am not very familiar with Rust, I expect it to take _a
lot_ of time, but I have really been drawn to the programming language for
a while due to its roots in functional programming as well as its focus on
concurrency.

The latter part currently consists of my understanding of concepts in Rust to
try to make myself a better programmer. Ha.

Design (#design)
------

The project will contain several phases.

The phases are:

1. Figure out which components are necessary
2. Figure out the memory layout
3. Create CPU and its instructions
4. Figure out PPU and how to translate to ggez
5. Figure out audio and how that is processed.


Components (#components)
========================


Memory Layout (#memory)
=======================


Central Processing Unit (#cpu)
==============================

Instructions have six indexed addressing forms:

+--------+-------------------+-----------------------------------------------------------------------+--------+
| Abbr   | Name              | Formula                                                               | Cycles |
+--------+-------------------+-----------------------------------------------------------------------+--------+
| d,x    | Zero page indexed | `val = PEEK((arg + X) % 256)`                                         | 4      |
| d,y    | Zero page indexed | `val = PEEK((arg + Y) % 256)`                                         | 4      |
| a,x    | Absolute indexed  | `val = PEEK(arg + X)`                                                 | 4+     |
| a,y    | Absolute indexed  | `val = PEEK(arg + Y)`                                                 | 4+     |
| (d,x)  | Indexed indirect  | `val = PEEK(PEEK((arg + X) % 256) + PEEK((arg + X + 1) % 256) * 256)` | 6      |
| (d),y  | Indirect indexed  | `val = PEEK(PEEK(arg) + PEEK((arg + 1) % 256) * 256 + Y)`             | 5+     |
+--------+-------------------+-----------------------------------------------------------------------+--------+

`PEEK` means to retrieve the contents of memory.

Notice the "Zero page" indexing. :


Rust (#rust)
----

### Data types

**Attributes:**  

mutable
:   Mutable data, can be changed after being assigned -- similar to not using
    `const` in C.

immutable
:   This is the default variable state, similar to `const` in C.

shadowing
:   It is possible to reuse an immutable variable's name inside the same scope.
    The implication of this is the possibility to transform immutable data
    a few times.


**Constants:**  
Global predefined constants can be found in the standard library, each in
individual modules. For example, you can use `std::u64::MAX` to get the maximum
value for an unsigned 64-bit integer.
[This](https://doc.rust-lang.org/std/index.html) is a useful link regarding
both constants and other functions in the standard library.

**Integers:**  
Integers can be represented as decimal, hex, octal, binary and bytes (which
requires ASCII characters and is only supported on `u8` types). `_` can be used
as a separator and is ignored when the compiler parses the number. You can also
explicitly add a type suffix, for instance in `57u8` to declare it as an
unsigned 8-bit integer.

Here's a table:

+------------------+---------------+
| Number literals  |    Example    |
+------------------+---------------+
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `ob1111_0000` |
| Byte (`u8` only) | `b'A'`        |
+------------------+---------------+

It is worth noting that Rust will check for over- and underflows when running
in debug mode, which causes a `panic!`. These checks are omitted when compiling
in release mode (with `--release`), and the integers will wrap. If you
explicitly want to add the possibility for wrapping, the standard library type
`Wrapping` can be used.

**Characters:**  
In Rust, the `char` datatype represents Unicode Scalar Values. This means that
characters are 4 bytes, and can represent vast amounts of symbols.

**Tuples:**  
Similar to Haskell, 

**Functions:**  
To explicitly describe what a function shall return, you can add `-> i32` to
the end of the declaration, similar to how it is done in Haskell.

    fn double(x: i32) -> i32 {
        x * 2
    }

**Ownership:**  
Rust takes care of heap allocation for you, and freeing as well. When you exit
the scope with a heap allocated variable, Rust will automatically free it for
you by using `drop` at the end of the scope.

Heap allocated (and mutable) String variables can be created by using the
`from` function from the `String` module, like so: `let mut
s = String::from("hello");`. To manipulate this string, you can for instance do
`s.push_str(", world!");` to append to it. This will also grow the allocated
memory area.

When a `String` is created on the heap, a structure with metadata is also added
onto the stack, consisting of a pointer, the length and the capacity of the
string. If you were to say that `let s2 = s;` (from the earlier example), this
structure will be copied, and it will therefore also point to the same `String`
in heap memory.

However, because you would then have created two structures of metadata
pointing to the same heap allocated text, the first reference (`s`) will be
invalidated (to prevent double `free` errors). This also means that `s` cannot
be used after assigning `s2` to `s`. Because of this mechanism, we are not
doing a _shallow copy_ of the data, but rather a _move_. Therefore, we can also
say that we _move_ `s` to `s2`.

Rust will also never automatically create _deep copies_ of your data, and
therefore any shallow copying (or _moving_) will be relatively inexpensive in
terms of performance.

We can create deep copies if we want, however. The procedure for doing this, is
a common method called `clone`. As it implies, it clones a data structure. For
strings, you will be able to do this (and end up with separate heap-allocated
strings):

    let s1 = String::from("Test, my dude");
    let s2 = s1.clone();

    println!("{} = {}\n", s1, s2);

This operation is also more expensive, as new heap memory has to be allocated,
and the contents of the previous string needs to be copied.

Rust also has a special annotation called `Copy`, which is used for types like
integer. This makes it so that older variables are still usable after
assignment. The `Drop` annotation (signifying the _move_) cannot be used in
conjunction with `Copy`.
[Here](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) is an
interesting link on the subject. Simple scalar values (probably) always has the
`Copy` trait (including _tuples_, if it only contains other scalar values).

When passing a variable to a function, the same rules apply. In other words,
a variable with the `Drop` trait will be invalidated passing to a function,
while one with the `Copy` trait will not. Heap allocated variables that are
returned by a given function, will be retained until the assigned variable
exits its scope.

**References and Borrowing**  

[mode]: # (vim: set spl=en:)
