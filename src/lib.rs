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
use std::io::BufReader;
use std::sync::mpsc;

use xi_core_lib::{XiCore};
use xi_rpc::parse::{RpcObject, MessageReader};

/// We are keeping our internal state in a private struct and only expose
/// a simple handler to callees. Callees must go through the public interface
/// to interact with internals.
struct XiInternalState {
    core: XiCore,
    recv_message: RecvMessageCallback,
    tx: mpsc::Sender<RpcObject>,
    rx: mpsc::Receiver<RpcObject>,
reader: MessageReader,
}

impl XiInternalState {
    fn new(cb: RecvMessageCallback) -> Self {
        let (tx, rx) = mpsc::channel();
        XiInternalState {
            core: XiCore::new(),
            reader: MessageReader::default(),
            recv_message: cb,
            tx: tx,
            rx: rx,
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
pub unsafe extern "C" fn xi_start(xi: *mut XiHandle) {
}

#[no_mangle]
pub unsafe extern "C" fn xi_send_message(xi: *mut XiHandle, cmsg: *const c_char, len: u32) -> bool {
    let internal = (*xi).internal;
    let msg = CStr::from_ptr(cmsg);
    let mut reader = BufReader::new(msg.to_bytes()); // I am unsure howe exspensive this is
    let json = match (*internal).reader.next(&mut reader) {
        Ok(json) => json,
        Err(err) => {
            // handle error
            panic!("this ain't no json");
        },
    };
    println!("{}", json.get_method().unwrap());
    if json.is_response() {
        let id = json.get_id().unwrap();
        match json.into_response() {
            Ok(resp) => {
                // let resp = resp.map_err(Error::from);
                // self.peer.handle_response(id, resp);
                true
            }
            Err(msg) => {
                // handle error
                false
            }
        }
        //((*internal).recv_message)(CString::new("hello world").unwrap().as_ptr(), 12);
    } else {
        (*internal).tx.send(json);
        true
    }
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
