address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun operations() {

            let x: u16 = 97;
            let y: u16 = 11;

            x = 95;

            print(&(x + y));
            print(&(x - y));
            print(&(x * y));
            print(&(x / y));
            print(&(x % y));
        }

        #[test]
        fun testing() {

            operations();
        }
    }
}

// [debug] 106
// [debug] 84
// [debug] 1045
// [debug] 8
// [debug] 7
