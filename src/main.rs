use clap::{Arg, ArgMatches, Command};
use std::fs::{read_to_string, remove_file, File, OpenOptions};
use std::io::Write;

pub fn new() -> ArgMatches {
    Command::new("vec-new")
        .bin_name("vec-new")
        .about("create a rust vec using command line")
        .arg(
            Arg::new("run")
                .long("run")
                .short('r')
                .help("the command to create vec")
                .required(true)
                .require_equals(true),
        )
        .arg(
            Arg::new("comment")
                .long("comment")
                .short('c')
                .help("the const comment")
                .required(true)
                .require_equals(true),
        )
        .arg(
            Arg::new("name")
                .long("name")
                .short('n')
                .help("the name of the constant to create")
                .required(true)
                .require_equals(true),
        )
        .arg(
            Arg::new("type")
                .long("type")
                .short('t')
                .help("the type of the constant to create")
                .required(true)
                .require_equals(true),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("the output to the vec file")
                .required(true)
                .require_equals(true),
        )
        .arg(
            Arg::new("append")
                .long("append")
                .short('a')
                .help("append the new vec the vec file")
                .required(true)
                .require_equals(true),
        )
        .arg(
            Arg::new("edit")
                .long("edit")
                .short('e')
                .help("edit file the vec file after the creation")
                .required(true)
                .require_equals(true),
        )
        .get_matches()
}
fn main() {
    let x = new();
    if let Some(name) = x.get_one::<String>("name") {
        if let Some(comment) = x.get_one::<String>("comment") {
            if let Some(typ) = x.get_one::<String>("type") {
                if let Ok(save) = File::create("const.rs") {
                    if let Some(command) = x.get_one::<String>("run") {
                        if let Some(output) = x.get_one::<String>("output") {
                            if let Some(append) = x.get_one::<String>("append") {
                                let cmd =
                                    command.split_whitespace().collect::<Vec<&str>>().join(" ");
                                if let Ok(mut cmd) = std::process::Command::new("sh")
                                    .stdout(save)
                                    .arg("-c")
                                    .arg(cmd)
                                    .current_dir(".")
                                    .spawn()
                                {
                                    assert!(cmd.wait().is_ok());
                                    if append.eq("true") {
                                        if let Ok(content) = read_to_string("const.rs") {
                                            let lines = content.lines().collect::<Vec<&str>>();
                                            let size = lines.len();
                                            if let Ok(mut file) = OpenOptions::new()
                                                .append(true)
                                                .write(true)
                                                .create(true)
                                                .open(output)
                                            {
                                                assert!(writeln!(
                                                    file,
                                                    "\n#[doc = \"{comment}\"]\nconst {} : {typ} = [{typ};{size}] = [",
                                                    name.to_uppercase()
                                                )
                                                .is_ok());
                                                for line in &lines {
                                                    if typ.eq("&str") {
                                                        assert!(
                                                            writeln!(file, "\"{line}\",").is_ok()
                                                        );
                                                    } else {
                                                        assert!(writeln!(file, "{line},").is_ok());
                                                    }
                                                }
                                                assert!(writeln!(file, "];").is_ok());
                                                assert!(remove_file("const.rs").is_ok());
                                            }
                                        }
                                    } else {
                                        if let Ok(mut file) = OpenOptions::new()
                                            .append(false)
                                            .write(true)
                                            .create(true)
                                            .open(output)
                                        {
                                            if let Ok(content) = read_to_string("const.rs") {
                                                let lines = content.lines().collect::<Vec<&str>>();
                                                let size = lines.len();
                                                assert!(writeln!(
                                                    file,
                                                    "\n#[doc = \"{comment}\"]\nconst {} : {typ} = [{typ};{size}] = [",
                                                    name.to_uppercase()
                                                )
                                                .is_ok());
                                                for line in &lines {
                                                    if typ.eq("&str") {
                                                        assert!(
                                                            writeln!(file, "\"{line}\",").is_ok()
                                                        );
                                                    } else {
                                                        assert!(writeln!(file, "{line},").is_ok());
                                                    }
                                                }
                                                assert!(writeln!(file, "];").is_ok());
                                                assert!(remove_file("const.rs").is_ok());
                                            }
                                        }
                                    }
                                }
                            }
                            if let Some(edit) = x.get_one::<String>("edit") {
                                if edit.eq("true") {
                                    if let Ok(mut editor) = std::process::Command::new("vim")
                                        .arg(output.as_str())
                                        .current_dir(".")
                                        .spawn()
                                    {
                                        assert!(editor.wait().is_ok());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
