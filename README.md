# TERMINUS

A TUI library for Rust. Or, rather, for me to learn Rust by writing a TUI library.

The creation of this library [was mostly streamed on YouTube](https://www.youtube.com/playlist?list=PLicC_uGS5eIKvYrRzh-_CnqDLb5ved3MQ).

### TODO

- [ ] Create a ForEach view
- [ ] Audit pub vs pub(crate) functions

### Inspiration + Blatant Theft

- The view DSL is inspired by SwiftUI.
- Various aspects of idiomatic Rust DSL design were pilfered from [rui](https://github.com/audulus/rui), particularly for ViewId and State management.
- The rendering logic is _HEAVILY_ inspired by [ratatui](https://github.com/ratatui-org/ratatui).
