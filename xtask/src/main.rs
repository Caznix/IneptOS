// Target
// Release
// Build threads
// Config Files

// cargo xtask run target
// cargo xtask build target
// cargo xtask clean target

mod run;
mod kbuild;
mod clean;
mod cli;
mod iso;

use cli::{Target, Task};
use run::{qemu_open_aarch64, qemu_open_riscv64, qemu_open_x86_64};
use kbuild::{build_aarch64, build_riscv64, build_x86_64};
use clean::clean;

use clap::Parser;

fn main() {
    let task = Task::parse();

    match task {
        Task::Run { target } => match target {
            Target::X86_64 => {
                qemu_open_x86_64();
            }
            Target::Aarch64 => {
                qemu_open_aarch64();
            }
            Target::Riscv64 => {
                qemu_open_riscv64();
            }
            Target::All => {
                //qemu_open_x86_64();
                //qemu_open_aarch64();
                //qemu_open_riscv64();
            }
        },
        Task::Build { target } => match target {
            Target::X86_64 => build_x86_64(),
            Target::Aarch64 => build_aarch64(),
            Target::Riscv64 => build_riscv64(),
            Target::All => {
                build_x86_64();
                build_aarch64();
                build_riscv64();
            }
        },
        Task::Clean { target } => match target {
            Target::X86_64 => {
                clean("x86_64");
            }
            Target::Aarch64 => {
                clean("aarch64");
            }
            Target::Riscv64 => {
                clean("riscv64");
            }
            Target::All => {
                clean("x86_64");
                clean("aarch64");
                clean("riscv64");
            }
        },
    }
}
