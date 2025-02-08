use heck::ToShoutySnakeCase;
use regex::Regex;
use std::fs::{File, create_dir_all};
use std::io::{self, Write};
use std::path::Path;
use std::sync::LazyLock;

// get cmdids from proto
static CMD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\sCmd\w*\s=\s\d+"#).unwrap());

// If you change these, make sure to change `src/lib.rs`
// Prost output file is `_.rs`
const OUTPUT_DIR: &str = "out/";
const CMD_OUT: &str = "./out/cmd.rs";

// Source files
const PROTO_FILE: &str = "3.proto";
// not necessary, i just like preallocating.
// u can find this by doing a regex search in vscode
const CMD_LINE_CNT: usize = 1915;

fn main() -> io::Result<()> {
    if !Path::new(OUTPUT_DIR).exists() {
        create_dir_all(OUTPUT_DIR)?;
    }

    if Path::new(PROTO_FILE).exists() {
        println!("cargo::rerun-if-changed={}", PROTO_FILE);

        prost_build::Config::new()
            .out_dir(OUTPUT_DIR)
            .compile_protos(&[PROTO_FILE], &["."])
            .expect("Failed to compile protobuf");

        let cmd_output = parse_cmd_ids(PROTO_FILE)?;

        let mut file = File::create(CMD_OUT)?;

        writeln!(file, "{}", cmd_output.join("\n"))?;

        Ok(())
    } else {
        panic!("`{}` does not exist", PROTO_FILE);
    }
}

fn parse_cmd_ids(proto: &str) -> io::Result<Vec<String>> {
    use std::fs::read_to_string;
    let content = read_to_string(proto)?;

    let mut results = Vec::with_capacity(CMD_LINE_CNT);
    for cap in CMD_REGEX.captures_iter(&content) {
        if let Some(matched) = cap.get(0) {
            let cmd_line = matched.as_str();
            let stripped = cmd_line.replace("\tCmd", "");
            let parts: Vec<&str> = stripped.split(" = ").collect();
            if parts.len() == 2 {
                let constant_name = parts[0].to_shouty_snake_case();
                let value = parts[1];
                results.push(format!("pub const {}: u16 = {};", constant_name, value));
            }
        }
    }

    Ok(results)
}
