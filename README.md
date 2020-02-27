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

Communication is async to a dart client

# picana_dart

Communicates with the ffi interface using Ports

**NB**: Ensure Submodules are initialized before building and apply any relevant patches i.e

```bash
git submodule update --init
cd ./nix && git apply ../nix_patch.diff 
cd ./socketcan-rs && git apply ../socketcan-libc.diff
```


## Building

To build the project you would require (Tested on Linux - x64):
1. A valid [Dart SDK(> version 2.7.0)](https://github.com/dart-lang/sdk)
2. Cargo - to compile rust sources
3. [Can-utils](https://github.com/linux-can/can-utils.git) package and access to VCAN interface for testing

* To build the shared library

```bash
cd ./picana_native
BINDGEN_DART_SDK_PATH=... cargo build --release
cd ..
```

* To test the shared library - copy it out to the picana dart directory

1. Get latest dependencies
```bash
cp ./picana_native/target/release/libpicana.so ./picana_dart/
cd ./picana_dart
pub upgrade
```
2. Start the CAN interface

```bash
sudo modprobe vcan
sudo ip link add vcan0 type vcan
sudo ip link add vcan1 type vcan
sudo ip link set vcan0 up
sudo ip link set vcan1 up
```


3. Run the main example to test 
```bash
pub run bin/main.dart
```

Open a new shell and you can send can messages via cansend to the virtual can interface and a handler should handle the messages e.g

```bash
cansend vcan0 6b1#ffff0000
cansend vcan1 6b1#ffff0000
```

You should see the messages been printed out to original console running the program!

##TODO: 

 -> Use as a library in other packages!!
