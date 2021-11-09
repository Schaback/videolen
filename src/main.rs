use anyhow::Result;
use ffmpeg_next as ffmpeg;
use humantime;
use prettytable::{cell, row, table};
use std::{path::PathBuf, time::Duration};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "videolen", about = "Extract and tally video length")]
struct Config {
    #[structopt(required = true, parse(from_os_str))]
    inputs: Vec<PathBuf>,

    #[structopt(short, long, help = "Print in seconds")]
    seconds: bool,

    #[structopt(short, long, help = "Only print summary")]
    quiet: bool,
}
fn format_duration(config: &Config, duration: Duration) -> String {
    if config.seconds {
        duration.as_secs().to_string()
    } else {
        humantime::format_duration(duration).to_string()
    }
}

fn main() -> Result<()> {
    let config = Config::from_args();
    let mut table = table!(["File path", "Duration"]);
    let mut total_duration = Duration::default();
    for path in config.inputs.iter() {
        let context = ffmpeg::format::input(path)?;
        if context.duration() == ffmpeg::ffi::AV_NOPTS_VALUE {
            continue;
        }
        let duration = Duration::from_secs_f64(
            context.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE),
        );
        total_duration += duration;
        table.add_row(row![
            &path.to_string_lossy(),
            r->format_duration(&config, duration)
        ]);
    }
    table.add_row(row![b->"TOTAL:", br->format_duration(&config, total_duration)]);
    if config.quiet {
        println!("{}", format_duration(&config, total_duration));
    } else {
        table.printstd();
    }
    Ok(())
}
