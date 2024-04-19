use std::io::Write;
use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};
use clio::Input;

use mdparser::frontmatter::Frontmatter;
use mdparser::links;
use mdparser::utils;

#[derive(Parser, Debug)]
#[command(version = "0.1", about = "", long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(global = true, default_value = "-")]
    input: Input,

    #[arg(short, long, global = true, action = ArgAction::SetTrue)]
    write: bool,

    #[arg(long, global = true, default_value = "lines")]
    list_format: cli::ListFormat,

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
    },
    Frontmatter {
        #[arg(short, long, action = ArgAction::SetTrue)]
        list: bool,

        #[arg(short, long, num_args = 2, value_names = ["PROPERTY", "VALUE"], allow_negative_numbers = true)]
        set_value: Vec<String>,

        #[arg(short, long, num_args = 2, value_names = ["FROM", "TO"])]
        rename_prop: Vec<String>,
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
        Commands::Links { list, replace_url } => {
            let list = if replace_url.len() == 0 && !list {
                true
            } else {
                list
            };

            // TODO: Remove clone
            replace_url
                .chunks(2)
                .for_each(|p| links::replace_links(ast, p[0].clone(), p[1].clone()));

            if list {
                cli::ResultType::List(links::get_links(ast))
            } else {
                cli::ResultType::Markdown(ast)
            }
        }
        Commands::Frontmatter {
            list,
            set_value,
            rename_prop,
        } => {
            let list = if set_value.len() + rename_prop.len() == 0 && !list {
                true
            } else {
                list
            };

            let mut frontmatter = Frontmatter::try_from(ast).unwrap();

            // I don't care anymore
            set_value
                .chunks(2)
                .map(|c| {
                    if let Ok(j) = serde_json::from_str::<serde_json::Value>(&c[1]) {
                        (c[0].clone(), serde_yaml::to_value(j).unwrap())
                    } else if let Ok(i) = String::from(&c[1]).parse::<f64>() {
                        (c[0].clone(), serde_yaml::to_value(i).unwrap())
                    } else {
                        (c[0].clone(), serde_yaml::to_value(c[1].clone()).unwrap())
                    }
                })
                .for_each(|p| frontmatter.set(&p.0, p.1));

            rename_prop
                .chunks(2)
                .for_each(|p| frontmatter.rename_prop(&p[0], &p[1]));

            frontmatter.place_on_ast(ast);

            if list {
                todo!()
            } else {
                cli::ResultType::Markdown(ast)
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
        if cli.input.is_tty() {
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

    if cli.input.is_std() || !cli.write {
        if let Err(e) = std::io::stdout().write(str.as_bytes()) {
            cli::print_error(
                cli::Error {
                    code: cli::ErrorCode::EIOWR,
                    description: format!("Error trying to write result to stdout\n{e:#?}"),
                    fix: None,
                    url: None,
                },
                cli.surpress_errors,
            );
        }
    } else if let Some(f) = cli.input.path().to_str() {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(PathBuf::from(f));

        match file {
            Ok(mut f) => {
                if let Err(e) = f.write(str.as_bytes()) {
                    cli::print_error(
                        cli::Error {
                            code: cli::ErrorCode::EIOWR,
                            description: format!("Error, failed to write to file\n{e:#?}"),
                            fix: None,
                            url: None,
                        },
                        cli.surpress_errors,
                    );
                }
            }
            Err(e) => cli::print_error(
                cli::Error {
                    code: cli::ErrorCode::EIORD,
                    description: format!("Error, failed to open input's file\n{e:#?}"),
                    fix: None,
                    url: None,
                },
                cli.surpress_errors,
            ),
        }
    } else {
        cli::print_error(
            cli::Error {
                code: cli::ErrorCode::EIOTY,
                description: format!("Error, output is not a valid file"),
                fix: None,
                url: None,
            },
            cli.surpress_errors,
        );
    }
}

mod cli {
    use core::panic;
    use std::{cell::RefCell, fmt};

    use comrak::{arena_tree::Node, nodes::Ast};
    use mdparser::utils;

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
        EIOTY,
    }
    impl fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = format!("{:?}", self);
            write!(f, "{}", s)
        }
    }

    #[derive(Debug)]
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
                ErrorCode::EIOTY => "IO error on input/output type",
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

    #[derive(Debug)]
    pub enum ResultType<'a, T>
    where
        T: fmt::Display + fmt::Debug + serde::Serialize,
    {
        List(Vec<T>),
        String(String),
        Markdown(&'a Node<'a, RefCell<Ast>>),
        Err(Error),
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
            ResultType::Markdown(ast) => {
                let mut str = vec![];
                if let Err(e) = comrak::format_commonmark(ast, &utils::default_options(), &mut str)
                {
                    return Err(Error {
                        code: ErrorCode::EPRSG,
                        description: format!("Error formatting ast back to markdown\n{e:#?}"),
                        fix: None,
                        url: None,
                    });
                }
                match String::from_utf8(str) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(Error {
                        code: ErrorCode::EPRSG,
                        description: format!(
                            "Error making string from utf8, after markdown formatting\n{e:#?}",
                        ),
                        fix: None,
                        url: None,
                    }),
                }
            }
            ResultType::Err(e) => Err(e),
        }
    }

    pub fn print_error(err: Error, panics: bool) {
        eprintln!("{}", err);
        if panics {
            panic!();
        }
    }
}
