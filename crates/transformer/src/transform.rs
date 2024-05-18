mod analysis;
mod convert;
mod registry;

use std::env;
use std::fs;

use analysis::GlobalAnalysis;
use bumpalo::Bump;
use roc_fmt::Ast;
use roc_parse::ast::ValueDef;
use roc_parse::debug;
use serde_json::{Result, Value};
use tower_lsp::lsp_types::Url;
fn transform(input: &str) -> String {
    input.to_uppercase()
}

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <filename.roc>", args[0]);
        std::process::exit(1);
    }

    // Get the input file name
    let input_file_name = &args[1];
    // resolve file path
    let binding = fs::canonicalize(input_file_name).unwrap_or_else(|err| {
        eprintln!("Error resolving path: {}", err);
        std::process::exit(1);
    });
    let full_path = binding.to_str().unwrap_or_else(|| {
        eprintln!("Error converting path to string");
        std::process::exit(1);
    });

    // create url out of a path
    let source_url = Url::from_file_path(full_path).unwrap_or_else(|err| {
        eprintln!("Error creating URL");
        std::process::exit(1);
    });

    // Read the contents of the input file
    let _source = match fs::read_to_string(full_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading {}: {}", input_file_name, err);
            std::process::exit(1);
        }
    };

    let source = _source.clone();

    let mut docs = GlobalAnalysis::new(source_url, _source).documents;

    // println!(
    //     "docs: {:?}",
    //     docs.iter()
    //         .find(|x| x.url().as_str().ends_with("main.roc"))
    //         .unwrap()
    //         .module()
    //         .unwrap()
    // );

    // for url in docs
    //     .iter()
    //     .filter(|x| x.url().as_str().ends_with("transformer/main.roc"))
    //     .map(|x| x.url())
    // {
    //     println!("url: {:?}", url.as_str());
    // }

    let arena = &Bump::new();

    let ast = crate::analysis::parse_ast::Ast::parse(arena, source.as_str())
        .ok()
        .unwrap();

    println!(
        "{:?}",
        ast.defs
            .value_defs
            .iter()
            .filter(|x| match x {
                ValueDef::Body(_, _) => true,
                _ => false,
            })
            .collect::<Vec::<_>>() // docs.iter()
                                   //     .find(|x| x.url().as_str().ends_with("transformer/main.roc"))
                                   //     .unwrap()
                                   //     .module()
                                   //     .unwrap()
    );

    let doc = docs.pop().unwrap();
    // println!("Analysis: {:?}", doc.module().unwrap());

    // Clone the input content before passing it to the transform function
    let transformed_content = transform(&source);

    // Create the output file name by appending ".tr" before the extension
    let output_file_name = format!("{}.tr.roc", input_file_name);

    // Write the transformed content to the output file
    match fs::write(&output_file_name, transformed_content) {
        Ok(_) => println!(
            "Transformation successful. Result saved to {}",
            output_file_name
        ),
        Err(err) => eprintln!("Error writing to {}: {}", output_file_name, err),
    }
}
