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
    // Optimization. The number of arguments is less than or equal to 10.
    ( $a1:tt, $a2:tt $(,)? ) => {
        format_args!("{}{}", $a1, $a2)
    };
    ( $a1:tt, $a2:tt, $a3:tt $(,)? ) => {
        format_args!("{}{}{}", $a1, $a2, $a3)
    };
    ( $a1:tt, $a2:tt, $a3:tt, $a4:tt $(,)? ) => {
        format_args!("{}{}{}{}", $a1, $a2, $a3, $a4)    
    };
    ( $a1:tt, $a2:tt, $a3:tt, $a4:tt, $a5:tt $(,)? ) => {
        format_args!("{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5)
    };
    ( $a1:tt, $a2:tt, $a3:tt, $a4:tt, $a5:tt, $a6:tt $(,)? ) => {
        format_args!("{}{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5, $a6)
    };
    ( $a1:tt, $a2:tt, $a3:tt, $a4:tt, $a5:tt, $a6:tt, $a7:tt $(,)? ) => {
        format_args!("{}{}{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5, $a6, $a7)
    };
    ( $a1:tt, $a2:tt, $a3:tt, $a4:tt, $a5:tt, $a6:tt, $a7:tt, $a8:tt $(,)? ) => {
        format_args!("{}{}{}{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8)
    };
    ( $a1:tt, $a2:tt, $a3:tt, $a4:tt, $a5:tt, $a6:tt, $a7:tt, $a8:tt, $a9:tt $(,)? ) => {
        format_args!("{}{}{}{}{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9)
    };
    ( $a1:tt, $a2:tt, $a3:tt, $a4:tt, $a5:tt, $a6:tt, $a7:tt, $a8:tt, $a9:tt, $a10:tt $(,)? ) => {
        format_args!("{}{}{}{}{}{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9, $a10)
    };
    // Any number of arguments.
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
    macro_rules! check {
        ( $test_name:tt; $($a:tt),* $(,)? ) => {
            #[test]
            fn $test_name() {
                let result = format!("{}", format_all_args!($($a),*));
                assert_eq!(result, concat!($($a),*));
            }
        };
    }
    
    check!(a1; 1);
    check!(a2; 1,2);
    check!(a3; 1,2,3);
    check!(a4; 1,2,3,4);
    check!(a5; 1,2,3,4,5);
    check!(a6; 1,2,3,4,5,6);
    check!(a7; 1,2,3,4,5,6,7);
    check!(a8; 1,2,3,4,5,6,7,8);
    check!(a9; 1,2,3,4,5,6,7,8,9);
    check!(a10; 1,2,3,4,5,6,7,8,9,10);
    check!(long_list; 1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0);

    #[test]
    fn no_colon() {
        let result = format!("{}", format_all_args!(1,2,3,4,5 ));
        //                                                   ^
        assert_eq!(result, "12345");
        
        let result = format!("{}", format_all_args!(1,2,3,4,5,6,7,8,9,0,1,2 ));
        //                                                                 ^
        assert_eq!(result, "123456789012");
    }
    
    #[test]
    fn colon() {
        let result = format!("{}", format_all_args!(1,2,3,4,5,));
        //                                                   ^
        assert_eq!(result, "12345");
        
        let result = format!("{}", format_all_args!(1,2,3,4,5,6,7,8,9,0,1,2,));
        //                                                                 ^
        assert_eq!(result, "123456789012");
    }
    
    macro_rules! optional_arg_test { ( $($a:expr)? ) => { optional_arg!($($a)?) }; }
    //                                           ^                           ^
    
    #[test]
    fn optional_arg_test() {        
        let result = format!("{}", format_all_args!(1,2,3,4,5,(optional_arg_test!(6)),7));
        //                                                                        ^
        assert_eq!(result, "1234567");
        //                       ^
        
        let result = format!("{}", format_all_args!(1,2,3,4,5,6,7,8,9,0,1,2,3,(optional_arg_test!(4)),5));
        //                                                                                        ^
        assert_eq!(result, "123456789012345");
        //                               ^
    }

    #[test]
    fn optional_arg_test_none() {
        let result = format!("{}", format_all_args!(1,2,3,4,5,(optional_arg_test!( )),7));
        //                                                                        ^
        assert_eq!(result, "123457");
        //                      ^^
        
        let result = format!("{}", format_all_args!(1,2,3,4,5,6,7,8,9,0,1,2,3,(optional_arg_test!( )),5));
        //                                                                                        ^
        assert_eq!(result, "12345678901235");
        //                              ^^
    }
}
