use nix::sched::{unshare, CloneFlags};

pub fn test() {
    println!("test");

    let mut flags = CloneFlags::empty();

    flags.insert(CloneFlags::CLONE_NEWPID);
    flags.insert(CloneFlags::CLONE_NEWUTS);
    flags.insert(CloneFlags::CLONE_NEWNS);
    flags.insert(CloneFlags::CLONE_NEWUSER);

    unshare(flags).unwrap();

    // wait so i can check if the namespace exists in lsns
    std::thread::sleep(std::time::Duration::from_secs(5));
}
