use std::fs::read_to_string;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use anyhow::{Context, Result};
use clap::{Arg, Command};

fn main() -> Result<()> {
    let matches = Command::new("tsq")
        .version("1.0.0")
        .author("Richard Mathieson <richard@tutero.com.au>")
        .arg(Arg::new("file").index(1).required(true))
        .arg(Arg::new("query").index(2).required(true))
        // .arg(arg!(-l --lang <STRING> "Language of the source file").required(true))
        .get_matches();
    let extract = |key: &str| matches.value_of(key).unwrap();

    let source = read_to_string(extract("file"))?;
    let query = read_to_string(extract("query"))?;
    let res = setup_parser()?;

    perform_query(res.0, res.1, &source, &query)
}

fn setup_parser() -> Result<(Parser, Language)> {
    let mut parser = Parser::new();
    extern "C" {
        fn tree_sitter_yaml() -> Language;
    }
    let lang = unsafe { tree_sitter_yaml() };
    parser.set_language(lang)?;
    Ok((parser, lang))
}

fn perform_query(mut parser: Parser, lang: Language, source: &str, query: &str) -> Result<()> {
    let tree = parser.parse(source, None).unwrap();
    let query = Query::new(lang, query).context("Could not construct query")?;
    let root_node = tree.root_node();

    let mut qc = QueryCursor::new();
    let provider = source.as_bytes();
    for qm in qc.matches(&query, root_node, provider) {
        let package_name = qm.captures[0]
            .node
            .utf8_text(provider)
            .context("Could not parse package-name")?;
        let version = qm.captures[2]
            .node
            .utf8_text(provider)
            .context("Could not parse version")?;

        println!("{}: {}", package_name, version);
    }

    Ok(())
}
