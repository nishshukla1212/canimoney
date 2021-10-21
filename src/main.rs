extern crate serde;

use std::io::Read;
use std::convert::TryFrom;

const TOTAL_INCOME:f64 = 5000.00;
const HIDDEN_MONEY_PERCENTAGE:f64 = 0.10;

fn calculate_spending_money() -> f64{
    let bills_vec: Vec<(String, f64)> = vec![(String::from("rent"), 1750.00),
                                             (String::from("mustang"),980.00),
                                             (String::from("mazda"),430.00),
                                             (String::from("gym"),120.00),
                                             (String::from("regus"),219.00),
                                             (String::from("credit_card"),300.00),
                                             (String::from("internet"),45.00),
                                             (String::from("groceries"),280.00),
                                             (String::from("car_insurance"),166.00),
                                             (String::from("marijuana"),200.00),
                                             (String::from("Evie"),200.00),
    ];

    let mut spending_money: f64 = 0.00;
    let mut total_bills: f64 = 0.00;
    for vec in bills_vec {
        total_bills += vec.1
    }
    spending_money = TOTAL_INCOME-total_bills;
    spending_money=(spending_money*0.75)-(spending_money*HIDDEN_MONEY_PERCENTAGE);
    println!("{}",spending_money);
    spending_money
}


fn main() {
    let spending_money:f64 = calculate_spending_money();
    let mut actual_operands = Operands {
        query_money: 10.00,
        spending_money,
        level: Level::Hippie,
        action: Action::Spend,
        commit: false
    };

    let result = golden_calc(&mut actual_operands);
    println!("{:?}",result)
}


enum Level {
    Hippie,
    Modest,
    Hotshot,
}

impl Level {
    fn getLevel(&self) -> f64 {
        match self {
            Level::Hippie => {0.25}
            Level::Modest => {0.5}
            Level::Hotshot => {0.75}
            _ => {0.25}
        }
    }
}

enum Action {
    Save,
    Spend,
}

impl Action {
    fn getAction(&self) -> f64 {
        match self {
            Action::Save => {1.00}
            Action::Spend => {1.25}
            _ => {1.00}
        }
    }
}

struct Operands {
    query_money: f64,
    spending_money: f64,
    level: Level,
    action: Action,
    commit: bool
}

fn golden_calc(operands: &mut Operands) -> f64 {
   let mut spending_new: f64 = operands.spending_money;
    let level = operands.level.getLevel();
   let calculated_percentage = calculate(operands);
    if level <= calculated_percentage {
        println!("deny");
    } else {
        spending_new = operands.spending_money - operands.query_money;
        if operands.commit {
            operands.spending_money = spending_new
        }
        println!("approve")
    }
    return spending_new
}

fn calculate(op:&mut Operands) -> f64 {
    op.query_money / op.spending_money * op.action.getAction()
}