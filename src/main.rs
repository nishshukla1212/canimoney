fn main() {
    let mut actual_operands = Operands {
        query_money: 10.00,
        spending_money: 100.00,
        level: Level::Hippie,
        action: Action::Spend,
        commit: false
    };

    let result = golden_calc(&mut actual_operands);
    println!("{}",result)
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
            Action::Spend => {1.125}
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
    let action = operands.action.getAction();
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