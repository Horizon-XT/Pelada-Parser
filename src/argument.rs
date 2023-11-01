use crate::application_error;

pub enum OperationMode {
    API,
    Service,
    CLI,
}

const HELP_STRING: &str = "Choose one of the expected usages:
    CLI -> <executable> --cli <filepath>
    API -> <executable> --api
    Service -> <executable> --service <file_repository_path>\n";

pub fn parse_args(args: Vec<String>) {
    let argc: usize = args.len();

    //let incorrect_usage_error = application_error::ApplicationError::IncorrectUsage(HELP_STRING);

    println!("{}", HELP_STRING.to_string());
}
