use nix::sched::{clone, CloneFlags};
use nix::sys::wait::waitpid;
use nix::unistd::{execve, fork, ForkResult};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;

pub fn run(command: &str) -> nix::Result<()> {
    let flags = CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS;

    let command = command.to_string();
    match clone(
        Box::new(|| child_func(&command)),
        &mut [0u8; 1024 * 1024],
        flags,
        None,
    ) {
        Ok(pid) => {
            println!("Started container with PID {}", pid);
            waitpid(pid, None)?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn child_func(command: &str) -> isize {
    if let Err(e) = mount_proc() {
        eprintln!("Failed to mount proc: {}", e);
        return -1;
    }

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            waitpid(child, None).expect("waitpid failed");
            0
        }
        Ok(ForkResult::Child) => {
            let args: Vec<&str> = command.split_whitespace().collect();
            let prog = CString::new(args[0]).unwrap();
            let args: Vec<CString> = args.iter().map(|&s| CString::new(s).unwrap()).collect();

            // Create an empty environment
            let env: Vec<CString> = Vec::new();

            execve(&prog, &args, &env).expect("execve failed");
            unreachable!();
        }
        Err(_) => -1,
    }
}

fn mount_proc() -> nix::Result<()> {
    use nix::mount::{mount, MsFlags};
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOEXEC | MsFlags::MS_RELATIME,
        None::<&str>,
    )
}
