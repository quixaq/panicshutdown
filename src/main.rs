// SPDX-FileCopyrightText: 2026 Quixaq
// SPDX-License-Identifier: GPL-3.0-or-later

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

use nix::mount::{MntFlags, MsFlags, mount, umount2};
use nix::sys::reboot::{RebootMode, reboot};
use nix::unistd::geteuid;
use nix::unistd::sync;

fn main() -> Result<(), std::io::Error> {
    let euid = geteuid();
    if !euid.is_root() {
        return Err(Error::new(ErrorKind::PermissionDenied, "root required"));
    }

    sync();

    let file = File::open("/proc/mounts")?;
    let reader = BufReader::new(file);

    let mut mounts: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.split_whitespace().nth(1).unwrap_or("").to_string())
        .filter(|path| path != "/" && path != "/proc" && path != "/sys" && path != "/dev")
        .collect();
    mounts.reverse();

    for path in mounts {
        let _ = umount2(Path::new(&path), MntFlags::MNT_DETACH);
    }

    let _ = mount(
        Some("/"),
        "/",
        None::<&str>,
        MsFlags::MS_REMOUNT | MsFlags::MS_RDONLY,
        None::<&str>,
    );

    let _ = reboot(RebootMode::RB_POWER_OFF);

    Ok(())
}
