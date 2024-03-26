use clap::{Parser, Subcommand};
use clio::*;
use comrak::nodes::NodeValue;

use mdparser::{links, utils};

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
    Not {},
}

fn main() {
    let mut cli = Cli::parse();

    let file = std::io::read_to_string(&mut cli.input).unwrap_or_else(|err| panic!("{err:#?}"));
    let arena = comrak::Arena::new();
    let ast = comrak::parse_document(&arena, &file, &mdparser::utils::default_options());

    // println!("{ast:#?}");

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
        _ => (),
    };

    let _ = comrak::format_commonmark(&ast, &mdparser::utils::default_options(), &mut cli.output);
}
