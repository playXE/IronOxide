# IronOxide

Rust MIR to .NET CLI translator.

This project aims to create translator from MIR to .NET CLI and allow run Rust programs on .NET Core with support of using classes and functions from .NET Core.



# TODO
- Assembler for CIL
- Basic translator
- Pattern matching
- FFI with .NET

  Example:
```rust
use System::Console;

fn main() {
    Console::WriteLine("Hello,world!");
}

```