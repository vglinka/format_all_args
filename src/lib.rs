// Copyright (c) 2022 Vadim Glinka
//
// See the COPYRIGHT file at the top-level directory of this distribution
// and at https://github.com/vglinka/format_all_args/blob/main/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export] macro_rules!
format_all_args {
    ( $arg:tt, $($args:tt),* $(,)? ) => { format_args!("{}{}", $arg, format_all_args!($($args),*)) };
    ( $arg:tt                      ) => { format_args!("{}"  , $arg                              ) };   
}

#[macro_export] macro_rules!
optional_arg {
    (           ) => { ""     };
    ( $($a:tt)* ) => { $($a)* };
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_colon() {
        let result = format!("{}", format_all_args!(1,2,3,4,5 ));
        //                                                   ^
        assert_eq!(result, "12345");
    }
    
    #[test]
    fn colon() {
        let result = format!("{}", format_all_args!(1,2,3,4,5,));
        //                                                   ^
        assert_eq!(result, "12345");
    }
    
    macro_rules! optional_arg_test { ( $($a:expr)? ) => { optional_arg!($($a)?) }; }
    //                                           ^                           ^
    
    #[test]
    fn optional_arg_test() {        
        let result = format!("{}", format_all_args!(1,2,3,4,5,(optional_arg_test!(6)),7));
        //                                                                        ^
        assert_eq!(result, "1234567");
        //                       ^
    }

    #[test]
    fn optional_arg_test_none() {
        let result = format!("{}", format_all_args!(1,2,3,4,5,(optional_arg_test!( )),7));
        //                                                                        ^
        assert_eq!(result, "123457");
        //                      ^^
    }
}
