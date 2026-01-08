address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun logic() {

            let x: bool = true;
            let y: bool = false;
            let z: bool = true;

            print(&(x != y &&  x == z));

            print(&(x != y ||  x != z));

            print(&((x != y &&  x == z) || y != z));

            print(&(!x));
        }

        #[test]
        fun testing() {

            logic();
        }
    }
}

// [debug] true
// [debug] true
// [debug] true
// [debug] false
