address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun find_max(x: u128, y: u128) : u128 {

            if (x > y) {
                
                x
            } else {

                y
            }
        }

        #[test]
        fun testing() {

            let max_number = find_max(11111111, 1000000); 

            print(&max_number); // [debug] 11111111
        }
    }
}

// aptos move test

