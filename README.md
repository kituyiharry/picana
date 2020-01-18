### Picana

![](https://github.com/kituyiharry/picana/workflows/Rust/badge.svg?branch=develop)

CANBus Communications tools and libraries for diagnosis and visualization

A WIP collection of libraries and materials to aid when working with CAN communications

# picana_native

Rust library usable via FFI

Provides an interface with functions that aid in setup and use of a can interface or dump!
Features include:

1. Loading a dump file and reading canframes!
2. Connecting to a local CAN interface

Communication is async to a dart client (described below) using Dart_PostCObject

# picana_dart

Communicates with the ffi interface using Ports

**NB**: Ensure Submodules are initialized before building and apply any relevant patches
