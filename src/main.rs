use std::cell::RefCell;
use std::io::Write;
use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};
use clio::Input;

use comrak::arena_tree::Node;
use comrak::nodes::{Ast, LineColumn, NodeValue};
use dot013_mdparser::convert;
use dot013_mdparser::frontmatter::Frontmatter;
use dot013_mdparser::links;

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
enum FrontmatterCommands {
    Set {
        #[clap()]
        property: String,

        #[clap()]
        value: String,
    },
    Remove {
        #[clap()]
        property: String,
    },
    Get {
        #[clap()]
        property: String,

        #[arg(short = 'j', long, action = ArgAction::SetTrue)]
        to_json: bool,
    },
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
        #[command(subcommand)]
        command: FrontmatterCommands,
    },
    Convert {
        #[arg(short, long)]
        format: convert::Formats,
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
    let ast = comrak::parse_document(&arena, &file, &dot013_mdparser::utils::default_options());

    let result = match &cli.command {
        Commands::Links { list, replace_url } => {
            let list = if replace_url.len() == 0 && !list {
                true
            } else {
                *list
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
        Commands::Frontmatter { command } => {
            if let None = ast.children().find(|c| {
                if let NodeValue::FrontMatter(_) = c.data.borrow().value {
                    true
                } else {
                    false
                }
            }) {
                let node = arena.alloc(Node::new(RefCell::from(Ast::new(
                    NodeValue::FrontMatter(String::from("---\n\n---")),
                    LineColumn { line: 0, column: 0 },
                ))));
                ast.prepend(node);
            }

            match Frontmatter::try_from(ast) {
                Ok(mut frontmatter) => match command {
                    FrontmatterCommands::Set { property, value } => {
                        match serde_yaml::from_str(&value) {
                            Ok(value) => {
                                frontmatter.insert(String::from(property), value);
                                frontmatter.insert_ast(ast);
                                cli::ResultType::Markdown(ast)
                            }
                            Err(err) => cli::ResultType::Err(cli::Error {
                                code: cli::ErrorCode::EPRSG,
                                description: format!(
                                    "Error parsing value to Yaml calue:\n{:#?}",
                                    err
                                ),
                                fix: None,
                                url: None,
                            }),
                        }
                    }
                    FrontmatterCommands::Remove { property } => {
                        let _ = frontmatter.remove(String::from(property));
                        frontmatter.insert_ast(ast);
                        cli::ResultType::Markdown(ast)
                    }
                    FrontmatterCommands::Get { property, to_json } => {
                        let value = frontmatter
                            .get(String::from(property))
                            .unwrap_or(&serde_yaml::Value::Null);

                        let result = if *to_json {
                            serde_json::to_string(value).map_err(|err| cli::Error {
                                code: cli::ErrorCode::EPRSG,
                                description: format!(
                                    "Failed to parse frontmatter value to yaml string\n{:#?}",
                                    err
                                ),
                                fix: None,
                                url: None,
                            })
                        } else {
                            serde_yaml::to_string(value).map_err(|err| cli::Error {
                                code: cli::ErrorCode::EPRSG,
                                description: format!(
                                    "Failed to parse frontmatter value to yaml string\n{:#?}",
                                    err
                                ),
                                fix: None,
                                url: None,
                            })
                        };

                        match result {
                            Ok(s) => cli::ResultType::String(s),
                            Err(err) => cli::ResultType::Err(err),
                        }
                    }
                },
                Err(err) => cli::ResultType::Err(cli::Error {
                    code: cli::ErrorCode::EPRSG,
                    description: format!("Error parsing Markdown frontmatter:\n{:#?}", err),
                    fix: None,
                    url: None,
                }),
            }
        }
        Commands::Convert { format } => match format {
            convert::Formats::NPF => match convert::npf::from(ast) {
                Ok(npf) => {
                    let function = if cli.input.is_tty() {
                        serde_json::to_string_pretty
                    } else {
                        serde_json::to_string
                    };

                    match function(&npf).map_err(|e| {
                        cli::ResultType::Err(cli::Error {
                            description: format!(
                                "Failed to parse Tumblr NPF struct to JSON string
                            on line {}, column {}. Used vector: \n{:#?}",
                                e.line(),
                                e.column(),
                                &npf
                            ),
                            code: cli::ErrorCode::EPRSG,
                            url: None,
                            fix: None,
                        })
                    }) {
                        Ok(s) => cli::ResultType::String(s),
                        Err(e) => e,
                    }
                }
                Err(err) => cli::ResultType::Err(cli::Error {
                    description: format!(
                        "Failed to convert to Tumblr NPF format, due to error:\n{:#?}",
                        err
                    ),
                    code: cli::ErrorCode::ECNPF,
                    url: None,
                    fix: None,
                }),
            },
        },
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
    use dot013_mdparser::utils;

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
        ECNPF,
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
                ErrorCode::ECNPF => "Error converting to NPF format",
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
