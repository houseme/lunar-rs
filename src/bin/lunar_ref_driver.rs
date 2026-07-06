use std::env;
use std::process::ExitCode;

use lunar_rs::Solar;
use lunar_rs::differential_support::solar_snapshot;

fn usage(program: &str) -> String {
    format!(
        "usage:\n  {program} solar <year> <month> <day> <hour> <minute> <second>\n\noutputs newline-delimited key=value pairs"
    )
}

fn parse_i32(name: &str, raw: &str) -> Result<i32, String> {
    raw.parse::<i32>().map_err(|err| format!("failed to parse {name} `{raw}` as i32: {err}"))
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let program = args.first().cloned().unwrap_or_else(|| "lunar_ref_driver".to_string());

    if args.len() != 8 || args[1] != "solar" {
        return Err(usage(&program));
    }

    let year = parse_i32("year", &args[2])?;
    let month = parse_i32("month", &args[3])?;
    let day = parse_i32("day", &args[4])?;
    let hour = parse_i32("hour", &args[5])?;
    let minute = parse_i32("minute", &args[6])?;
    let second = parse_i32("second", &args[7])?;

    let solar = Solar::from_ymd_hms(year, month, day, hour, minute, second)
        .map_err(|err| format!("invalid solar input: {err}"))?;

    for (key, value) in solar_snapshot(solar) {
        println!("{key}={value}");
    }

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
