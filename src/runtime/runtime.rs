use std::ffi::CString;
use std::process;

use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execve, fork, getpid, ForkResult};

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
