address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun learn_move() {

            let current_address: address = @my_temp_addr;

            print(&current_address);
        }

        #[test]
        fun test_learn_move() {

            learn_move();
        }
    }
}

// aptos move test

