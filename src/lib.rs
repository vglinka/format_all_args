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
    // 0 args
    () => { "" };
    // 1 arg
    ( $arg:expr $(,)? ) => {
        format_args!("{}", $arg)
    };
    // Optimization. The number of arguments is less than or equal to 10.
    ( $a1:expr, $a2:expr $(,)? ) => {
        format_args!("{}{}", $a1, $a2)
    };
    ( $a1:expr, $a2:expr, $a3:expr $(,)? ) => {
        format_args!("{}{}{}", $a1, $a2, $a3)
    };
    ( $a1:expr, $a2:expr, $a3:expr, $a4:expr $(,)? ) => {
        format_args!("{}{}{}{}", $a1, $a2, $a3, $a4)    
    };
    ( $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr $(,)? ) => {
        format_args!("{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5)
    };
    ( $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr $(,)? ) => {
        format_args!("{}{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5, $a6)
    };
    ( $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr $(,)? ) => {
        format_args!("{}{}{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5, $a6, $a7)
    };
    ( $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr $(,)? ) => {
        format_args!("{}{}{}{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8)
    };
    ( $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr $(,)? ) => {
        format_args!("{}{}{}{}{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9)
    };
    ( $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr $(,)? ) => {
        format_args!("{}{}{}{}{}{}{}{}{}{}", $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9, $a10)
    };
    // Any number of arguments > 10.
    // At each level of recursion, we subtract 10 arguments from
    // the total number of arguments.
    ( $arg_1:expr, $arg_2:expr, $arg_3:expr, $arg_4:expr, $arg_5:expr, $arg_6:expr, $arg_7:expr, $arg_8:expr, $arg_9:expr, $arg_10:expr, $($args:expr),* $(,)? ) => {        
        format_args!("{}{}{}{}{}{}{}{}{}{}{}", $arg_1, $arg_2, $arg_3, $arg_4, $arg_5, $arg_6, $arg_7, $arg_8, $arg_9, $arg_10, format_all_args!($($args),*))
    };
}

#[macro_export] macro_rules!
optional_arg {
    (           ) => { ""     };
    ( $($a:expr)* ) => { $($a)* };
}

#[cfg(test)]
mod tests {
    macro_rules! check {
        ( $test_name:tt; $($a:expr),* $(,)? ) => {
            #[test]
            fn $test_name() {
                let result = format!("{}", format_all_args!($($a),*));
                assert_eq!(result, concat!($($a),*));
            }
        };
    }
    
    macro_rules! optional_arg_test { ( $($a:expr)? ) => { optional_arg!($($a)?) }; }
    //                                 ----------^                      -----^
    //                                 optional                         optional
    
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
    fn no_args() {
        let result = format!("{}", format_all_args!( ));
        //                                          ^
        assert_eq!(result, "");
    }
    
    #[test]
    fn no_colon() {
        let result = format!("{}", format_all_args!(1 ));
        //                                           ^
        assert_eq!(result, "1");
        
        let result = format!("{}", format_all_args!(1,2,3,4,5 ));
        //                                                   ^
        assert_eq!(result, "12345");
        
        let result = format!("{}", format_all_args!(1,2,3,4,5,6,7,8,9,0,1,2 ));
        //                                                                 ^
        assert_eq!(result, "123456789012");
        
        let result = format!("{}", format_all_args!(optional_arg_test!(6) ));
        //                                                               ^
        assert_eq!(result, "6");
    }
    
    #[test]
    fn colon() {
        let result = format!("{}", format_all_args!(1,));
        //                                           ^
        assert_eq!(result, "1");
    
        let result = format!("{}", format_all_args!(1,2,3,4,5,));
        //                                                   ^
        assert_eq!(result, "12345");
        
        let result = format!("{}", format_all_args!(1,2,3,4,5,6,7,8,9,0,1,2,));
        //                                                                 ^
        assert_eq!(result, "123456789012");
        
        let result = format!("{}", format_all_args!(optional_arg_test!(6),));
        //                                                               ^
        assert_eq!(result, "6");
    }
        
    #[test]
    fn optional_arg_test() {             
        let result = format!("{}", format_all_args!(1,2,3,4,5,optional_arg_test!(6),7));
        //                                                                       ^
        assert_eq!(result, "1234567");
        //                       ^
        
        let result = format!("{}", format_all_args!(1,2,3,4,5,6,7,8,9,0,1,2,3,optional_arg_test!(4),5));
        //                                                                                       ^
        assert_eq!(result, "123456789012345");
        //                               ^
    }

    #[test]
    fn optional_arg_test_none() {
        let result = format!("{}", format_all_args!(1,2,3,4,5,optional_arg_test!( ),7));
        //                                                                       ^
        assert_eq!(result, "123457");
        //                      ^^
        
        let result = format!("{}", format_all_args!(1,2,3,4,5,6,7,8,9,0,1,2,3,optional_arg_test!( ),5));
        //                                                                                       ^
        assert_eq!(result, "12345678901235");
        //                              ^^
    }
}
