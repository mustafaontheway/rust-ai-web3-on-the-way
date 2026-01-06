address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun find_smaller(x: u128, y: u128) : u128 {

            if (x > y) {
                
                return y
            } else {

                return x
            }
        }

        #[test]
        fun testing() {

            let small_number = find_smaller(11111111, 1000000); 

            print(&small_number); // [debug] 1000000
        }
    }
}

// aptos move test

