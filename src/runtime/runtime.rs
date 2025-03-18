use std::ffi::CString;
use std::io::Write;
use std::{fs, process};

use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execve, fork, getpid, ForkResult, Pid};

fn write_file(path: &str, content: &str) {
    println!("{}", path);

    let mut file = fs::OpenOptions::new().write(true).open(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn set_uid_map(pid: Pid) {
    let path = format!("/proc/{}/uid_map", pid);
    let uid_map = format!("0 {} 1\n", 1000);
    println!("{}", uid_map);

    write_file(&path, uid_map.to_string().as_str());
}
fn set_gid_map(pid: Pid) {
    let path = format!("/proc/{}/gid_map", pid);
    let gid_map = format!("0 {} 1\n", 100);

    write_file(&path, &gid_map);
}
fn setgroups(pid: Pid) {
    write_file(&format!("/proc/{}/setgroups", pid), &"deny\n");
}

pub fn run_process(command: CString, args: Vec<CString>) {
    let flags = CloneFlags::CLONE_NEWPID
        | CloneFlags::CLONE_NEWUTS
        | CloneFlags::CLONE_NEWNS
        | CloneFlags::CLONE_NEWUSER;

    unshare(flags).unwrap();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("parent pid: {} (child pid: {})", getpid(), child);
            println!("waiting for child {} to exit...", child);

            setgroups(child);
            set_uid_map(child);
            set_gid_map(child);

            match waitpid(child, None) {
                Ok(WaitStatus::Exited(_, status)) => {
                    println!("child exited with status: {}", status);
                }
                Ok(_) => eprintln!("child did not exit as expected"),
                Err(err) => eprintln!("error waiting for child: {}", err),
            }
        }
        Ok(ForkResult::Child) => {
            println!("child pid: {}", getpid());

            let env: Vec<CString> = Vec::new();

            let _ = execve(&command, &args, &env);

            // this should never be reached, as execve replaces the child
            eprintln!("execve failed");
            process::exit(1);
        }
        Err(err) => {
            eprintln!("fork has failed: {}", err);
            process::exit(1);
        }
    }
}
