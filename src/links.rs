use std::io::Write;
use std::{cell::RefCell, collections::HashMap};
use std::{fs, io, path::PathBuf};

use comrak::nodes::{Ast, NodeLink, NodeValue};
use comrak::{arena_tree::Node, Arena};

pub struct ParseOptions {
    pub alias_prop: Option<String>,
    pub path_root: PathBuf,
    pub to_complete_paths: bool,
    pub remove_unalised: bool,
    pub remove_unfound: bool,
}
impl Default for ParseOptions {
    fn default() -> Self {
        ParseOptions {
            alias_prop: None,
            path_root: PathBuf::new(),
            to_complete_paths: true,
            remove_unalised: true,
            remove_unfound: true,
        }
    }
}

#[derive(Debug)]
pub enum ParsingError {
    AliasNotFound { file: String },
    AliasErr(GetAliasError),
    IoErr(io::Error),
}
pub fn parse<'a>(
    node: &'a Node<'a, RefCell<Ast>>,
    link: &mut NodeLink,
    opts: &ParseOptions,
) -> Result<(), ParsingError> {
    let path = match find_path(link, &opts.path_root) {
        Ok(p) => p,
        Err(err) => {
            if opts.remove_unfound {
                node.children().for_each(|n| node.insert_before(n));
                node.detach();
                return Ok(());
            } else {
                return Err(ParsingError::IoErr(err));
            }
        }
    };

    if opts.to_complete_paths {
        link.url = String::from(path.to_string_lossy())
    }

    if let Some(a) = &opts.alias_prop {
        let alias = match get_alias(&path, &a) {
            Ok(a) => a,
            Err(err) => return Err(ParsingError::AliasErr(err)),
        };
        if let Some(v) = alias {
            link.url = v;
        } else if opts.remove_unalised {
            node.children().for_each(|n| node.insert_before(n));
            node.detach();
        } else {
            return Err(ParsingError::AliasNotFound {
                file: link.url.clone(),
            });
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum GetAliasError {
    IoErr(io::Error),
    YamlErr(serde_yaml::Error),
}
pub fn get_alias(path: &PathBuf, alias_prop: &String) -> Result<Option<String>, GetAliasError> {
    let file = match fs::read_to_string(path) {
        Ok(f) => f,
        Err(err) => return Err(GetAliasError::IoErr(err)),
    };

    let arena = Arena::new();
    let ast = comrak::parse_document(&arena, &file, &crate::utils::default_options());

    let alias = crate::utils::iter_nodes_r(ast, &|node| {
        if let NodeValue::FrontMatter(f) = &node.data.borrow().value {
            // Removes starting and trailing "---" delimiters from frontmatter's string.
            let f = String::from(f.split("---").collect::<Vec<&str>>()[1]);

            let map = match serde_yaml::from_str::<HashMap<String, String>>(&f) {
                Ok(m) => m,
                Err(err) => {
                    return Some(Err::<String, GetAliasError>(GetAliasError::YamlErr(err)));
                }
            };

            match map.get(alias_prop) {
                // The hashmap will be dropped anyways and free up space,
                // so whatever
                Some(r) => Some(Ok(r.clone())),
                None => None,
            }
        } else {
            None
        }
    });

    match alias {
        Some(r) => match r {
            Ok(s) => Ok(Some(s)),
            Err(err) => Err(err),
        },
        None => Ok(None),
    }
}

pub fn find_path(link: &NodeLink, path_root: &PathBuf) -> Result<PathBuf, io::Error> {
    find_file(&link.url, &path_root)
}

fn find_file(file: &String, path: &PathBuf) -> Result<PathBuf, io::Error> {
    match fs::read_dir(path)?.find_map(|e| match e {
        Ok(e) => {
            if e.path().is_dir() {
                match find_file(&file, &e.path()) {
                    Ok(r) => Some(r),
                    Err(_) => None,
                }
            } else if file == &e.file_name().to_string_lossy().to_string() {
                Some(e.path())
            } else {
                None
            }
        }
        _ => None,
    }) {
        Some(r) => Ok(r),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
    }
}
