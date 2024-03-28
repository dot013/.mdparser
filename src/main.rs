use core::panic;
use std::borrow::Borrow;

use clap::{ArgAction, Parser, Subcommand};
use clio::*;
use comrak::nodes::NodeValue;

use mdparser::{
    convert,
    frontmatter::{self, Frontmatter},
    links, utils,
};

#[derive(Parser, Debug)]
#[command(version = "0.1", about = "", long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "-")]
    input: Input,

    #[arg(short, long, default_value = "-")]
    output: Output,

    #[arg(long)]
    surpress_errors: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Links {
        #[arg(short, long)]
        path_root: clio::ClioPath,

        #[arg(short, long, default_value = "x_alias_url")]
        alias_prop: String,

        #[arg(long)]
        to_absolute_paths: bool,

        #[arg(long)]
        not_remove_unalised: bool,

        #[arg(long)]
        not_remove_unfound: bool,
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

    let file = std::io::read_to_string(&mut cli.input).unwrap_or_else(|err| panic!("{err:#?}"));
    let arena = comrak::Arena::new();
    let ast = comrak::parse_document(&arena, &file, &mdparser::utils::default_options());

    if let Commands::Convert { format } = &cli.command {
        let r = match format {
            convert::Formats::TumblrNPF => convert::to_tumblr_npf(&ast),
        };
        println!("{:#?}", r.borrow());
        return;
    }

    match &cli.command {
        Commands::Links {
            path_root,
            alias_prop,
            to_absolute_paths,
            not_remove_unalised,
            not_remove_unfound,
        } => utils::iter_nodes(&ast, &|node| {
            if let NodeValue::Link(ref mut link) = &mut node.data.borrow_mut().value {
                match links::parse(
                    node,
                    link,
                    &links::ParseOptions {
                        path_root: path_root.to_path_buf(),
                        alias_prop: Some(String::from(alias_prop)),
                        to_complete_paths: *to_absolute_paths,
                        remove_unalised: !*not_remove_unalised,
                        remove_unfound: !*not_remove_unfound,
                    },
                ) {
                    Ok(_) => (),
                    Err(err) => {
                        if !&cli.surpress_errors {
                            panic!("{err:#?}\n");
                        } else {
                            eprint!("{err:#?}\n");
                        }
                    }
                };
            }
        }),
        Commands::Frontmatter { command } => utils::iter_nodes(&ast, &|node| {
            if let NodeValue::FrontMatter(ref mut f) = &mut node.data.borrow_mut().value {
                let mut frontmatter: Frontmatter<serde_yaml::Value> = match Frontmatter::new(f) {
                    Ok(f) => f,
                    Err(e) => panic!("{e:#?}"),
                };
                match command {
                    FrontmatterCommands::Set {
                        property,
                        value,
                        json,
                    } => {
                        frontmatter.set(
                            &property,
                            frontmatter::to_yaml_value(value.to_vec(), !*json),
                        );
                    }
                    FrontmatterCommands::Get {
                        property,
                        error_on_unfound,
                        stderr_on_unfound,
                    } => {
                        let v = frontmatter.get(property);
                        if let Some(v) = v {
                            print!("{v:#?}")
                        } else if *error_on_unfound {
                            panic!()
                        } else if *stderr_on_unfound {
                            eprint!("Not Found")
                        }
                    }
                };
                *f = match frontmatter.to_string() {
                    Ok(s) => s,
                    Err(e) => panic!("{e:#?}"),
                };
            }
        }),
        _ => (),
    };

    let _ = comrak::format_commonmark(&ast, &mdparser::utils::default_options(), &mut cli.output);
}
