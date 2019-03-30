# Xi-Lib RFC

This RFC's outlines the design of a Xi library.
## Goal
The goal of xi-lib is to enable clients to embed Xi directly into their program as a shared or static library. It aims
to offer a native rust interface as well as a C FFI.

## Design
Xi-lib is split into a low-level API and a high-level API. The low-level API communicates with the Xi RPC protocol by
sending and receiving JSON messages to and from Xi. A simple API is provided to allow for sending and receiving of
messages across multiple threads. A feature-rich higher-level API is provided, that allows for embedders to directly
interact with Xi's datastructures.

### Low Level Interface
The low level interface provides a simple messaging passing system to and from core. An embedding application must
start core on a separate thread. It then can send messages to core. If the application choses to receive callbacks, it
must provide a callback function and start a receiver thread on a separate thread from core (see discussion on
callback vs polling).

Xi-lib aims to offer two low-level interfaces. A native Rust crate that can be easily ingested through cargo,
and a C API that provides an interface for all other languages.

#### C Interface
The low-level C interface offers 3 methods
	
- xi_start_core()
- xi_send_message()
- xi_start_receiver()

Error handling...

Threading
 - Currently Xi creates threads for plugins. This should be fine.
 
#### Rust

### High Level Interface
#### xi-protocol

## Proposal
Start creation of low-level C-API in xi-editor.
### Changes to Xi
#### RpcLoop
#### Creating lib/rs-native.rs lib/c-ffi.rs

## Discussions
### Callback vs Polling