use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::env;
use cc;

use super::Asm;

impl Asm {
    #[cfg(not(windows))]
    pub fn generate(&self) {
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

    #[cfg(windows)]
    pub fn generate(&self) {
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
