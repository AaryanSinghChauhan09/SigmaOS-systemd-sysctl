// Integration test for SigmaOS-systemd-sysctl
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use systemd_sysctl as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
