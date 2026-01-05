address hello_blockchain {

    module Daily {

        use std::debug::print;

        fun learn_move() {

            // from u8 to u128

            let age: u8 = 17;
            let year: u16 = 1970;
            let yearly_sales_tl: u64 = 312_425_650;

            print(&age);
            print(&year);
            print(&yearly_sales_tl);
        }

        #[test]
        fun test_learn_move() {

            learn_move();
        }
    }
}

// [debug] 17
// [debug] 1970
// [debug] 312425650

// aptos move test

