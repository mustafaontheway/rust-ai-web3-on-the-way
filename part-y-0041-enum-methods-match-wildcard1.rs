fn main() {
    
    let strategy1 = InvestmentStrategy::Buy;

    strategy1.invest_or_not(); // You can buy or sell. You are free!

    let strategy2 = InvestmentStrategy::Keep;

    strategy2.invest_or_not(); // Winter is coming. You should keep them!
}

enum InvestmentStrategy {

    Buy,
    Sell,
    Keep
}

impl InvestmentStrategy {

    fn invest_or_not(&self) {

        match self {
            
            InvestmentStrategy::Keep => println!("Winter is coming. You should keep them!"),
            _ => println!("You can buy or sell. You are free!")
        }
    }
    
}

