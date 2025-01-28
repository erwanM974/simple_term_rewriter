# A basic interface to rewrite concrete tree terms according to a set of rules

I initially wrote this code for this project: [hibou](https://github.com/erwanM974/hibou_label).

Since then it has evolved a bit and I decided to repackage it on its own as a library.

This does not claim to be a Term Rewriting Library.

Only concrete terms are intended to be handled.

Rewriting rules are represented by a Generic Trait with a "try_apply" function that tries to apply it to a specific (sub)term, returning the result as an Option.

Rewriting modulo theories (AC or other) is not handled by default.



## A small example

In "str/tests/" a small example that makes use of the crate's interface is given.

Below is an output produced by a graphviz logger connected to the rewriting process.

In the example process below all paths are computed but we can choose to only compute one.

Of course, depending on the way with which you define the rules, the system is not necessarily guaranteed to be terminating/confluent/convergent.
To check that, you should use a dedicated tool (it is not handled here).


<img src="./README_images/rewrite.svg" alt="example">



