address my_temp_addr {

    module Daily {

        use std::debug::print;
        use std::string::{String, utf8};

        fun str_ops() {

            let say_hi1: String = utf8(b"Hi there!");

            print(&(say_hi1));

            let say_hi2: vector<u8> = b"Hi there!";

            print(&(say_hi2));

            print(&(utf8(say_hi2)));

        }

        #[test]
        fun testing() {

            str_ops();
        }
    }
}

// [debug] "Hi there!"
// [debug] 0x486920746865726521
// [debug] "Hi there!"
