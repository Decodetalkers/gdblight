mod backlight;
use backlight::BackLightInfo;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "gdblight")]
#[command(about="a tool to set backlight", long_about = None)]
enum Cli {
    #[command(short_flag = 'A')]
    Add {
        percent: u32,
    },
    #[command(short_flag = 'D')]
    Decrease {
        percent: u32,
    },
    #[command(short_flag = 'T')]
    To {
        percent: u32,
    },
    CurrentInfo,
}

fn main() {
    let backlight = match BackLightInfo::new() {
        Ok(backlight) => backlight,
        Err(e) => {
            eprintln!("Error, maybe not backlight file, {e}");
            return;
        }
    };

    let current_percent = backlight.get_light_percent();

    let args = Cli::parse();

    let final_percent = match args {
        Cli::To { percent } => percent,
        Cli::Add { percent } => current_percent + percent,
        Cli::Decrease { percent } => {
            if percent > current_percent {
                eprintln!("cannot decrease that much");
                return;
            }
            current_percent - percent
        }
        Cli::CurrentInfo => {
            let current_light = backlight.current_light();
            let max_light = backlight.max_light();
            println!("current_light: {current_light}");
            println!("max_light:     {max_light}");
            println!("light_percent: {current_percent}");
            return;
        }
    };
    println!("will set light to {} percent", final_percent);
    backlight.set_light_percent(final_percent).unwrap();
}
