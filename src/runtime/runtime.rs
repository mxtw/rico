use std::ffi::CString;
use std::io::Write;
use std::{fs, process};

use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::chroot;
use nix::unistd::{execve, fork, getgid, getpid, getuid, ForkResult, Gid, Pid, Uid};

fn write_file(path: &str, content: &str) {
    println!("{}", path);

    let mut file = fs::OpenOptions::new().write(true).open(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn set_uid_map(pid: Pid, uid: Uid, target_uid: Uid) {
    let path = format!("/proc/{}/uid_map", pid);
    let uid_map = format!("{} {} 1\n", target_uid, uid);
    println!("{}", uid_map);

    write_file(&path, uid_map.to_string().as_str());
}

fn set_gid_map(pid: Pid, gid: Gid, target_gid: Gid) {
    let path = format!("/proc/{}/gid_map", pid);
    let gid_map = format!("{} {} 1\n", target_gid, gid);

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

    // get *ids before unsharing
    let uid = getuid();
    let gid = getgid();

    unshare(flags).unwrap();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("parent pid: {} (child pid: {})", getpid(), child);
            println!("waiting for child {} to exit...", child);

            setgroups(child);

            // TODO: allow setting target *ids per flag
            set_uid_map(child, uid, Into::into(0));
            set_gid_map(child, gid, Into::into(0));

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

            // TODO: make this configurable
            // using
            // https://dl-cdn.alpinelinux.org/alpine/v3.21/releases/x86_64/alpine-minirootfs-3.21.3-x86_64.tar.gz
            // for testing
            chroot("./alpine").unwrap();

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
