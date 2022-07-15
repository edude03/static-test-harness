use proc_macro2::TokenStream;
use serde::Deserialize;
use std::fmt::format;
use std::fs;
use std::path::Path;
use std::{env, io::BufRead, path::PathBuf, process::Command};

#[derive(Debug, Deserialize)]
struct Target {
    name: String,
    test: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]

enum Message {
    CompilerArtifact { executable: PathBuf, target: Target },
    Other { reason: String },
}

fn get_test_paths(input: PathBuf) -> Result<Vec<PathBuf>, ()> {
    let out = Command::new("cargo")
        .arg("test")
        .arg("--no-run")
        .arg("--message-format")
        .arg("json")
        .current_dir(input)
        .output()
        .expect("failed to execute process");

    //dbg!(out.stdout);
    let lines = out.stdout.lines();

    let stuff = lines
        .flat_map(|l| l.map(|s| serde_json::from_str::<Message>(&s)))
        .map(|m| match m {
            Ok(Message::CompilerArtifact { executable, target }) => {
                let file_clone = executable.clone();
                let file_name = file_clone.file_name().unwrap();
                println!("Found {} {:?}", &target.name, file_name);
                Some(executable)
            }
            _ => {
                println!("Got other");
                None
            }
        })
        .filter_map(|x| x)
        .collect::<Vec<PathBuf>>();

    Ok(stuff)
}

fn main() {
    let input = fs::canonicalize(env::var("INPUT_DIR").unwrap()).unwrap();
    let output = env::var("OUT_DIR").unwrap();
    println!("output: {output}");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tests.rs");
    let paths = get_test_paths(input).unwrap();
    let path_strs = paths
        .iter()
        .map(|s| format!(r#"{}"#, s.to_str().unwrap()))
        .collect::<Vec<String>>();
    let printme = format!("{}", path_strs.join(","));

    let out: Vec<TokenStream> = paths
        .iter()
        .map(|f| {
            let x = f.to_str().unwrap();
            quote::quote! { #x }
        })
        .collect();

    let out = quote::quote! {
        use std::{env, io::BufRead, path::PathBuf, process::Command};
        const TEST_PATHS: &'static [&'static str] = &[ #printme ];

        pub fn run_tests() {
            for s in TEST_PATHS {
                Command::new(s).status().expect("failed");
            }
        }
    };

    fs::write(&dest_path, TokenStream::from(out).to_string()).unwrap();
    // println!("cargo:rerun-if-changed=build.rs");
    // fs::create_dir(&output).expect("couldn't create dir");
    // panic!("just because");
}
