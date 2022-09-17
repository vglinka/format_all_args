## format_all_args macro

Formats any number of arguments without heap allocation.

Additionally, the library provides macro `optional_arg` which may be
required when programming macros with optional arguments.

**main.rs**

```rust
use format_all_args::{format_all_args, optional_arg};

fn main() {
    macro_rules! optional_arg_test { ( $($a:expr)? ) => { optional_arg!($($a)?) }; }
    //                                 ----------^                      -----^
    //                                 optional                         optional
    //
    let result = format!("{}", format_all_args!(1,2,3,4,5,optional_arg_test!( ),7));
    assert_eq!(result, "123457");
}
```
