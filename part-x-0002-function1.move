address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun sum(x: u32, y: u32) {

            let r = x + y;

            print(&r);
        }

        #[test]
        fun testing() {

            sum(11, 1000); // [debug] 1011
        }
    }
}
