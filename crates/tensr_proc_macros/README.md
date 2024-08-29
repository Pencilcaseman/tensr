# Tensr Procedural Macros

This crate contains procedural macros for the Tensr multidimensional array library.

Unfortunately, due to Rust's orphan rules, it is not possible to implement lazy
evaluation concisely with traits. As a result, a fair amount of code duplication is
required to get the required functionality.

Luckily, the duplicated code is pretty simple and can be auto-generated quite easily.
Rust also seems to compile the code very quickly, so this is not a problem.
