/*
    Name: Reverse polish notation calculator
    Description: Simple reverse polish notation calculator with bad UI implemented in Rust

    Author: https://vk.com/scoped_lock
    Github: https://github.com/PatriotRossii
    
    Made for the glory of my Lord Jesus Christ and Dima.
*/

use std::io;
use std::process::exit;

struct ReversePolishCalculator {
    stack: Vec<i64>,
}

impl ReversePolishCalculator {
	fn push_to_stack(&mut self, arg: i64) {
        self.stack.push(arg);
    }

    fn imply_operation(&mut self, operation: char) {
        let second_operand = self.stack.pop();
        let second_operand = match second_operand {
            None => panic!("Can't get second operand"),
            Some(i) => i,
        };

        let first_operand = self.stack.pop();
        let first_operand = match first_operand {
            None => panic!("Can't get first operand"),
            Some(i) => i,
        };

        self.push_to_stack(match operation {
            '+' => first_operand + second_operand,
            '-' => first_operand - second_operand,
            '*' => first_operand * second_operand,
            '/' => first_operand / second_operand,
            _ => panic!("Invalid operation"),
        });
    }

    fn get_last(&mut self) -> i64 {
        let result = self.stack.pop();
        match result {
            None => panic!("Too few values"),
            Some(i) => return i,
        };
    }
}

fn main() {
    let mut calculator = ReversePolishCalculator{stack: vec![]};

    println!("Zero action code: push an integer");
    println!("First action code: imply entered operation");
    println!("Second action code: get the last value");
    println!("Third action code: exit\n");

    loop {
        let mut line = String::new();

        println!("What you want to do? ");
        io::stdin().read_line(&mut line).expect("Failed to read line");

        let action: u8 = line.trim().parse().expect("Please type a number!");
        match action {
            0 => {
                println!("Please, type a number you want to push: ");

                let mut line = String::new();
                io::stdin().read_line(&mut line).expect("Failed to read line");
                let value_to_push: i64 = line.trim().parse().expect("Please type a number!");

                calculator.push_to_stack(value_to_push);
            },
            1 => {
                println!("Please, type an operation you want to imply: ");

                let mut line = String::new();
                io::stdin().read_line(&mut line).expect("Failed to read line");
                let operation: char = line.trim().parse().expect("Please type a character!");

                calculator.imply_operation(operation);
            },
            2 => {
                println!("Last value: {}", calculator.get_last());
            },
            3 => exit(0),
            _  => println!("Please, enter correct action number"),
        }
    }    
}