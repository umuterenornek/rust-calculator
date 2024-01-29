slint::include_modules!();

macro_rules! println {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::println!($($rest)*)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Associativity {
    LEFT,
    RIGHT,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Operator {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}
impl Operator {
    fn get_precedence(&self) -> i64 {
        match self {
            Operator::ADD => 1,
            Operator::SUBTRACT => 1,
            Operator::MULTIPLY => 2,
            Operator::DIVIDE => 2,
        }
    }

    fn get_associativity(&self) -> Associativity {
        match self {
            Operator::ADD => Associativity::LEFT,
            Operator::SUBTRACT => Associativity::LEFT,
            Operator::MULTIPLY => Associativity::LEFT,
            Operator::DIVIDE => Associativity::LEFT,
        }
    }

    fn cmp_precedence(&self, other: &Operator) -> std::cmp::Ordering {
        self.get_precedence().cmp(&other.get_precedence())
    }
}
impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '+' => Operator::ADD,
            '-' => Operator::SUBTRACT,
            '*' => Operator::MULTIPLY,
            '/' => Operator::DIVIDE,
            _ => panic!("Invalid operator"),
        }
    }
}
impl From<String> for Operator {
    fn from(s: String) -> Self {
        match s.as_str() {
            "+" => Operator::ADD,
            "-" => Operator::SUBTRACT,
            "*" => Operator::MULTIPLY,
            "/" => Operator::DIVIDE,
            _ => panic!("Invalid operator"),
        }
    }
}

fn is_operator(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/'
}

fn calculate_result(input: String) -> f64 {
    let mut operator_stack: Vec<char> = vec![];
    let mut num_unparsed: Vec<char> = vec![];
    let mut output_buffer: Vec<String> = vec![];

    for c in input.chars() {
        if c.is_digit(10) || c == '.'{
            let digit = c;
            num_unparsed.push(digit);
            continue;
        }

        if !num_unparsed.is_empty() {
            let number = num_unparsed.iter().collect::<String>();
            output_buffer.push(number);
            num_unparsed.clear();
        }

        if is_operator(c) {
            let cur_operator = Operator::from(c);
            while !operator_stack.is_empty() && is_operator(*operator_stack.last().unwrap()) {
                let last_operator = *operator_stack.last().unwrap();
                if
                cur_operator.get_associativity() == Associativity::LEFT && cur_operator.cmp_precedence(&Operator::from(last_operator)) != std::cmp::Ordering::Greater
                ||
                cur_operator.get_associativity() == Associativity::RIGHT && cur_operator.cmp_precedence(&Operator::from(last_operator)) == std::cmp::Ordering::Less {
                    let last_operator = operator_stack.pop().unwrap();
                    output_buffer.push(last_operator.to_string());
                    continue;
                }

                break;
            }
            operator_stack.push(c);
            continue;
        }
        if c == '(' {
            operator_stack.push(c);
            continue;
        }
        if c == ')' {
            while !operator_stack.is_empty() && *operator_stack.last().unwrap() != '(' {
                println!("opt stack: {:?}", operator_stack);
                println!("out_buf: {:?}", output_buffer);
                let last_operator = operator_stack.pop().unwrap();
                println!("last opt: {:?}", last_operator);
                output_buffer.push(last_operator.to_string());
                println!("out_buf after: {:?}", output_buffer);
            }
            operator_stack.pop();
        }
    }
    while !num_unparsed.is_empty() {
        let number = num_unparsed.iter().collect::<String>();
        output_buffer.push(number);
        num_unparsed.clear();
    }

    while !operator_stack.is_empty() {
        let last_operator = operator_stack.pop().unwrap();
        output_buffer.push(last_operator.to_string());
    }

    println!("{:?}", output_buffer);

    let mut calc_stack: Vec<f64> = vec![];
    while !output_buffer.is_empty() {
        let token = output_buffer.remove(0);
        if is_operator(token.chars().next().unwrap()) {
            let right = calc_stack.pop().unwrap_or(0.0);
            let left = calc_stack.pop().unwrap_or(0.0);
            let operator = Operator::from(token);
            match operator {
                Operator::ADD => calc_stack.push(left + right),
                Operator::SUBTRACT => calc_stack.push(left - right),
                Operator::MULTIPLY => calc_stack.push(left * right),
                Operator::DIVIDE => calc_stack.push(left / right),
            }
        } else {
            calc_stack.push(token.parse::<f64>().unwrap());
        }
    }

    calc_stack.pop().unwrap()
}

fn check_parentheses(input: String) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in input.chars() {
        if c == '(' {
            stack.push(c);
        }
        if c == ')' {
            if stack.is_empty() {
                return false;
            }
            stack.pop();
        }
    }
    stack.is_empty()
}

fn check_result_validity(result: String) -> bool {
    let last_char_of_result = result.chars().rev().next().unwrap();
    check_parentheses(result.clone()) && result != "inf" && result != "-inf" && result != "NaN" && !is_operator(last_char_of_result)
}

fn check_dot_acceptable(result: String) -> bool {
    let tokens = result.split(|c| c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')');
    tokens.rev().next().unwrap().chars().filter(|c| *c == '.').count() == 0
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_clear({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_result("0".into());
        }
    });

    ui.on_request_remove_last_char({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let result: String = ui.get_result().into();
            if result.len() == 1 {
                ui.set_result("0".into());
                return;
            }
            ui.set_result(result[..result.len() - 1].into());
        }
    });

    ui.on_request_append_to_result({
        let ui_handle = ui.as_weak();
        move |text| {
            let ui = ui_handle.unwrap();
            let result: String = ui.get_result().into();
            let last_char_of_result = result.chars().rev().next().unwrap();
            let text: char = text.chars().next().unwrap();
            match (result.as_str(), text) {
                ("0", _) if text.is_digit(10) || text == '(' => {
                    ui.set_result(text.into());
                    return;
                }
                (_, '.') if !last_char_of_result.is_digit(10) || !check_dot_acceptable(result.clone()) => {
                    return;
                }
                (_, ')') => {
                    if is_operator(last_char_of_result) || last_char_of_result == '(' || last_char_of_result == '.' || check_parentheses(result.clone()) {
                        return;
                    }
                }
                (_, '(') => {
                    if !is_operator(last_char_of_result) {
                        return;
                    }
                }
                ("inf" | "-inf" | "NaN", _) => {
                    ui.set_result(text.into());
                    return;
                }
                (_, _) if is_operator(text) => {
                    if last_char_of_result == '(' || last_char_of_result == '.' {
                        return;
                    }
                    if is_operator(last_char_of_result) {
                        ui.set_result(format!("{}{}", result[..result.len() - 1].to_string(), text).into());
                        return;
                    }
                }
                (_, _) if text.is_digit(10) && last_char_of_result == ')' => {
                    return;
                }
                _ => {}
            }
            ui.set_result(format!("{}{}", result, text).into());
        }
    });

    ui.on_request_calculate({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let result: String = ui.get_result().into();
            if !check_result_validity(result.clone()) {
                return;
            }
            let result = calculate_result(result);
            if result.fract() == 0.0 {
                ui.set_result(result.to_string().into());
                return;
            }
            ui.set_result(format!("{:.5}", result).into());
        }
    });

    ui.run()
}
