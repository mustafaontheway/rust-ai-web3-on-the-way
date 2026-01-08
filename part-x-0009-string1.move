address my_temp_addr {

    module Daily {

        use std::debug::print;
        use std::string::{String, utf8};

        fun str_ops() {

            let say_hi1: String = utf8(b"Hi there!");

            print(&(say_hi1));

        }

        #[test]
        fun testing() {

            str_ops();
        }
    }
}
