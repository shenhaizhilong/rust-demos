

enum Res {
    Ok(i64),
    Err(String),
}

enum Expression {
    OP {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Value(i64),
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}


fn eval(e: Expression) -> Res {
    match e {
        Expression::OP { op, left, right } => {
            let left_res = match eval(*left) {
                Res::Ok(v) => v,
                Res::Err(msg) => return Res::Err(msg),
            };

            let right_res = match eval(*right) {
                Res::Ok(v) => v,
                Res::Err(msg) => return Res::Err(msg),
            };

            Res::Ok(match op {
                Operation::Add => left_res + right_res,
                Operation::Sub => left_res - right_res,
                Operation::Mul => left_res * right_res,
                Operation::Div => {
                    if right_res == 0 {
                        return Res::Err(String::from("division by zero"));
                    } else {
                        left_res / right_res
                    }
                }
            })
        }
        Expression::Value(v) => Res::Ok(v),
    }
}
