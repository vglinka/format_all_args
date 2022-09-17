#[cfg(test)]
mod tests {
    use format_all_args::{format_all_args, optional_arg};

    #[test]
    fn it_work() {    
        macro_rules! optional_arg_test { ( $($a:expr)? ) => { optional_arg!($($a)?) }; }
        //                                           ^                           ^
        
        let result = format!("{}", format_all_args!(1,2,3,4,5,optional_arg_test!( ),7));
        //                                                                       ^
        assert_eq!(result, "123457");
    }
}
