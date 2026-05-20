use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, path::Path};
use tempfile::{NamedTempFile, TempDir, tempdir};
use walkdir::WalkDir;

pub struct TestProject {
    project_dir: TempDir,
    replace_file: Box<Path>,
}

#[derive(Debug, Deserialize)]
struct TestConfig {
    file: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    test: TestConfig,
}

impl TestProject {
    pub fn new(template: &Path) -> Result<TestProject> {
        let dir = tempdir()?;

        // Copy template to the temp directory.
        for entry in WalkDir::new(template).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let relative_path = path.strip_prefix(template)?;
                let target_path = Path::new(dir.path()).join(relative_path);
                if let Some(parent) = target_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(path, target_path)?;
            }
        }
        let config_str =
            fs::read_to_string(template.join("test.toml")).context("Config read error")?;
        let config: ConfigFile =
            toml::from_str(&config_str).context("Could not parse test config")?;
        let absolute_test_path = dir.path().join(config.test.file);
        Ok(TestProject {
            project_dir: dir,
            replace_file: absolute_test_path.into(),
        })
    }

    pub fn compile(&self, test: &Path, toolchain: &Path) -> Result<NamedTempFile> {
        fs::copy(test, &self.replace_file)?;

        let toolchain_bin = toolchain.join("bin");
        let cargo = toolchain_bin.join("cargo");

        let path_env = env::var_os("PATH").unwrap_or_default();
        let mut paths = env::split_paths(&path_env).collect::<Vec<_>>();
        paths.insert(0, toolchain_bin);
        let path_env = env::join_paths(paths)?;

        let output = Command::new(cargo)
            .arg("build")
            .arg("--release")
            .arg("--target")
            .arg("cdm-none")
            .arg("--target-dir")
            .arg("target")
            .current_dir(&self.project_dir)
            .env("PATH", path_env)
            .output()
            .context("Could not run cargo")?;
        if !output.status.success() {
            return Err(anyhow!(
                "Cargo build failed\nstatus: {:?}\nstderr:\n{}",
                output.status,
                str::from_utf8(&output.stderr).context("Cargo build failed\nstatus: {:?}",)?
            ));
        }
        let dir_path: &Path = self.project_dir.as_ref();
        let bin = fs::read(
            dir_path
                .join("target")
                .join("cdm-none")
                .join("release")
                .join("cdm-test"),
        )
        .context("Could not read image")?;
        let mut image = NamedTempFile::with_suffix(".img")?;
        write!(image, "v2.0 raw\n")?;
        for byte in bin {
            write!(image, "{:02x}\n", byte)?;
        }
        Ok(image)
    }
}
