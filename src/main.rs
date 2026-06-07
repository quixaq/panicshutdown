// SPDX-FileCopyrightText: 2026 Quixaq
// SPDX-License-Identifier: GPL-3.0-or-later

use std::fs;
use std::io::{Error, ErrorKind};

use nix::unistd::geteuid;

fn main() -> Result<(), std::io::Error> {
    let euid = geteuid();
    if !euid.is_root() {
        return Err(Error::new(ErrorKind::PermissionDenied, "root required"));
    }

    let path = "/proc/sysrq-trigger";
    for command in &['s', 'u', 'o'] {
        fs::write(path, &[*command as u8])?;
    }

    Ok(())
}
