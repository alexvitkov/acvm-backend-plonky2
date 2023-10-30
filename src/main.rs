use std::{eprintln, fs::{self, OpenOptions}, process::exit, io::{Write, self}};

const VERSION: &str = "0.1.0";

struct Options {
    // not sure what some of these are, but the official backend takes them in as options:
    // https://github.com/AztecProtocol/aztec-packages/blob/master/barretenberg/cpp/src/barretenberg/bb/main.cpp#L362
    pub bytecode_path: String,
    pub witness_path: String,
    pub proof_path: String,
    pub vk_path: String,
    pub crs_path: String,
    pub output: Option<String>,
}

fn dump_args() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/home/vitkov/args.txt")?;

    // Join the strings with spaces and write to the file
    let concatenated_string = args.join(" ");
    file.write_all(concatenated_string.as_bytes())?;
    Ok(())
}

fn main() {
    dump_args();

    let mut options = Options {
        bytecode_path: "./target/acir.gz".to_string(),
        witness_path: "./target/witness.gz".to_string(),
        proof_path: "./proofs/proof".to_string(),
        vk_path: "./target/vk".to_string(),
        crs_path: "./crs".to_string(),
        output: None,
    };

    let args: Vec<String> = std::env::args().collect();
    let cmd = &args[1];

    let mut idx = 1;
    while idx < args.len() {
        match args[idx].as_str() {
            "-b" => {
                idx += 1;
                options.bytecode_path = args[idx].clone();
            }
            "-w" => {
                idx += 1;
                options.witness_path = args[idx].clone();
            }
            "-p" => {
                idx += 1;
                options.proof_path = args[idx].clone();
            }
            "-k" => {
                idx += 1;
                options.vk_path = args[idx].clone();
            }
            "-c" => {
                idx += 1;
                options.crs_path = args[idx].clone();
            }
            "-o" => {
                idx += 1;
                options.output = Some(args[idx].clone());
            }
            _ => {}
        }
        idx += 1;
    }

    match cmd.as_str() {
        "--version" => {
            println!("{}", VERSION);
        }
        "info" => {
            let output = r#"{
                "language": {
                    "name": "PLONK_CSAT",
                    "width": 3
                },
                "opcodes_supported": ["arithmetic"],
                "black_box_functions_supported": []
            }"#;

            let outfile = options.output.unwrap_or("info.json".to_owned());
            if outfile == "-" {
                eprint!("{}", output);
            } else {
                fs::write(outfile, output).expect("failed to write to file");
            }
        }
        _ => {
            eprintln!("unknown command {cmd}");
            exit(1);
        }
    }
}
