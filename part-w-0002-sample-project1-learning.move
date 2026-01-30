module my_sample_addr::learning {

    use std::debug;

    public entry fun print_year() {

        let year = 2026;

        debug::print(&year);
    }


    #[test]
    public fun test_print_year() {
       
       print_year();
    }

}

// sui move test
