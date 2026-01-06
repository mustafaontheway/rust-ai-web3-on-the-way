address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun sum_or_mult(x: u128, y: u128) : (u128, u128) {

            (x + y, x * y)
        }

        #[test]
        fun testing() {

            let (sum, mult) = sum_or_mult(700, 7777777);

            print(&sum);

            print(&mult);
        }
    }
}

// [debug] 7778477
// [debug] 5444443900
