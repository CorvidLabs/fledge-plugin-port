use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "fledge-port",
    version,
    about = "Show, kill, or list processes bound to TCP ports"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Sub>,

    /// Shorthand: `fledge port 3000` is the same as `fledge port show 3000`
    port: Option<u16>,
}

#[derive(clap::Subcommand)]
enum Sub {
    /// Show what's bound to a port
    Show { port: u16 },
    /// Kill the process bound to a port (SIGTERM, then SIGKILL on retry)
    Kill {
        port: u16,
        /// Send SIGKILL immediately instead of trying SIGTERM first
        #[arg(short, long)]
        force: bool,
    },
    /// List all currently bound ports
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match (cli.command, cli.port) {
        (Some(Sub::Show { port }), _) => show(port),
        (Some(Sub::Kill { port, force }), _) => kill(port, force),
        (Some(Sub::List), _) => list(),
        (None, Some(port)) => show(port),
        (None, None) => list(),
    }
}

#[derive(Debug, Clone)]
struct Bound {
    pid: u32,
    command: String,
    user: String,
    port: u16,
}

fn show(port: u16) -> Result<()> {
    let entries = scan(Some(port))?;
    if entries.is_empty() {
        println!("Nothing bound to port {port}.");
        return Ok(());
    }
    print_table(&entries);
    Ok(())
}

fn list() -> Result<()> {
    let entries = scan(None)?;
    if entries.is_empty() {
        println!("No bound TCP ports detected.");
        return Ok(());
    }
    print_table(&entries);
    Ok(())
}

fn kill(port: u16, force: bool) -> Result<()> {
    let entries = scan(Some(port))?;
    if entries.is_empty() {
        println!("Nothing bound to port {port}.");
        return Ok(());
    }

    for entry in &entries {
        let signal = if force { "KILL" } else { "TERM" };
        println!(
            "Sending SIG{signal} to PID {} ({}, port {})",
            entry.pid, entry.command, entry.port
        );
        kill_pid(entry.pid, force)?;
    }
    Ok(())
}

fn print_table(entries: &[Bound]) {
    let port_w = entries
        .iter()
        .map(|e| e.port.to_string().len())
        .max()
        .unwrap_or(4)
        .max(4);
    let pid_w = entries
        .iter()
        .map(|e| e.pid.to_string().len())
        .max()
        .unwrap_or(5)
        .max(3);
    let cmd_w = entries
        .iter()
        .map(|e| e.command.len())
        .max()
        .unwrap_or(8)
        .max(7);

    println!(
        "  {:<port_w$}  {:<pid_w$}  {:<cmd_w$}  USER",
        "PORT",
        "PID",
        "COMMAND",
        port_w = port_w,
        pid_w = pid_w,
        cmd_w = cmd_w
    );
    for e in entries {
        println!(
            "  {:<port_w$}  {:<pid_w$}  {:<cmd_w$}  {}",
            e.port,
            e.pid,
            e.command,
            e.user,
            port_w = port_w,
            pid_w = pid_w,
            cmd_w = cmd_w
        );
    }
}

#[cfg(unix)]
fn scan(port_filter: Option<u16>) -> Result<Vec<Bound>> {
    let mut args = vec![
        "-iTCP".to_string(),
        "-sTCP:LISTEN".to_string(),
        "-P".to_string(),
        "-n".to_string(),
        "-F".to_string(),
        "pcuLn".to_string(),
    ];
    if let Some(p) = port_filter {
        args = vec![
            format!("-iTCP:{p}"),
            "-sTCP:LISTEN".to_string(),
            "-P".to_string(),
            "-n".to_string(),
            "-F".to_string(),
            "pcuLn".to_string(),
        ];
    }

    let output = Command::new("lsof")
        .args(&args)
        .output()
        .context("running lsof — is it installed?")?;

    if !output.status.success() && output.stdout.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            return Err(anyhow!("lsof failed: {}", stderr.trim()));
        }
        return Ok(vec![]);
    }

    Ok(parse_lsof(&String::from_utf8_lossy(&output.stdout)))
}

#[cfg(windows)]
fn scan(port_filter: Option<u16>) -> Result<Vec<Bound>> {
    let output = Command::new("netstat")
        .args(["-ano", "-p", "TCP"])
        .output()
        .context("running netstat")?;
    if !output.status.success() {
        bail!(
            "netstat failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(parse_netstat(
        &String::from_utf8_lossy(&output.stdout),
        port_filter,
    ))
}

/// Parse `lsof -F pcuLn` output. Each record is a block of `\n`-separated
/// fields, prefixed by a single character: `p`=pid, `c`=command, `u`=uid,
/// `L`=login name, `n`=name (host:port). A new `p` line starts a new record.
fn parse_lsof(input: &str) -> Vec<Bound> {
    let mut out = Vec::new();
    let mut pid: Option<u32> = None;
    let mut command = String::new();
    let mut user = String::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (tag, rest) = line.split_at(1);
        match tag {
            "p" => {
                pid = rest.parse().ok();
                command.clear();
                user.clear();
            }
            "c" => command = rest.to_string(),
            "L" => user = rest.to_string(),
            "u" if user.is_empty() => user = rest.to_string(),
            "n" => {
                let port = rest
                    .rsplit_once(':')
                    .and_then(|(_, p)| p.split_whitespace().next())
                    .and_then(|p| p.parse::<u16>().ok());
                if let (Some(pid), Some(port)) = (pid, port) {
                    out.push(Bound {
                        pid,
                        command: command.clone(),
                        user: user.clone(),
                        port,
                    });
                }
            }
            _ => {}
        }
    }
    out
}

#[cfg(windows)]
fn parse_netstat(input: &str, port_filter: Option<u16>) -> Vec<Bound> {
    let mut out = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("TCP") {
            continue;
        }
        let cols: Vec<&str> = trimmed.split_whitespace().collect();
        if cols.len() < 5 {
            continue;
        }
        let local = cols[1];
        if !cols.get(3).map(|s| *s == "LISTENING").unwrap_or(false) {
            continue;
        }
        let port = local
            .rsplit_once(':')
            .and_then(|(_, p)| p.parse::<u16>().ok());
        let pid = cols[4].parse::<u32>().ok();
        if let (Some(port), Some(pid)) = (port, pid) {
            if port_filter.map_or(true, |p| p == port) {
                out.push(Bound {
                    pid,
                    command: String::new(),
                    user: String::new(),
                    port,
                });
            }
        }
    }
    out
}

#[cfg(unix)]
fn kill_pid(pid: u32, force: bool) -> Result<()> {
    let signal = if force { "KILL" } else { "TERM" };
    let status = Command::new("kill")
        .args([format!("-{signal}").as_str(), pid.to_string().as_str()])
        .status()
        .context("running kill")?;
    if !status.success() {
        bail!("kill -{signal} {pid} failed");
    }
    Ok(())
}

#[cfg(windows)]
fn kill_pid(pid: u32, _force: bool) -> Result<()> {
    let status = Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/F"])
        .status()
        .context("running taskkill")?;
    if !status.success() {
        bail!("taskkill /PID {pid} failed");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lsof_single_entry() {
        let input = "p1234\ncnode\nLalice\nu501\nn*:3000\n";
        let entries = parse_lsof(input);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].pid, 1234);
        assert_eq!(entries[0].command, "node");
        assert_eq!(entries[0].user, "alice");
        assert_eq!(entries[0].port, 3000);
    }

    #[test]
    fn parse_lsof_ipv6_address() {
        let input = "p42\nccaddy\nLroot\nn[::1]:8080\n";
        let entries = parse_lsof(input);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].port, 8080);
    }

    #[test]
    fn parse_lsof_multiple_entries() {
        let input = "p1\ncfoo\nLa\nn*:80\np2\ncbar\nLb\nn*:443\n";
        let entries = parse_lsof(input);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].port, 80);
        assert_eq!(entries[0].command, "foo");
        assert_eq!(entries[1].port, 443);
        assert_eq!(entries[1].command, "bar");
    }

    #[test]
    fn parse_lsof_falls_back_to_uid_when_no_login() {
        let input = "p99\ncworker\nu1001\nn*:9000\n";
        let entries = parse_lsof(input);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].user, "1001");
    }

    #[test]
    fn parse_lsof_skips_records_without_port() {
        let input = "p1\ncfoo\nLa\nn*:notaport\n";
        let entries = parse_lsof(input);
        assert!(entries.is_empty());
    }
}
