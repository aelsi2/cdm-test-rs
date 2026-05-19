mod cocoemu;
mod compile;

use anyhow::Result;
use clap::Parser;
use compile::TestProject;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::cocoemu::Cocoemu;

/// Runner for CDM-16 Rust compiler compliance tests.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RunnerArgs {
    /// Path to the sysroot of the toolchain to compile the tests with.
    #[arg(long, short = 'c', required = true)]
    toolchain: PathBuf,

    /// Path to the test template project.
    #[arg(long, short = 't', required = true)]
    template: PathBuf,

    /// Test files to compile and run.
    /// If a directory is specified, then it will be scanned recursively
    /// and all .rs files will be compiled and executed.
    #[arg(required = true)]
    tests: Vec<PathBuf>,
}

impl RunnerArgs {
    fn collect_tests(&self, mut visitor: impl FnMut(&Path)) -> Result<()> {
        for path in &self.tests {
            for maybe_entry in WalkDir::new(path).into_iter() {
                let entry = maybe_entry?;
                let path = entry.path();
                if path.is_file()
                    && let Some(extension) = path.extension()
                    && extension == "rs"
                {
                    visitor(path);
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = RunnerArgs::parse();

    let mut tests: Vec<Box<Path>> = vec![];
    args.collect_tests(|file| {
        tests.push(file.into());
    })?;

    println!("Found {} tests.", tests.len());

    let project = TestProject::new(&args.template)?;
    let mut cocoemu = Cocoemu::new(7001)?;

    let mut assertion_errors: usize = 0;
    let mut compile_errors: usize = 0;
    for test in &tests {
        println!("Compiling {}", test.display());
        let image = match project.compile(test, &args.toolchain) {
            Ok(image) => image,
            Err(error) => {
                println!("{}", error);
                println!("Failed");
                compile_errors += 1;
                continue;
            }
        };

        println!("Running {}", test.display());
        if !cocoemu.run(image.path())? {
            println!("Running {}", test.display());
            println!("Failed due to assertion error");
            assertion_errors += 1;
            continue;
        }

        println!("Passed");
    }

    let total = tests.len();
    let passed = total - assertion_errors - compile_errors;
    println!(
        "Testing complete.\n  Passed: {}\n  Compilation errors: {}\n  Assertion errors: {}\n  Total: {}",
        passed,
        compile_errors,
        assertion_errors,
        total
    );

    Ok(())
}
