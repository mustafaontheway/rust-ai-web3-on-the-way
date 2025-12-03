fn main() {
    
    let strategy1 = InvestmentStrategy::Buy; // Your strategy is 'Buy'. You can buy or sell. You are free!

    strategy1.invest_or_not(); 

    let strategy2 = InvestmentStrategy::Keep; // Winter is coming. You should keep them!

    strategy2.invest_or_not(); 
}

#[derive(Debug)]
enum InvestmentStrategy {

    Buy,
    Sell,
    Keep
}

impl InvestmentStrategy {

    fn invest_or_not(&self) {

        match self {
            
            InvestmentStrategy::Keep => println!("Winter is coming. You should keep them!"),
            _other_strategies => println!("Your strategy is '{_other_strategies:?}'. You can buy or sell. You are free!")
        }
    }
    
}

