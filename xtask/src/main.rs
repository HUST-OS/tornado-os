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
    BuildUserAppError,
    KernelObjcopyError,
    SharedSchedulerObjcopyError,
    UserAppObjcopyError,
    QemuExecuteError,
}

fn main() -> Result {
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
    let mut xtask = Xtask::debug();
    if let Some(matches) = matches.subcommand_matches("build") {
        if matches.is_present("release") {
            xtask.set_release();
        }
        xtask.build_kernel()?;
        xtask.build_shared_scheduler()?;
        xtask.build_all_user_app()?;
        
    } else if let Some(matches) = matches.subcommand_matches("qemu") {
        let app = matches.args.get("user").unwrap();
        xtask.build_kernel()?;
        xtask.build_shared_scheduler()?;
        xtask.build_user_app(app.vals[0].to_str().unwrap())?;
        xtask.kernel_binary()?;
        xtask.shared_scheduler_binary()?;
        xtask.user_app_binary(app.vals[0].to_str().unwrap())?;
        xtask.execute_qemu(app.vals[0].to_str().unwrap(), 1)?;
    } else if let Some(matches) = matches.subcommand_matches("asm") {
        // todo
    } else if let Some(matches) = matches.subcommand_matches("debug") {

    } else if let Some(matches) = matches.subcommand_matches("gdb") {
        // todo
    } else {
        // todo
    }
    Ok(())
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
        cargo.current_dir(self.root.join("tornado-user"));
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
    /// 编译所有用户程序
    fn build_all_user_app(&self) -> Result {
        let mut cargo = Command::new(&self.cargo);
        cargo.current_dir(self.root.join("tornado-user"));
        cargo.arg("build");
        if matches!(self.mode, CompileMode::Release) {
            cargo.arg("--release");
        }
        cargo.args(&["--target", self.target]);
        cargo.arg("--bins");
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
    /// 生成内核二进制文件
    fn kernel_binary(&self) -> Result {
        // objcopy := "rust-objcopy --binary-architecture=riscv64"
        // @{{objcopy}} {{kernel-elf}} --strip-all -O binary {{kernel-bin}}
        let mut objcopy = Command::new(&self.objcopy);
        objcopy.current_dir(self.target_dir())
            .arg("tornado-kernel")
            .args(&["--binary-architecture=riscv64", "--strip-all"])
            .args(&["-O", "binary", "tornado-kernel.bin"]);
        if let Ok(status) = objcopy.status() {
            if status.success() {
                return Ok(())
            } else {
                return Err(XTaskError::KernelObjcopyError)
            }
        } else {
            return Err(XTaskError::CommandNotFound)
        }
    }
    /// 生成共享调度器二进制文件
    fn shared_scheduler_binary(&self) -> Result {
        // objcopy := "rust-objcopy --binary-architecture=riscv64"
        // @{{objcopy}} {{shared-elf}} --strip-all -O binary {{shared-bin}}
        let mut objcopy = Command::new(&self.objcopy);
        objcopy.current_dir(self.target_dir())
            .arg("shared-scheduler")
            .args(&["--binary-architecture=riscv64", "--strip-all"])
            .args(&["-O", "binary", "shared-scheduler.bin"]);
        if let Ok(status) = objcopy.status() {
            if status.success() {
                return Ok(())
            } else {
                return Err(XTaskError::SharedSchedulerObjcopyError)
            }
        } else {
            return Err(XTaskError::CommandNotFound)
        }
    }
    /// 生成用户程序二进制文件
    fn user_app_binary<APP: AsRef<str>>(&self, app: APP) -> Result {
        // objcopy := "rust-objcopy --binary-architecture=riscv64"
        // @{{objcopy}} {{build-path}}/{{app}} --strip-all -O binary {{build-path}}/{{app}}.bin
        let mut objcopy = Command::new(&self.objcopy);
        objcopy.current_dir(self.target_dir())
            .arg(app.as_ref())
            .args(&["--binary-architecture=riscv64", "--strip-all"])
            .args(&["-O", "binary", format!("{}.bin", app.as_ref()).as_str()]);
        if let Ok(status) = objcopy.status() {
            if status.success() {
                return Ok(())
            } else {
                return Err(XTaskError::UserAppObjcopyError)
            }
        } else {
            return Err(XTaskError::CommandNotFound)
        }
    }
    /// 运行 qemu
    fn execute_qemu<APP: AsRef<str>>(&self, app: APP, threads: u32) -> Result {
        /* @qemu-system-riscv64 \
                -machine virt \
                -nographic \
                -bios none \
                -device loader,file={{bootloader-bin}},addr=0x80000000 \
                -device loader,file={{kernel-bin}},addr=0x80200000 \
                -device loader,file={{shared-bin}},addr=0x86000000 \
                -device loader,file={{app-path}}{{app}}.bin,addr=0x87000000 \
                -smp threads={{threads}} \ */
        
        let mut qemu = Command::new(&self.qemu);
        qemu.current_dir(self.target_dir());
        qemu.args(&["-machine", "virt"]);
        // qemu.args(&["-bios", "none"]);
        // qemu.args(&["-device", "loader,file=../../../SBI/rustsbi-qemu.bin,addr=0x80000000"]); // todo: 这里的地址需要可配置
        // qemu.args(&["-device", "loader,file=tornado-kernel.bin,addr=0x80200000"]);
        qemu.args(&["-bios", "../../../SBI/rustsbi-qemu.bin"]);
        qemu.args(&["-kernel", "tornado-kernel.bin"]);
        qemu.args(&["-device", "loader,file=shared-scheduler.bin,addr=0x86000000"]);  // todo: 这里的地址需要可配置
        qemu.args(&["-device", format!("loader,file={}.bin,addr=0x87000000", app.as_ref()).as_str()]);
        qemu.args(&["-smp", format!("threads={}", &threads).as_str()]);
        qemu.arg("-nographic");
        
        if let Ok(status) = qemu.status() {
            if status.success() {
                return Ok(())
            } else {
                return Err(XTaskError::QemuExecuteError)
            }
        } else {
            return Err(XTaskError::CommandNotFound)
        }
    }
}