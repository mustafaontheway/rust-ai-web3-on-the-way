address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun profit_calc(sales: u128, cost: u128) : u128 {

            if (cost > sales) {

                0
            } else {

                sales - cost
            }
        }

        #[test]
        fun testing() {

           print(&(profit_calc(5_000_000, 4_750_000)));

           print(&(profit_calc(5_000_000, 6_650_000)));
        }
    }
}

// [debug] 250000
// [debug] 0
