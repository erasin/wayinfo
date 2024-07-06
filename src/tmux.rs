use clap::{Args, Subcommand};

use crate::{utils::shell_exec, Result};

#[derive(Subcommand)]
pub enum TmuxCommands {
    /// split window and run
    Run(SplitArgs),
    /// open file with helix editor
    Hx(OpenArgs),
}

#[derive(Args)]
pub struct SplitArgs {
    #[arg(short, long)]
    pub vertical: bool,

    #[arg(long)]
    pub project: Option<String>,

    pub commands: Vec<String>,
}

/// {project} {file}:{line}:{col}
#[derive(Args)]
pub struct OpenArgs {
    #[arg(short, long)]
    pub vertical: bool,

    /// {project} folder of project
    pub project: String,
    /// {file}:{line}:{col}
    /// open file on specified line and char position.
    pub file: String,
}

pub fn parse(cmd: &TmuxCommands) -> Result<()> {
    match cmd {
        TmuxCommands::Run(args) => split_run(args),
        TmuxCommands::Hx(args) => hx_open(args),
    }
}

fn split_run(args: &SplitArgs) -> Result<()> {
    let script = args.commands.join(" ");

    let split = match &args.vertical {
        true => "-v",
        false => "-h",
    };

    let project = match &args.project {
        Some(p) => format!("-c {p}"),
        None => "".to_owned(),
    };

    let cmd = format!("tmux split-window {split} {project} & tmux send '{script}' Enter");

    let cmd = tmux_pane(cmd, script, &["sh", "tmux"])?;
    let _output = shell_exec(&cmd)?;
    // println!("{cmd} \n {output}");

    Ok(())
}

fn hx_open(args: &OpenArgs) -> Result<()> {
    let project = args.project.clone();
    let file = args.file.clone();

    let split = match &args.vertical {
        true => "-v",
        false => "-h",
    };

    let cmd =
        format!("tmux split-window {split} {project} & tmux send 'hx -w {project} {file} ' Enter");

    let script = format!(":o {file}");

    let cmd = tmux_pane(cmd, script, &["hx"])?;
    let _output = shell_exec(&cmd)?;
    // println!("{cmd} \n {output}");

    Ok(())
}

fn tmux_pane(cmd: String, script: String, ends: &[&str]) -> Result<String> {
    let has_pane =
        shell_exec("tmux list-panes -F \"#{window_index} #{pane_index} #{pane_current_command}\"")?;

    let cmd = if let Some(has) = has_pane
        .lines()
        .filter(
            |&s| ends.iter().any(|&b| s.ends_with(b)), // s.ends_with("tmux") || s.ends_with("sh")
        )
        .last()
    {
        let parts: Vec<&str> = has.split_whitespace().collect();

        if parts.len() == 3 {
            let window_id = parts[0];
            let pane_id = parts[1];
            // let name = parts[2];
            let cmd = format!("tmux send -t {window_id}.{pane_id} '{script}' Enter");
            cmd
        } else {
            cmd
        }
    } else {
        cmd
    };

    Ok(cmd)
}
