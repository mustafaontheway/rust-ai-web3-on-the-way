fn main() {

    let data_out: i32 = -202_500;

    {
        let mut data_in = 203_000;

        data_in += data_out;

        println!("Data inner: {data_in}") // Data inner: 500
    }

    //println!("Data inner: {data_in}") // Error! => cannot find value `data_in` in this scope

}

