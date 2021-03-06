#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unexpected format in /proc/self/mounts")]
    InvalidProcMounts,

    #[error("Systemd not found in standard locations (machinectl and systemd are necessary)")]
    NoSystemdFound,

    #[error("Systemd is not running")]
    NotRunning,

    #[error("Something went wrong while waiting")]
    WaitFailed,

    #[error("Something went wrong while starting, exitstatus={0}")]
    StartFailed(i32),
}
