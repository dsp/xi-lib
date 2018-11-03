// Copyright 2018 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
extern crate xi_core_lib;
extern crate xi_rpc;

use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::io;
use std::sync::mpsc;

use xi_core_lib::{XiCore};

/// We are keeping our internal state in a private struct and only expose
/// a simple handler to callees. Callees must go through the public interface
/// to interact with internals.
struct XiInternalState {
    core: XiCore,
    rpc_loop: xi_rpc::RpcLoop<io::Stdout>,
    recv_message: RecvMessageCallback,
}

impl XiInternalState {
    fn new(cb: RecvMessageCallback) -> Self {
        XiInternalState {
            core: XiCore::new(),
            rpc_loop: xi_rpc::RpcLoop::new(io::stdout()), // TODO: stdout() is actually not needed
            recv_message: cb,
        }
    }
}

type RecvMessageCallback = extern "C" fn (msg: *const c_char, len: u32);

#[repr(C)]
pub struct XiHandle {
    internal: *mut XiInternalState,
}

/// Allocate Xi internal state.
#[no_mangle]
pub unsafe extern "C" fn xi_create(cb: RecvMessageCallback) -> *mut XiHandle {
    let internal = Box::new(XiInternalState::new(cb));
    let xi = Box::new(
        XiHandle {
            internal: Box::into_raw(internal),
        });
    Box::into_raw(xi)
}

/// Starts Xi on the current thread. This should be on a designated Xi thread
#[no_mangle]
pub unsafe extern "C" fn xi_start_core(xi: *mut XiHandle) {
    let internal = (*xi).internal;
    (*internal).rpc_loop.embedded_mainloop(&mut (*internal).core);
}

#[no_mangle]
pub unsafe extern "C" fn xi_start_receiver(xi: *mut XiHandle) {
    // let internal = (*xi).internal;
    // (*internal).rpc_loop.rx;
}

#[no_mangle]
pub unsafe extern "C" fn xi_send_message(xi: *mut XiHandle, cmsg: *const c_char, len: u32) -> bool {
    let internal = (*xi).internal;
    let msg = CStr::from_ptr(cmsg);
    let mut reader = io::BufReader::new(msg.to_bytes()); // I am unsure howe exspensive this is
    (*internal).rpc_loop.send_message(&mut reader);
    true
}

/// Shutdown an Xi instance. Must be called from the same thread as xi_start
#[no_mangle]
pub unsafe extern "C" fn xi_shutdown(xi: *mut XiHandle) {
}

/// Destruct the XiHandler object correctly
#[no_mangle]
pub unsafe extern "C" fn xi_free(xi: *mut XiHandle) {
    if ! xi.is_null() && ! (*xi).internal.is_null() {
        std::mem::drop(Box::from_raw((*xi).internal));
        std::mem::drop(Box::from_raw(xi));
    }
}
