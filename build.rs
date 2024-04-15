/*
 * Copyright (c) 2024, MLC 'Strawmelonjuice' Bloeiman
 *
 * Licensed under the BSD 3-Clause License. See the LICENSE file for more info.
 */

// Javascript runtimes:
//     NodeJS:
#[cfg(windows)]
const NODEJSR: &str = "node.exe";
#[cfg(not(windows))]
const NODEJSR: &str = "node";
//     Bun:
#[cfg(windows)]
const BUNJSR: &str = "bun.exe";
#[cfg(not(windows))]
const BUNJSR: &str = "bun";

// Javascript package managers:
//     NPM:
#[cfg(windows)]
const NODE_NPM: &str = "npm.cmd";
#[cfg(not(windows))]
const NODE_NPM: &str = "npm";
//     Bun:
#[cfg(windows)]
const BUN_NPM: &str = "bun.exe";
#[cfg(not(windows))]
const BUN_NPM: &str = "bun";

fn main() {
    match std::process::Command::new(BUN_NPM).arg("install").output() {
        Ok(t) => {
            if t.status.code().unwrap() != 0 {
                panic!(
                    "Could not install dependencies: {}",
                    std::str::from_utf8(&t.stderr).unwrap_or("Unknown error.")
                )
            }
        }
        Err(_err) => match std::process::Command::new(NODE_NPM).arg("install").output() {
            Ok(t) => {
                if t.status.code().unwrap() != 0 {
                    panic!(
                        "Could not install dependencies: {}",
                        std::str::from_utf8(&t.stderr).unwrap_or("Unknown error.")
                    )
                }
            }
            Err(_err) => {
                panic!("No supported (Node.JS or Bun) Javascript runtimes found on path! Or could not install dependencies.");
            }
        },
    };
    match std::process::Command::new(BUNJSR)
        .arg("--bun")
        .arg("run")
        .arg("build:deps")
        .output()
    {
        Ok(t) => {
            if t.status.code().unwrap() != 0 {
                panic!(
                    "Could not generate assets: {}",
                    std::str::from_utf8(&t.stderr).unwrap_or("Unknown error.")
                )
            }
        }
        Err(_err) => match std::process::Command::new(NODEJSR)
            .arg("run")
            .arg("build:deps")
            .output()
        {
            Ok(t) => {
                if t.status.code().unwrap() != 0 {
                    panic!(
                        "Could not generate assets: {}",
                        std::str::from_utf8(&t.stderr).unwrap_or("Unknown error.")
                    )
                }
            }
            Err(_err) => {
                panic!("No supported (Node.JS or Bun) Javascript runtimes found on path! Or could not generate necessary files.");
            }
        },
    };
}