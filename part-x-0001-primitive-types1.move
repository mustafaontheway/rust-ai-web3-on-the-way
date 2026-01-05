address hello_blockchain {

    module Daily {

        use std::debug::print;

        fun learn_move() {

            let is_completed: bool = false;

            let ready: bool = true;

            let check1: bool = 5 != 6;

            let check2: bool = 3 == 3;

            print(&is_completed);
            print(&ready);
            print(&check1);
            print(&check2);
        }

        #[test]
        fun test_learn_move() {

            learn_move();
        }
    }
}


// aptos move test

