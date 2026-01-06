address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun sum(x: u32, y: u32) : u32 {

            return x + y
        }

        #[test]
        fun testing() {

            let result = sum(11111, 1000); 

            print(&result); // [debug] 12111
        }
    }
}
