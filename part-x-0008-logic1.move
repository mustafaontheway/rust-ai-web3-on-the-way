address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun logic() {

            let x: u8 = 11;
            let y: u8 = 14;

            print(&(x == y));
            print(&(x != y));
            print(&(x > y));
            print(&(x >= y));
            print(&(x < y));
            print(&(x <= y));

            print(&(x < y && x != y));

            print(&(x < y || x == y));
        }

        #[test]
        fun testing() {

            logic();
        }
    }
}

// [debug] false
// [debug] true
// [debug] false
// [debug] false
// [debug] true
// [debug] true
// [debug] true
// [debug] true
