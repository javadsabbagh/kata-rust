# VS Code installation configuration

[VS Code](https://code.visualstudio.com/) is an open source code editor produced by Microsoft. It inherits Microsoft's excellent genes in the IDE field and is an editor/IDE with considerable potential. .

VScode currently also has good support for Rust.



## Download VSCode

Please open the official website https://code.visualstudio.com/ to download the editor.

## Dependencies

As mentioned in the first section of this chapter, prepare the four things `racer`, `rust source code`, `rustfmt`, `rls`, and configure the corresponding environment variables, so I won’t repeat them here.

## Install the Rust extension Rust

1. Open the VScode editor;
2. Press Ctrl + p to open the command panel;
3. In the input box that appears in the upper part of the editor, enter `ext install vscode-rust`, and it will automatically search for available plug-ins. After searching, click to install;
4. Use `VScode` to open any `.rs` file, and the plug-in will automatically guide the user to complete the configuration when it starts for the first time.

Note: It is recommended to use the RLS mode, that is, to use [Rust Langular Server](https://github.com/rust-lang-nursery/rls) to provide various functional support
