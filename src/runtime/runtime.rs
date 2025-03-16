use std::ffi::CString;
use std::process;

use nix::libc::clone_args;
use nix::sched::{unshare, CloneFlags};
use nix::unistd::{execve, fork, getpid, ForkResult};

pub fn test() {
    println!("test");

    let flags = CloneFlags::CLONE_NEWPID
        | CloneFlags::CLONE_NEWUTS
        | CloneFlags::CLONE_NEWNS
        | CloneFlags::CLONE_NEWUSER;

    unshare(flags).unwrap();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("parent pid: {} (child pid: {})", getpid(), child);
        }
        Ok(ForkResult::Child) => {
            println!("child pid: {}", getpid());

            let cmd = CString::new("/bin/sh").unwrap();
            let args = [cmd.clone(), CString::new("-l").unwrap()];
            let env: Vec<CString> = Vec::new();

            let _ = execve(&cmd, &args, &env);

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
