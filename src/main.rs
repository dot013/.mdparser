use core::panic;
use std::io::Write;

use clap::{ArgAction, Parser, Subcommand};
use clio::{Input, Output};

use mdparser::convert;
use mdparser::links;
use mdparser::utils;

#[derive(Parser, Debug)]
#[command(version = "0.1", about = "", long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true, default_value = "lines")]
    list_format: cli::ListFormat,

    #[arg(short, long, global = true, default_value = "-")]
    input: Input,

    #[arg(short, long, global = true, default_value = "-")]
    output: Output,

    #[arg(long)]
    surpress_errors: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Links {
        #[arg(short, long, action = ArgAction::SetTrue)]
        list: bool,

        #[arg(short, long, num_args = 2, value_names = ["FROM", "TO"])]
        replace_url: Vec<String>,

        #[arg(long, default_value = ".")]
        root: clio::ClioPath,
    },
    Frontmatter {
        #[command(subcommand)]
        command: FrontmatterCommands,
    },
    Convert {
        #[arg(short, long)]
        format: convert::Formats,
    },
}

#[derive(Debug, Subcommand)]
enum FrontmatterCommands {
    Set {
        #[clap()]
        property: String,

        #[clap(num_args(1..))]
        value: Vec<String>,

        #[arg(short, long, action = ArgAction::SetTrue)]
        json: bool,
    },
    Get {
        #[clap()]
        property: String,

        #[arg(short, long, action = ArgAction::SetTrue)]
        error_on_unfound: bool,

        #[arg(short, long, action = ArgAction::SetTrue)]
        stderr_on_unfound: bool,
    },
}

fn main() {
    let mut cli = Cli::parse();

    let file = match std::io::read_to_string(&mut cli.input) {
        Ok(s) => s,
        Err(e) => {
            cli::print_error(
                cli::Error {
                    code: cli::ErrorCode::EIORD,
                    description: format!("Failed to read input\n{e:#?}"),
                    fix: Some(String::from(
                        "Try to check if you have input permission to input file",
                    )),
                    url: None,
                },
                cli.surpress_errors,
            );
            return ();
        }
    };
    let arena = comrak::Arena::new();
    let ast = comrak::parse_document(&arena, &file, &mdparser::utils::default_options());

    let result = match cli.command {
        Commands::Links {
            list,
            root,
            replace_url,
        } => {
            let list = if replace_url.len() == 0 && !list {
                true
            } else {
                list
            };

            replace_url.chunks(2).for_each(|p| {
                links::iterate_links(ast, |l| {
                    if l.url == p[0] {
                        l.url = (*p[1]).to_string()
                    }
                })
            });

            if list {
                let links = links::get_links(ast);
                cli::ResultType::List(links)
            } else {
                let mut str = vec![];
                match comrak::format_commonmark(ast, &utils::default_options(), &mut str) {
                    Ok(_) => cli::ResultType::String(String::from_utf8(str).unwrap()),
                    Err(e) => cli::ResultType::Err(cli::Error {
                        code: cli::ErrorCode::EPRSG,
                        description: format!("Error formatting ast back to markdown\n{e:#?}"),
                        fix: None,
                        url: None,
                    }),
                }
            }
        }
        _ => cli::ResultType::Err(cli::Error {
            description: "".to_string(),
            code: cli::ErrorCode::EPRSG,
            url: None,
            fix: None,
        }),
    };

    if let cli::ListFormat::JSON = &cli.list_format {
        if cli.output.is_tty() {
            cli.list_format = cli::ListFormat::PrettyJSON
        }
    }

    if let cli::ResultType::Err(e) = result {
        cli::print_error(e, cli.surpress_errors);
        return;
    }

    let str = match cli::result_to_str(result, &cli.list_format) {
        Ok(s) => s,
        Err(e) => {
            cli::print_error(e, cli.surpress_errors);
            return;
        }
    };

    match cli.output.write(str.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            cli::print_error(
                cli::Error {
                    code: cli::ErrorCode::EIOWR,
                    description: format!("Error trying to write result to output's writer\n{e:#?}"),
                    fix: Some(String::from(
                        "Try to check if you have write permission to output file",
                    )),
                    url: None,
                },
                cli.surpress_errors,
            );
        }
    }
}

mod cli {
    use core::panic;
    use std::fmt;

    #[derive(Clone, Debug, clap::ValueEnum)]
    pub enum ListFormat {
        Lines,
        Comma,
        JSON,
        UglyJSON,
        PrettyJSON,
    }

    #[derive(Debug)]
    pub enum ErrorCode {
        EPRSG,
        EIORD,
        EIOWR,
    }
    impl fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = format!("{:?}", self);
            write!(f, "{}", s)
        }
    }

    pub struct Error {
        pub description: String,
        pub code: ErrorCode,
        pub fix: Option<String>,
        pub url: Option<String>,
    }
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let title = match self.code {
                ErrorCode::EPRSG => "Parsing error",
                ErrorCode::EIORD => "IO error on read operation",
                ErrorCode::EIOWR => "IO error on write operation",
            };

            let fix = if let Some(fix) = &self.fix {
                format!("\nFix: {}", fix)
            } else {
                String::new()
            };

            let url = if let Some(url) = &self.url {
                format!("\nMore info: {}", url)
            } else {
                String::new()
            };

            write!(
                f,
                "Error {:?} - {:?} \n{}{}{}",
                self.code, title, self.description, fix, url
            )
        }
    }

    pub enum ResultType<T>
    where
        T: fmt::Display + fmt::Debug + serde::Serialize,
    {
        List(Vec<T>),
        String(String),
        Err(Error),
    }

    #[derive(serde::Serialize)]
    struct YAMLList<T: fmt::Display + fmt::Debug + serde::Serialize> {
        list: Vec<T>,
    }

    pub fn result_to_str<T: fmt::Display + fmt::Debug + serde::Serialize>(
        result: ResultType<T>,
        list_format: &ListFormat,
    ) -> Result<String, Error> {
        match result {
            ResultType::List(list) => match list_format {
                ListFormat::Lines => Ok(list
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
                    .to_string()),
                ListFormat::Comma => Ok(list
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string()),
                ListFormat::UglyJSON | ListFormat::JSON => {
                    serde_json::to_string(&list).map_err(|e| Error {
                        description: format!(
                            "Failed to parse list vector into a JSON output \
                        on line {}, column {}. Used vector: \n{:#?}",
                            e.line(),
                            e.column(),
                            list
                        ),
                        code: ErrorCode::EPRSG,
                        url: None,
                        fix: None,
                    })
                }
                ListFormat::PrettyJSON => serde_json::to_string_pretty(&list).map_err(|e| Error {
                    description: format!(
                        "Failed to parse list vector into a JSON output \
                    on line {}, column {}. Used vector: \n{:#?}",
                        e.line(),
                        e.column(),
                        list
                    ),
                    code: ErrorCode::EPRSG,
                    url: None,
                    fix: None,
                }),
            },
            ResultType::String(s) => Ok(s),
            _ => Ok(String::new()),
        }
    }

    pub fn print_error(err: Error, panics: bool) {
        eprintln!("{}", err);
        if panics {
            panic!();
        }
    }
}
