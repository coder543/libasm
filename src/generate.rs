use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::env;
use cc;

use super::Asm;

impl Asm {
    #[cfg(unix)]
    pub fn generate(&self) {
        if let Some(ref ret) = self.ret {
            if ret != "%rax" {
                panic!("for now, the return argument for an assembly function must be %rax.");
            }
        }

        let out_dir = env::var("OUT_DIR").unwrap();
        let path = PathBuf::from(out_dir).join(self.name.clone() + ".S");
        let mut output = File::create(&path).unwrap();
        writeln!(
            output,
            ".intel_syntax\n.text\n.globl {}\n{}:",
            self.name, self.name
        ).unwrap();
        for line in &self.body {
            writeln!(output, "  {}", line).unwrap();
        }

        writeln!(output, "  ret").unwrap();

        cc::Build::new().file(&path).compile(&self.name.clone());
    }

    #[cfg(not(unix))]
    pub fn generate(&self) {
        if let Some(ref ret) = self.ret {
            if ret != "rax" {
                panic!("for now, the return argument for an assembly function must be %rax.");
            }
        }

        let out_dir = env::var("OUT_DIR").unwrap();
        let path = PathBuf::from(out_dir).join(self.name.clone() + ".asm");
        let mut output = File::create(&path).unwrap();
        writeln!(output, ".code\nPUBLIC {}\n{} PROC", self.name, self.name).unwrap();
        for line in &self.body {
            writeln!(output, "{}", line).unwrap();
        }

        writeln!(output, "ret\n{} ENDP\nEND", self.name).unwrap();

        cc::Build::new().file(&path).compile(&self.name.clone());
    }
}
