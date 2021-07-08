use std::{env, ffi::OsStr, path::{Path, PathBuf}, process::{self, Command}};

#[macro_use]
extern crate clap;

const DEFAULT_TARGET: &'static str = "riscv64imac-unknown-none-elf";

type Result<T = ()> = core::result::Result<T, XTaskError>;

#[derive(Debug)]
struct Xtask<'x, S: AsRef<OsStr>> {
    mode: CompileMode,
    root: PathBuf,
    target: &'x str,
    cargo: S,
    qemu: S,
    objdump: S,
    objcopy: S,
    size: S
}

#[derive(Debug)]
enum CompileMode {
    Debug,
    Release
}

#[derive(Debug)]
enum XTaskError {
    CommandNotFound,
    BuildKernelError,
    BuildSharedSchedulerError,
    BuildUserAppError
}

fn main() {
    let matches = clap_app!(xtask =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@subcommand build =>
            (about: "Build project")
            (@arg release: --release "Build artifacts in release mode, with optimizations")
        )
        (@subcommand qemu =>
            (about: "Execute qemu")
            (@arg user: +required "Select user binary to execute")
            (@arg release: --release "Build artifacts in release mode, with optimizations")
        )
        (@subcommand asm =>
            (about: "Dump asm code")
            (@arg bin: +required "Select binary to dump")
            (@arg release: --release "Build artifacts in release mode, with optimizations")
        )
        (@subcommand debug =>
            (about: "Debug with qemu and gdb stub")
            (@arg user: +required "Select user binary to debug")
        )
        (@subcommand gdb =>
            (about: "Run gdb debugger")
        )
    )
    .get_matches();
    eprintln!("{:?}", matches);
    if let Some(matches) = matches.subcommand_matches("build") {
        // todo
    } else if let Some(matches) = matches.subcommand_matches("qemu") {
        // todo
    } else if let Some(matches) = matches.subcommand_matches("asm") {
        // todo
    } else if let Some(matches) = matches.subcommand_matches("debug") {

    } else if let Some(matches) = matches.subcommand_matches("gdb") {
        // todo
    } else {
        // todo
    }
}

impl<'x> Xtask<'x, String> {
    fn debug() -> Self {
        let root = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .unwrap()
            .to_path_buf();
        let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        Self {
            mode: CompileMode::Debug,
            root,
            target: DEFAULT_TARGET,
            cargo,
            qemu: String::from("qemu-system-riscv64"),
            objcopy: String::from("rust-objcopy"), // todo: 检查系统中有哪些 objcopy
            objdump: String::from("rust-objdump"), // todo: 检查系统中有哪些 objdump
            size: String::from("rust-size"), // todo: 检查系统中有哪些 size
        }
    }
    fn release() -> Self {
        let root = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .unwrap()
            .to_path_buf();
        let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        Self {
            mode: CompileMode::Release,
            root,
            target: DEFAULT_TARGET,
            cargo,
            qemu: String::from("qemu-system-riscv64"),
            objcopy: String::from("rust-objcopy"), // todo: 检查系统中有哪些 objcopy
            objdump: String::from("rust-objdump"), // todo: 检查系统中有哪些 objdump
            size: String::from("rust-size"), // todo: 检查系统中有哪些 size
        }
    }
}

impl<'x, S: AsRef<OsStr>> Xtask<'x, S> {
    fn set_debug(&mut self) {
        self.mode = CompileMode::Debug;
    }
    fn set_release(&mut self) {
        self.mode = CompileMode::Release;
    }
    fn target_dir(&self) -> PathBuf {
        let mut p = self.root.join("target").join(self.target);
        p = match self.mode {
            CompileMode::Debug => p.join("debug"),
            CompileMode::Release => p.join("release")
        };
        p
    }
    /// 编译内核
    fn build_kernel(&self) -> Result {
        let mut cargo = Command::new(&self.cargo);
        cargo.current_dir(self.root.join("tornado-kernel"));
        cargo.arg("build");
        if matches!(self.mode, CompileMode::Release) {
            cargo.arg("--release");
        }
        cargo.args(&["--target", self.target]);
        if let Ok(status) = cargo.status() {
            if status.success() {
                return Ok(())
            } else {
                return Err(XTaskError::BuildKernelError)
            }
        } else {
            return Err(XTaskError::CommandNotFound)
        }
    }
    /// 编译共享调度器
    fn build_shared_scheduler(&self) -> Result {
        let mut cargo = Command::new(&self.cargo);
        cargo.current_dir(self.root.join("shared-scheduler"));
        cargo.arg("build");
        if matches!(self.mode, CompileMode::Release) {
            cargo.arg("--release");
        }
        cargo.args(&["--target", self.target]);
        if let Ok(status) = cargo.status() {
            if status.success() {
                return Ok(())
            } else {
                return Err(XTaskError::BuildSharedSchedulerError)
            }
        } else {
            return Err(XTaskError::CommandNotFound)
        }
    }
    /// 编译用户程序
    fn build_user_app<APP: AsRef<str>>(&self, app: APP) -> Result {
        let mut cargo = Command::new(&self.cargo);
        cargo.current_dir(self.root.join("shared-scheduler"));
        cargo.arg("build");
        if matches!(self.mode, CompileMode::Release) {
            cargo.arg("--release");
        }
        cargo.args(&["--target", self.target]);
        cargo.args(&["--bin", app.as_ref()]);
        if let Ok(status) = cargo.status() {
            if status.success() {
                return Ok(())
            } else {
                return Err(XTaskError::BuildUserAppError)
            }
        } else {
            return Err(XTaskError::CommandNotFound)
        }
    }
    
}