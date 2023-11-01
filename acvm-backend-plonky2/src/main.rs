use std::{
    eprintln,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    process::exit,
};

use acir::circuit::Circuit;

const VERSION: &str = "0.1.0";
const LOG_FILE: &str = "/home/vitkov/plonky2-backend-log.txt";

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

fn dump_args(log: &mut File) -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    writeln!(log, "-------------")?;
    write!(log, "Called with arguments args: ")?;
    let concatenated_string = args.join(" ") + "\n";
    log.write(concatenated_string.as_bytes())?;
    writeln!(log, "")?;
    Ok(())
}

fn run() -> io::Result<()> {
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)?;

    dump_args(&mut log)?;

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
            Ok(())
        }
        "info" => {
            let output = r#"{
                "language": {
                    "name": "PLONK-CSAT",
                    "width": 3
                },
                "opcodes_supported": ["arithmetic"],
                "black_box_functions_supported": []
            }"#;

            let outfile = options.output.unwrap_or("info.json".to_owned());
            if outfile == "-" {
                print!("{}", output);
            } else {
                fs::write(outfile, output).expect("failed to write to file");
            }
            Ok(())
        }
        "prove" => {
            let outfile = options.output.unwrap_or("./proofs/proof".to_owned());

            let file = File::open(options.bytecode_path)?;
            let circuit = Circuit::read(file)?;

            writeln!(log, "Circuit: {}", circuit)?;

            if outfile == "-" {
                println!(
                    "pproofproofproofproofproofproofproofproofproofproofproofproofroofproofproof"
                );

                writeln!(log, "info written to stdout")?;
            } else {
                writeln!(log, "info written to {outfile}")?;
            }
            Ok(())
        }
        _ => {
            eprintln!("unknown command {cmd}");
            exit(1);
        }
    }
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("fatal error: {}", err);
            exit(1);
        }
    }
}
