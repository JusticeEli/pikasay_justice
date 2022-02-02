
const PIKACHU: &str =
    "
`;-.          ___,
  `.`_...._/`.-\"`
\\        /      ,
/()   () \\    .' `-._
|)  .    ()\\  /   _.'
\\  -'-     ,; '. <
;.__     ,;|   > \
/ ,    / ,  |.-'.-'
(_/    (_/ ,;|.<`
\\    ,     ;-`
>   \\    /
(_,-'`> .'
jgs      (_,'
";

const POKEBALL: &str =
    "\
  `;,;.;,;.;.'
              ..:;:;::;:
        ..--''' '' ' ' '''--.
      /' .   .'        '.   .`\
     | /    /            \\   '.|
     | |   :             :    :|
   .'| |   :             :    :|
 ,: /\\ \\.._\\ __..===..__/_../ /`.
|'' |  :.|  `'          `'  |.'  ::.
|   |  ''|    :'';          | ,  `''\
|.:  \\/  | /'-.`'   ':'.-'\\ |  \\,   |
| '  /  /  | / |...   | \\ |  |  |';'|
 \\ _ |:.|  |_\\_|`.'   |_/_|  |.:| _ |
/,.,.|' \\__       . .      __/ '|.,.,\
     | ':`.`----._____.---'.'   |
l42   \\   ` |   |
       ',-,-',             .'-=,=,
";

use std::io::Read;
use structopt::StructOpt;
use colour::*;
use failure::ResultExt;

#[derive(StructOpt, Debug)]
struct CommandLineArgs {
    #[structopt(short, long)]
    ///Activate pokeball mode
    pokeball: bool,
    #[structopt(short, long, default_value = "Justice")]
    ///What pokeman will speak
    message: String,
    #[structopt(short, long, parse(from_os_str))]
    ///File to extract pokeman from
    alt: Option<std::path::PathBuf>,
    #[structopt(short, parse(from_occurrences))]
    ///Number of occurrence of the photo
    repeat: u8,
}
use exitfailure::ExitFailure;
fn main()->Result<(),ExitFailure> {

    let command_line_args = CommandLineArgs::from_args();
    if command_line_args.repeat == 0 {
        print_on_console(&command_line_args)?;
    } else {
        for _ in 0..command_line_args.repeat {
            print_on_console(&command_line_args)?;
        }
    }
    Ok(())
}

fn print_on_console(command_line_args: &CommandLineArgs)->Result<(),ExitFailure> {
    let message = &command_line_args.message;

    if command_line_args.pokeball {
        red_ln!("{}",POKEBALL);
    } else {
        blue_ln!(" {}", "-".repeat(message.len() + 2));
        blue_ln!("| {} |", message);
        blue_ln!(" {}", "-".repeat(message.len() + 2));
        blue_ln!("\\   /");
        blue_ln!(" \\ /");
        match &command_line_args.alt {
            Some(path) => {
                let alt = std::fs::read_to_string(path).with_context(|b|{
                    format!("Error Trying to read from file:{:?}",path)

                })?;
                green_ln!("{}", alt);
            }
            None => {
                green_ln!("{}", PIKACHU);
            }
        }
    }
    Ok(())
}
