use nix::mount::{mount, MsFlags};

pub fn mount_pseudo_filesystems() -> nix::Result<()> {
    // Mounting procfs
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        None as Option<&str>,
    )?;

    // Mounting sys
    mount(
        Some("sys"),
        "/sys",
        Some("sysfs"),
        MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        None as Option<&str>,
    )?;

    mount(
        Some("run"),
        "/run",
        Some("tmpfs"),
        MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        Some("mode=0755"),
    )?;

    mount(
        Some("dev"),
        "/dev",
        Some("devtmpfs"),
        MsFlags::MS_NOSUID,
        Some("mode=0755"),
    )?;

    Ok(())
}
