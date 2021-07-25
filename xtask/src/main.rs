use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

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
    gdb: S,
    objdump: S,
    objcopy: S,
    size: S,
}

#[derive(Debug)]
enum CompileMode {
    Debug,
    Release,
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
    QemuDebugError,
    GDBError,
    AsmError,
    SizeError,
}

fn main() -> Result {
    let matches = clap_app!(xtask =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@subcommand build =>
            (about: "Build project")
            (@arg platform: +required "Select execute platform")
            (@arg release: --release "Build artifacts in release mode, with optimizations")
        )
        (@subcommand qemu =>
            (about: "Execute qemu")
            (@arg user: +required "Select user binary to execute")
            (@arg release: --release "Build artifacts in release mode, with optimizations")
        )
        (@subcommand asm =>
            (about: "Dump asm code")
            (@arg elf: +required "Select elf to dump")
        )
        (@subcommand size =>
            (about: "Size")
            (@arg elf: +required "Select elf to size")
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
        let platform = matches.args.get("platform").unwrap();
        let platform = platform.vals[0].to_str().unwrap();
        xtask.build_kernel(platform)?;
        xtask.build_shared_scheduler(platform)?;
        xtask.build_all_user_app()?;
    } else if let Some(matches) = matches.subcommand_matches("qemu") {
        let app = matches.args.get("user").unwrap();
        if matches.is_present("release") {
            xtask.set_release();
        }
        xtask.build_kernel("qemu")?;
        xtask.build_shared_scheduler("qemu")?;
        xtask.build_user_app(app.vals[0].to_str().unwrap())?;
        xtask.kernel_binary()?;
        xtask.shared_scheduler_binary()?;
        xtask.user_app_binary(app.vals[0].to_str().unwrap())?;
        xtask.execute_qemu(app.vals[0].to_str().unwrap(), 1)?;
    } else if let Some(matches) = matches.subcommand_matches("asm") {
        let elf = matches.args.get("elf").unwrap().vals[0].to_str().unwrap();
        match elf {
            "kernel" => xtask.kernel_asm()?,
            "shared_scheduler" => xtask.shared_scheduler_asm()?,
            app => xtask.user_app_asm(app)?,
        };
    } else if let Some(matches) = matches.subcommand_matches("size") {
        let elf = matches.args.get("elf").unwrap().vals[0].to_str().unwrap();
        match elf {
            "kernel" => xtask.kernel_size()?,
            "shared_scheduler" => xtask.shared_scheduler_size()?,
            app => xtask.user_app_size(app)?,
        };
    } else if let Some(matches) = matches.subcommand_matches("debug") {
        let app = matches.args.get("user").unwrap();
        xtask.build_kernel("qemu")?;
        xtask.build_shared_scheduler("qemu")?;
        xtask.build_user_app(app.vals[0].to_str().unwrap())?;
        xtask.kernel_binary()?;
        xtask.shared_scheduler_binary()?;
        xtask.user_app_binary(app.vals[0].to_str().unwrap())?;
        xtask.debug_qemu(app.vals[0].to_str().unwrap(), 1)?;
    } else if let Some(_matches) = matches.subcommand_matches("gdb") {
        xtask.gdb()?;
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
        let mut toolchain = Self::available_toolchain();
        let size = toolchain.pop().unwrap();
        let objdump = toolchain.pop().unwrap();
        let objcopy = toolchain.pop().unwrap();
        Self {
            mode: CompileMode::Debug,
            root,
            target: DEFAULT_TARGET,
            cargo,
            qemu: "qemu-system-riscv64.exe".to_string(),
            gdb: "riscv64-unknown-elf-gdb.exe".to_string(), // todo: 检查系统中 riscv gdb 的位置
            objcopy,
            objdump,
            size,
        }
    }
    #[allow(unused)]
    fn release() -> Self {
        let root = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .unwrap()
            .to_path_buf();
        let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let mut toolchain = Self::available_toolchain();
        let size = toolchain.pop().unwrap();
        let objdump = toolchain.pop().unwrap();
        let objcopy = toolchain.pop().unwrap();
        Self {
            mode: CompileMode::Release,
            root,
            target: DEFAULT_TARGET,
            cargo,
            qemu: "qemu-system-riscv64".to_string(),
            gdb: "riscv64-unknown-elf-gdb".to_string(), // todo: 检查系统中 riscv gdb 的位置
            objcopy,
            objdump,
            size,
        }
    }
    fn available_toolchain() -> Vec<String> {
        let mut toolchain = Vec::new();
        match Self::check_tool("objcopy") {
            Some(objcopy) => {
                toolchain.push(objcopy);
            }
            None => {
                eprintln!("objcopy tool not found.");
                std::process::exit(1);
            }
        }
        match Self::check_tool("objdump") {
            Some(objdump) => {
                toolchain.push(objdump);
            }
            None => {
                eprintln!("objdump tool not found.");
                std::process::exit(1);
            }
        }
        match Self::check_tool("size") {
            Some(size) => {
                toolchain.push(size);
            }
            None => {
                eprintln!("size tool not found.");
                std::process::exit(1);
            }
        }
        toolchain
    }
    fn check_tool<S: AsRef<str>>(tool: S) -> Option<String> {
        // 先看系统中有没有 `rust-x` 工具
        if let Ok(status) = Command::new(format!("rust-{}", tool.as_ref()))
            .arg("--version")
            .stdout(Stdio::null())
            .status()
        {
            if status.success() {
                return Some(format!("rust-{}", tool.as_ref()));
            }
        }
        // 再检查系统中有没有 `riscv64-linux-gnu-x` 工具
        if let Ok(status) = Command::new(format!("riscv64-linux-gnu-{}", tool.as_ref()))
            .arg("--version")
            .stdout(Stdio::null())
            .status()
        {
            if status.success() {
                return Some(format!("riscv64-linux-gnu-{}", tool.as_ref()));
            }
        }
        // 最后检查系统中有没有 `riscv64-unknown-elf-x` 工具
        if let Ok(status) = Command::new(format!("riscv64-unknown-elf-{}", tool.as_ref()))
            .arg("--version")
            .stdout(Stdio::null())
            .status()
        {
            if status.success() {
                return Some(format!("riscv64-unknown-elf-{}", tool.as_ref()));
            }
        }
        None
    }
}

impl<'x, S: AsRef<OsStr>> Xtask<'x, S> {
    #[allow(unused)]
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
            CompileMode::Release => p.join("release"),
        };
        p
    }
    /// 编译内核
    fn build_kernel<P: AsRef<OsStr>>(&self, platform: P) -> Result {
        let mut cargo = Command::new(&self.cargo);
        cargo.current_dir(self.root.join("tornado-kernel"));
        cargo.arg("build");
        cargo.args(&["--features", platform.as_ref().to_str().unwrap()]);
        if matches!(self.mode, CompileMode::Release) {
            cargo.arg("--release");
        }
        cargo.args(&["--target", self.target]);
        cargo.env("PLATFORM", platform);
        if let Ok(status) = cargo.status() {
            if status.success() {
                Ok(())
            } else {
                Err(XTaskError::BuildKernelError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
        }
    }
    /// 编译共享调度器
    fn build_shared_scheduler<P: AsRef<OsStr>>(&self, platform: P) -> Result {
        let mut cargo = Command::new(&self.cargo);
        cargo.current_dir(self.root.join("shared-scheduler"));
        cargo.arg("build");
        if matches!(self.mode, CompileMode::Release) {
            cargo.arg("--release");
        }
        cargo.args(&["--target", self.target]);
        cargo.env("PLATFORM", platform);
        if let Ok(status) = cargo.status() {
            if status.success() {
                Ok(())
            } else {
                Err(XTaskError::BuildSharedSchedulerError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
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
                Ok(())
            } else {
                Err(XTaskError::BuildUserAppError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
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
                Ok(())
            } else {
                Err(XTaskError::BuildUserAppError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
        }
    }
    /// 生成内核二进制文件
    fn kernel_binary(&self) -> Result {
        // objcopy := "rust-objcopy --binary-architecture=riscv64"
        // @{{objcopy}} {{kernel-elf}} --strip-all -O binary {{kernel-bin}}
        let mut objcopy = Command::new(&self.objcopy);
        objcopy
            .current_dir(self.target_dir())
            .arg("tornado-kernel")
            .args(&["--binary-architecture=riscv64", "--strip-all"])
            .args(&["-O", "binary", "tornado-kernel.bin"]);
        if let Ok(status) = objcopy.status() {
            if status.success() {
                Ok(())
            } else {
                Err(XTaskError::KernelObjcopyError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
        }
    }
    /// 生成共享调度器二进制文件
    fn shared_scheduler_binary(&self) -> Result {
        // objcopy := "rust-objcopy --binary-architecture=riscv64"
        // @{{objcopy}} {{shared-elf}} --strip-all -O binary {{shared-bin}}
        let mut objcopy = Command::new(&self.objcopy);
        objcopy
            .current_dir(self.target_dir())
            .arg("shared-scheduler")
            .args(&["--binary-architecture=riscv64", "--strip-all"])
            .args(&["-O", "binary", "shared-scheduler.bin"]);
        if let Ok(status) = objcopy.status() {
            if status.success() {
                Ok(())
            } else {
                Err(XTaskError::SharedSchedulerObjcopyError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
        }
    }
    /// 生成用户程序二进制文件
    fn user_app_binary<APP: AsRef<str>>(&self, app: APP) -> Result {
        // objcopy := "rust-objcopy --binary-architecture=riscv64"
        // @{{objcopy}} {{build-path}}/{{app}} --strip-all -O binary {{build-path}}/{{app}}.bin
        let mut objcopy = Command::new(&self.objcopy);
        objcopy
            .current_dir(self.target_dir())
            .arg(app.as_ref())
            .args(&["--binary-architecture=riscv64", "--strip-all"])
            .args(&["-O", "binary", format!("{}.bin", app.as_ref()).as_str()]);
        if let Ok(status) = objcopy.status() {
            if status.success() {
                Ok(())
            } else {
                Err(XTaskError::UserAppObjcopyError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
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
        qemu.arg("-nographic");
        // qemu.args(&["-bios", "none"]);
        // qemu.args(&["-device", "loader,file=../../../SBI/rustsbi-qemu.bin,addr=0x80000000"]); // todo: 这里的地址需要可配置
        // qemu.args(&["-device", "loader,file=tornado-kernel.bin,addr=0x80200000"]);
        qemu.args(&["-bios", "../../../SBI/rustsbi-qemu.bin"]);
        qemu.args(&["-kernel", "tornado-kernel.bin"]);
        qemu.args(&[
            "-device",
            "loader,file=shared-scheduler.bin,addr=0x86000000",
        ]); // todo: 这里的地址需要可配置
        qemu.args(&[
            "-device",
            format!("loader,file={}.bin,addr=0x87000000", app.as_ref()).as_str(),
        ]);
        qemu.args(&["-smp", format!("threads={}", &threads).as_str()]);

        if let Ok(status) = qemu.status() {
            if status.success() {
                Ok(())
            } else {
                Err(XTaskError::QemuExecuteError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
        }
    }
    /// 反汇编
    fn asm<ELF: AsRef<str>>(&self, elf: ELF) -> Result {
        let mut dump = Command::new(&self.objdump);
        dump.current_dir(self.target_dir());
        dump.args(&["-D", elf.as_ref()]);
        if let Ok(status) = dump.status() {
            if status.success() {
                Ok(())
            } else {
                Err(XTaskError::AsmError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
        }
    }
    /// 内核反汇编
    fn kernel_asm(&self) -> Result {
        // @{{objdump}} -D {{kernel-elf}} | less
        self.asm("tornado-kernel")
    }
    /// 共享调度器反汇编
    fn shared_scheduler_asm(&self) -> Result {
        // @{{objdump}} -D {{shared-elf}} | less
        self.asm("shared-scheduler")
    }
    /// 用户程序反汇编
    fn user_app_asm<APP: AsRef<str>>(&self, app: APP) -> Result {
        // @{{objdump}} -D {{build-path}}/{{app}} | less
        self.asm(app)
    }
    /// size
    fn size<ELF: AsRef<str>>(&self, elf: ELF) -> Result {
        let mut size = Command::new(&self.size);
        size.current_dir(self.target_dir());
        size.args(&["-A", "-x", elf.as_ref()]);
        if let Ok(status) = size.status() {
            if status.success() {
                Ok(())
            } else {
                Err(XTaskError::SizeError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
        }
    }
    fn kernel_size(&self) -> Result {
        self.size("tornado-kernel")
    }
    fn shared_scheduler_size(&self) -> Result {
        self.size("shared-scheduler")
    }
    fn user_app_size<APP: AsRef<str>>(&self, app: APP) -> Result {
        self.size(app)
    }
    fn debug_qemu<APP: AsRef<str>>(&self, app: APP, threads: u32) -> Result {
        /* @qemu-system-riscv64 \
        -machine virt \
        -nographic \
        -bios none \
        -device loader,file={{bootloader-bin}},addr=0x80000000 \
        -device loader,file={{kernel-bin}},addr=0x80200000 \
        -device loader,file={{shared-bin}},addr=0x86000000 \
        -device loader,file={{app-path}}{{app}}.bin,addr=0x87000000 \
        -smp threads={{threads}} \
        -gdb tcp::1234 -S \ */

        let mut qemu = Command::new(&self.qemu);
        qemu.current_dir(self.target_dir());
        qemu.args(&["-machine", "virt"]);
        qemu.arg("-nographic");

        qemu.args(&["-bios", "../../../SBI/rustsbi-qemu.bin"]);
        qemu.args(&["-kernel", "tornado-kernel.bin"]);
        qemu.args(&[
            "-device",
            "loader,file=shared-scheduler.bin,addr=0x86000000",
        ]); // todo: 这里的地址需要可配置
        qemu.args(&[
            "-device",
            format!("loader,file={}.bin,addr=0x87000000", app.as_ref()).as_str(),
        ]);
        qemu.args(&["-smp", format!("threads={}", &threads).as_str()]);
        qemu.args(&["-gdb", "tcp::1234", "-S"]);

        if let Ok(status) = qemu.status() {
            if status.success() {
                Ok(())
            } else {
                Err(XTaskError::QemuDebugError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
        }
    }
    fn gdb(&self) -> Result {
        // @{{gdb}} --eval-command="file {{kernel-elf}}" --eval-command="target remote localhost:1234"
        let mut gdb = Command::new(&self.gdb);
        gdb.current_dir(self.target_dir());
        gdb.args(&["--eval-command", "file tornado-kernel"]);
        gdb.args(&["--eval-command", "target remote localhost:1234"]);
        gdb.arg("-q");
        if let Ok(status) = gdb.status() {
            if status.success() {
                Ok(())
            } else {
                Err(XTaskError::GDBError)
            }
        } else {
            Err(XTaskError::CommandNotFound)
        }
    }
}
