use std::process::{Command as StdCommand, ExitStatus};
use std::thread::sleep;
use std::time::Duration;

fn run_live_check(param_value: &str) -> ExitStatus {
    // Start registry live check command as a background process
    let mut live_check_cmd = StdCommand::new("weaver")
        .args([
            "registry",
            "live-check",
            "-r",
            "model",
            "--inactivity-timeout",
            "3",
        ])
        .spawn()
        .expect("Failed to start registry live check process");

    // Allow live check command to initialize
    sleep(Duration::from_secs(2));

    // Run weaver-example command
    let example_cmd = StdCommand::new(env!("CARGO_BIN_EXE_weaver-example"))
        .arg("TESTING")
        .arg(param_value)
        .output()
        .expect("Failed to execute weaver-example process");

    // Check that weaver-example command was successful
    assert!(
        example_cmd.status.success(),
        "weaver-example command failed: {}",
        String::from_utf8_lossy(&example_cmd.stderr)
    );

    // Wait for live check process to terminate due to inactivity timeout
    live_check_cmd
        .wait()
        .expect("Failed to wait for live check process to terminate")
}

#[test]
fn test_live_check() {
    let status = run_live_check("SHOULD PASS");
    assert!(
        status.success(),
        "Live check command did not exit successfully: {:?}",
        status
    );

    sleep(Duration::from_secs(2));

    let status = run_live_check("123");
    assert!(
        !status.success(),
        "Live check command did not exit with an error: {:?}",
        status
    );
}
