# Summary

* [First knowledge of Rust](1st-glance/README.md)
* [Install Rust](install/preface.md)「marvin-min」
  * [Linux](install/install_rust_on_linux.md)
  * [Mac](install/install_rust_on_mac_os.md)
  * [Windows](install/install_rust_on_windows.md)
  * [version management tool: rustup](install/rustup.md)
* [Editor](editors/preface.md)
  * [Preliminary preparation](editors/before.md)「wayslog」
  * [vim](editors/vim.md) "wayslog"
  * [emacs](editors/emacs.md) "tiansiyuan"
  * [vscode](editors/vscode.md)「daogangtang」
  * [atom](editors/atom.md) "wayslog"
  * [sublime](editors/sublime.md)
  * [visual studio](editors/visualstudio.md)「marvinguo」
  * [spacemacs](editors/spacemacs.md) "wayslog"
* [Rust Quick Start](quickstart/quickstart.md) "Naupio"
  * [Rust Travel](quickstart/rust-travel.md)
  * [Variable binding and primitive type](quickstart/primitive-type.md)
  * [arrays, dynamic arrays and strings](quickstart/vector-string.md)
  * [Structure and Enumeration](quickstart/struct-enum.md)
  * [Control Flow](quickstart/control-flow.md)
  * [Function and method](quickstart/function-method.md)
  * [Trait](quickstart/trait.md)
  * [Comments and Documentation](quickstart/comments-document.md)
  * [Input and output stream](quickstart/io-stream.md)
* [Cargo Project Manager](cargo-projects-manager/cargo-projects-manager.md) "fuyingfuying"
* [Basic program structure](flow/preface.md)「daogangtang」
  * [Comment](flow/comment.md)
  * [Condition](flow/condition.md)
  * [cycle](flow/repetition.md)
* [type, operator, and string](type/preface.md) "wayslog"
  * [Basic Types](type/types.md)
  * [Compound Types](type/compound-types.md)
  * [String class](type/string.md)
  * [Basic operators and string formatting](type/operator-and-formatting.md)
* [Function](function/overview.md)「qdao」
  * [function argument](function/arguement.md)
  * [function return value](function/return_value.md)
  * [statements and expressions](function/statement_expression.md)
  * [Higher order function](function/higher_order_function.md)
* [pattern matching](match/overview.md) "wayslog"
  * [match keyword](match/match.md)
  * [pattern pattern](match/pattern.md)
* [Feature Trait](trait/overview.md)「JohnSmithX」
  * [trait keywords](trait/trait.md)
  * [trait object](trait/trait-object.md)
* [generic](generic/generic.md) "stormgbs"
* [mutability, ownership, lease, and lifetime](ownership-system/preface.md) "stormgbs"
  * [ownership](ownership-system/ownership.md)
  * [Borrowing and borrowing](ownership-system/borrowing_reference.md)
  * [Lifetime](ownership-system/lifetime.md)
* [closure](closure/overview.md)「qdao」
  * [Syntax of closure](closure/syntax.md)
  * [Implementation of Closure](closure/implementation.md)
  * [closure as argument and return value](closure/as_argument_return_value.md)
* [Collection Type Collections](collections/overview.md)「wayslog」
  * [Dynamic Array Vec](collections/vec.md)
  * [HashMap HashMap](collections/hashmap.md)
* [iterator](iterator/overview.md) "wayslog"
  * [iterator, adapter, consumer](iterator/iterator.md)
* [Module and package system, Prelude](module/preface.md)「daogangtang」
  * [module module and crate](module/module.md)
  * [Prelude](module/prelude.md)
  * [pub restricted](module/pub-restricted.md)
* [Option, Result and error handling](error-handling/option-result.md)「JohnSmithX」
* [Input and output](io/preface.md)
  * [Standard input and output](io/io.md)
  * [print! macro](io/output.md)
  * [File I/O](io/file-io.md)「tennix」
* [Macro System](macro/macro.md) "tennix"
* [Heap, Stack and Box](heap-stack/heap-stack.md) "tennix"
* [Several smart pointers](rcarc/preface.md)「daogangtang」
  * [Rc, Arc](rcarc/rcarc.md)
  * [Mutex, RwLock](rcarc/mutex.md)
  * [Cell, RefCell](rcarc/cell.md)
* [Several common Traits in the type system](intoborrow/preface.md) 「daogangtang」
  * [Into/From and its application in String and &str conversion](intoborrow/into.md)
  * [AsRef, AsMut](intoborrow/asref.md)
  * [Borrow, BorrowMut, ToOwned](intoborrow/borrow.md)
  * [Deref and Deref coercions](intoborrow/deref.md)
  * [Cow and its use on String and &str](intoborrow/cow.md)
* [Send and Sync](marker/sendsync.md)「daogangtang」
* [Concurrent, parallel, multi-threaded programming](concurrency-parallel-thread/preface.md)「anzhihun」
  * [Thread](concurrency-parallel-thread/thread.md)
  * [Message Passing](concurrency-parallel-thread/message-passing.md)
  * [Shared memory](concurrency-parallel-thread/share-memory.md)
  * [Synchronization](concurrency-parallel-thread/synchronize.md)
  * [parallel](concurrency-parallel-thread/parallel.md)
* [Unsafe, raw pointer](unsafe-rawpointer/preface.md)「JohnSmithX」
  * [Unsafe](unsafe-rawpointer/unsafe.md)
  * [raw pointer](unsafe-rawpointer/raw-pointer.md)
* [FFI](ffi/preface.md) "42"
  * [rust calling ffi function](ffi/calling-ffi-function.md)
  * [Compile rust into a library](ffi/compiling-rust-to-lib.md)
* [Operator Overloading](operator-overloading/operator.md) "wayslog"
* [attr and compiler arguments](attr-and-compiler-arg/preface.md) "elton"
  * [attribute](attr-and-compiler-arg/attribute.md)
  * [Compiler arguments](attr-and-compiler-arg/rustc-option.md)
* [Cargo parameter configuration](cargo-detailed-cfg/cargo-detailed-cfg.md)「fuyingfuying」
* [Testing and Evaluation](testing/preface.md)「daogangtang」
  * [Testing (testing)](testing/threearchtest.md)
  * [benchmark](testing/bench.md)
* [Code Style](coding-style/style.md)「tiansiyuan」
* [Any and reflection](any/any.md) "wayslog"
* [Safety](safe/safety.md)「daogangtang」
* [Common data structure implementation](data-structure/preface.md)「Naupio」
  * [Stack structure](data-structure/stack.md)
  * [Queue](data-structure/queue.md)
  * [Binary tree](data-structure/binary_tree.md)
  * [Priority Queue](data-structure/priority_queue.md)
  * [Linked List](data-structure/linked_list.md)
  * [Graph Structure](data-structure/graph.md)
* [Standard Library Introduction](std/overview.md)「wayslog」
  * [system command: call grep](std/process.md)
  * [Directory operation: simple grep](std/fs-and-path.md)
  * [network module: W echo](std/net.md)
* [Actual Combat](action/preface.md)「wangyu190810」
  * [Actual combat: Json processing](action/json_data/readme.md)
  * [Practice: Getting Started with Web Application Development](action/mysite/readme.md)
  * [Actual combat: using Postgresql database](action/db/readme.md)
* [Appendix-Glossary](appendix/glossary.md) "tennix"
