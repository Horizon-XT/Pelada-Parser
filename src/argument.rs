use std::process;

//use crate::file;
use crate::application_error;

pub enum OperationMode {
    API,
    Service,
    CLI,
}

const OPERATIONS: [(&str, OperationMode); 3] = [
    ("--api", OperationMode::API),
    ("--service", OperationMode::Service),
    ("--cli", OperationMode::CLI),
];

const HELP_MENU: &str = "Choose one of the availables:

    CLI -> <executable> --cli <filepath>
    API -> <executable> --api
    Service -> <executable> --service <file_repository_path>\n";

fn get_operation(command: &str) -> Result<OperationMode, application_error::ApplicationError> {
    for (key, value) in OPERATIONS {
        if command == key {
            return Ok(value);
        }
    }

    let invalid_operation = application_error::ApplicationError::InvalidOperation;
    Err(invalid_operation)
}

pub fn parse_args(args: Vec<String>) {
    let argc: usize = args.len();

    match argc {
        1 => {
            println!("[No execution mode selected]");
            println!("{}", HELP_MENU.to_string());
            process::exit(1);
        }
        2 => {
            let result_operation = get_operation(&args[1]);
            match result_operation {
                Ok(_operation) => {
                    println!("Valid operation!");
                }
                _ => {
                    println!("[Invalid Operation]");
                    println!("{}", HELP_MENU.to_string());
                    process::exit(1);
                }
            }
        }
        _ => {
            println!("Unhandled argument case!");
            println!("{}", HELP_MENU.to_string());
            process::exit(1);
        }
    }
}
