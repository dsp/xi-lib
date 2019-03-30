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

use std::os::raw::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::io;
use std::sync::mpsc;

use xi_core_lib::{XiCore};

/// We are keeping our internal state in a private struct and only expose
/// a simple handler to callees. Callees must go through the public interface
/// to interact with internals.
struct XiInternalState {
    core: XiCore,
    rpc_loop: xi_rpc::CoreRpcLoop,
    recv_message: RecvMessageCallback,
    recv_user_data: *const c_void,
}

impl XiInternalState {
    fn new(cb: RecvMessageCallback, user_data: *const c_void) -> Self {
        XiInternalState {
            core: XiCore::new(),
            rpc_loop: xi_rpc::CoreRpcLoop::new(), // TODO: stdout() is actually not needed
            recv_message: cb,
            recv_user_data: user_data,
        }
    }
}

type RecvMessageCallback = extern "C" fn (msg: *const c_char, len: u32, user_data: *const c_void);

#[repr(C)]
pub struct XiHandle {
    internal: *mut XiInternalState,
}

/// Create an Xi instance and returns its handle.
///
/// This function creates a new instance of the Xi and returns a handle. Memory
/// is owned by xi-lib and internal state is hidden from the callee. You must use
/// xi_shutdown and xi_free to release the handle.
///
/// After creating a xi instance, you likely want to start a thread where xi-core() is run.
/// You need an additional thread to handle callbacks using xi_start_receiver().
#[no_mangle]
pub unsafe extern "C" fn xi_create(cb: RecvMessageCallback, user_data: *const c_void) -> *mut XiHandle {
    eprintln!("xi_create");
    let internal = Box::new(XiInternalState::new(cb, user_data));
    let xi = Box::new(
        XiHandle {
            internal: Box::into_raw(internal),
        });
    Box::into_raw(xi)
}

/// Starts Xi on the current thread.
///
/// This must be on a designated Xi thread.
#[no_mangle]
pub unsafe extern "C" fn xi_start_core(xi: *mut XiHandle) {
    let internal = (*xi).internal;
    (*internal).rpc_loop.mainloop(&mut (*internal).core);
}

/// Receives messages from xi-core and calls the callback handed to xi_create().
///
/// # Thread safety
/// The callback function will called on the thread where xi_start_receiver() is called.
/// The callee is responsbile for thread safety to other threads.
#[no_mangle]
pub unsafe extern "C" fn xi_start_receiver(xi: *mut XiHandle) {
    let internal = (*xi).internal;
    loop {
        let mut raw_peer = (*internal).rpc_loop.get_raw_peer();
        let json_value = xi_rpc::next_receive_wait(&mut raw_peer);
        run_callback(&(*xi), &xi_rpc::value_to_string(&json_value));
    }
}

/// Send a message to Xi.
///
/// Sends a json serialized message to xi-core.
#[no_mangle]
pub unsafe extern "C" fn xi_send_message(xi: *mut XiHandle, cmsg: *const c_char, len: u32) -> bool {
    let internal = (*xi).internal;
    let msg = CStr::from_ptr(cmsg);
    let mut raw_peer = (*internal).rpc_loop.get_raw_peer();
    let mut reader = xi_rpc::MessageReader::default();
    let mut stream = io::BufReader::new(msg.to_bytes()); // I am unsure how exspensive this is
    xi_rpc::send_message(&mut raw_peer, &mut reader, &mut stream);
    // (*internal).rpc_loop.send_message(&mut reader);
    true
}

/// Destruct the XiHandler object correctly
#[no_mangle]
pub unsafe extern "C" fn xi_free(xi: *mut XiHandle) {
    if ! xi.is_null() && ! (*xi).internal.is_null() {
        std::mem::drop(Box::from_raw((*xi).internal));
        std::mem::drop(Box::from_raw(xi));
    }
}

fn run_callback(xi: &XiHandle, msg: &str) {
    let internal = xi.internal;
    // We are generating a CString, to transfer ownership of the string to the callback.
    let c_string = CString::new(msg.as_bytes()).unwrap();
    let str_ptr = c_string.as_ptr();
    let str_len = c_string.as_bytes_with_nul().len() as u32;
    unsafe {
        ((*internal).recv_message)(str_ptr, str_len, (*internal).recv_user_data);
    }
}
