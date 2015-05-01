# A Library for Safe SIMD Operations [![Build Status](https://travis-ci.org/srijs/rust-simdop.svg?branch=master)](https://travis-ci.org/srijs/rust-simdop)

The `simdop` library provides SIMD functionality
for building applications using fast operations
on homogeneous vector types.
It provides safety on a type-level using traits.

## CPU identification and feature detection

In order to use a specific operations on a specific vector type,
you have to provide proof that the operation is valid on your architecture.
This is achieved by executing one or more detection functions, which return
an `Option<T>` type. You need to retrieve the wrapped value, which provides access
to the operations in the detected feature set in the form of trait implementations.

## Vector representation and encoding

Since the number of elements in a vector is always a power of two,
vectors are represented as a perfect binary tree, which encodes the length
of the vector as a logarithm in the height of the tree.
This makes structural modifications very easy, while providing strong type guarantees,
e.g. when splitting or joining vectors.

To perform SIMD operation, the tree is flattened into an array, and restored afterwards.
When combining vector operations, the compiler is able to perform "deforestation"
optimisations on the data structures, elminating most of the structural overhead. 
