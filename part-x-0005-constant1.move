address my_temp_addr {

    module Daily {

        use std::debug::print;

        const FIXED_COST: u16 = 14_000;

        fun calculate_profit(sales: u16) : u16 {

            if (sales < FIXED_COST) {

                0
            } else {

                sales - FIXED_COST
            }
        }

        #[test]
        fun testing() {

            let profit1 = calculate_profit(17000);

            print(&profit1); // 3000

            let profit2 = calculate_profit(10000);

            print(&profit2); // 0
        }
    }
}



