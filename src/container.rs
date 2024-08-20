use nix::sched::{clone, CloneFlags};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execvp, fork, ForkResult};
use std::ffi::CString;

pub fn run(command: Vec<String>) -> nix::Result<()> {
    let flags = CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS;

    match clone(
        Box::new(move || child_func(&command)),
        &mut [0u8; 1024 * 1024],
        flags,
        None,
    ) {
        Ok(pid) => {
            println!("Started container with PID {}", pid);
            match waitpid(pid, None) {
                Ok(WaitStatus::Exited(_, status)) => {
                    println!("Container exited with status: {}", status);
                    Ok(())
                }
                Ok(status) => {
                    println!("Container exited with unexpected status: {:?}", status);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error waiting for container: {:?}", e);
                    Err(e)
                }
            }
        }
        Err(err) => Err(err),
    }
}

fn child_func(command: &[String]) -> isize {
    println!("Child function started");
    if let Err(e) = mount_proc() {
        eprintln!("Failed to mount proc: {:?}", e);
        return -1;
    }

    println!("Proc mounted successfully");

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("Forked child with PID: {:?}", child);
            match waitpid(child, None) {
                Ok(status) => {
                    println!("Child exited with status: {:?}", status);
                    0
                }
                Err(e) => {
                    eprintln!("Error waiting for child: {:?}", e);
                    -1
                }
            }
        }
        Ok(ForkResult::Child) => {
            println!("In child process, preparing to exec");
            let prog = CString::new(command[0].clone()).unwrap();
            let args: Vec<CString> = command
                .iter()
                .map(|s| CString::new(s.as_str()).unwrap())
                .collect();

            println!("Executing command: {:?}", command);
            match execvp(&prog, &args) {
                Ok(_) => unreachable!(),
                Err(e) => {
                    eprintln!("execvp failed: {:?}", e);
                    -1
                }
            }
        }
        Err(e) => {
            eprintln!("Fork failed: {:?}", e);
            -1
        }
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
