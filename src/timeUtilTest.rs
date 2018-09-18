#![feature(plugin, decl_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]


#[cfg(test)]
mod timeUtilTests {

    use timeUtil::get_current_time_milli;

    #[test]
    fn test_get_current_time_milli() {
        let now = get_current_time_milli();
        println!("current time {} ", now);
        assert_eq!( now > 0, true);
        
    }
}
