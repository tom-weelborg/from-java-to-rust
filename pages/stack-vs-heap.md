# Stack vs. Heap

This is a very brief overview of the two memory regions Stack and Heap.

::left::

## Stack

<v-clicks at="1" depth="2">

- Memory is allocated and freed in a stack-like order.
- typically stores fixed-size local data
- fast allocation and deallocation
- usage:
  - [Java]{style="color:#ED8B00"}: local primitives and references
  - [Rust]{style="color:#D34516"}: all local variables by default

</v-clicks>

::right::

## Heap

<v-clicks at="1" depth="2">

- Memory is allocated and freed more flexibly.
- supports dynamically sized data
- Allocation is generally more expensive.
- usage:
  - [Java]{style="color:#ED8B00"}: objects and arrays
  - [Rust]{style="color:#D34516"}: contents of `Box`, `Vec` and `String`

</v-clicks>
