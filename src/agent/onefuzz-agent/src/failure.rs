use anyhow::{Context, Error, Result};
use onefuzz::fs::{onefuzz_logs, onefuzz_root};
use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub fn failure_path(machine_id: Uuid) -> Result<PathBuf> {
    Ok(onefuzz_root()?.join(format!("onefuzz-agent-failure-{}.txt", machine_id)))
}

pub fn save_failure(err: &Error, machine_id: Uuid) -> Result<()> {
    error!("saving failure: {:?}", err);
    let path = failure_path(machine_id)?;
    let message = format!("{:?}", err);
    fs::write(&path, message)
        .with_context(|| format!("unable to write failure log: {}", path.display()))
}

pub fn read_failure(machine_id: Uuid) -> Result<String> {
    let path = failure_path(machine_id)?;
    read_file_lossy(&path)
}

pub fn read_logs() -> Result<String> {
    let log_path = onefuzz_logs()?;

    let mut results = vec![];
    for entry in fs::read_dir(&log_path)
        .with_context(|| format!("unable to read logs directory: {}", log_path.display()))?
    {
        let path = entry.context("unable to get log file context")?.path();
        let content = read_file_lossy(&path).context("unable to read log file")?;
        results.push(path.display().to_string());
        results.push(content);
    }

    Ok(results.join("\n\n"))
}

fn read_file_lossy(path: &Path) -> Result<String> {
    let content =
        fs::read(path).with_context(|| format!("unable to read file: {}", path.display()))?;
    Ok(String::from_utf8_lossy(&content).to_string())
}
