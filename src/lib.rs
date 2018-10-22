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

use std::os::raw::c_char;

use xi_core_lib::{XiCore};

/// We are keeping our internal state in a private struct and only expose
/// a simple handler to callees. Callees must go through the public interface
/// to interact with internals.
struct XiInternalState {
    core: XiCore,
}

impl XiInternalState {
    fn new() -> Self {
        XiInternalState {
            core: XiCore::new(),
        }
    }
}

#[repr(C)]
pub struct XiHandle {
    version: u32,
    internal: XiInternalState,
}

#[no_mangle]
pub extern "C" fn xi_init(xi: &XiHandle) -> XiHandle {
    XiHandle {
        version: 1,
        internal: XiInternalState::new(),
    }
}

#[no_mangle]
pub extern "C" fn xi_send_message(xi: XiHandle, msg: *const c_char) {

}

#[no_mangle]
pub extern "C" fn xi_shutdown() {
}