use libcnb::data::buildpack::BuildpackApi;
use serde::Deserialize;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Command, ExitStatus, Stdio},
};

const DEFAULT_SHELL: &str = "#!/bin/sh";

#[derive(Deserialize)]
pub struct Script {
    // Ignored since can't dynamically change the Buildpack API while running
    #[allow(dead_code)]
    api: BuildpackApi,
    #[serde(default = "default_shell")]
    shell: String,
    inline: String,
}

impl Script {
    /// Writes inline script to file and executes it returning the exit code
    pub fn run(&self, path: impl AsRef<Path>) -> anyhow::Result<ExitStatus> {
        fs::write(
            &path,
            format!(
                r#"{}
{}
"#,
                self.shell, self.inline
            ),
        )?;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;

        let mut cmd = Command::new(path.as_ref().as_os_str())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        Ok(cmd.wait()?)
    }
}

fn default_shell() -> String {
    String::from(DEFAULT_SHELL)
}
