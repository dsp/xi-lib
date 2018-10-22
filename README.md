xi-lib: Embedding xi-editor using C FFI
=======================================

Threading
---------
xi-lib does not manage threads by it's own. Instead it is the responsibility of the embedder to create and manage
threads. This allows embedders to create multiple instances of Xi and manage their threads depending on the target
platform and the runtime.

xi-lib requires embedder to provide 2 tasks for every Xi instance.
- Xi message receiver task
- Xi message sender task

Todo
----
- [ ] Basic API
  - [ ] Design: See Flutter Embedded API as a starting point. https://github.com/flutter/engine/blob/master/shell/platform/embedder/embedder.h
  - [ ] Documentation
  - [ ] Error Handling using https://michael-f-bryan.github.io/rust-ffi-guide/errors/return_types.html
- [ ] Proof-of-concept:
  -  [ ] Build xi-core using xi-lib

Authors
-------
Written by David Soria Parra