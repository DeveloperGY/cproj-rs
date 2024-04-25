use std::{
    cell::RefCell,
    collections::HashSet,
    fs,
    io::Write,
    path::{self},
    process,
    rc::Rc,
};

use crate::{
    action::{Action, Result},
    config::Config,
};

pub struct CompileFiles {
    src_files: Rc<RefCell<HashSet<path::PathBuf>>>,
    files_to_compile: Rc<RefCell<HashSet<path::PathBuf>>>,
    config: Rc<RefCell<Config>>,
    release_mode: bool,
}

impl CompileFiles {
    pub fn new(
        src_files: Rc<RefCell<HashSet<path::PathBuf>>>,
        files_to_compile: Rc<RefCell<HashSet<path::PathBuf>>>,
        config: Rc<RefCell<Config>>,
        release_mode: bool,
    ) -> Box<Self> {
        Box::new(Self {
            src_files,
            files_to_compile,
            config,
            release_mode,
        })
    }

    fn get_code_files(&self) -> Vec<path::PathBuf> {
        let binding = self.files_to_compile.borrow();

        binding
            .iter()
            .filter(|path| {
                if let Some(ext) = path.extension() {
                    let ext = ext.to_str().unwrap();

                    (ext.contains('c') || ext.contains('i')) && !ext.contains('t')
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }

    fn convert_to_obj_path(&self, path: &path::Path) -> path::PathBuf {
        let mode = if self.release_mode {
            "release"
        } else {
            "debug"
        };

        let output_path = path.strip_prefix(path::PathBuf::from("src")).unwrap();
        let output_path = output_path.to_str().unwrap().replace('/', "_");
        let mut obj_path = path::PathBuf::from("bin")
            .join(mode)
            .join("obj")
            .join(output_path);
        obj_path.set_extension("o");
        obj_path
    }

    fn convert_to_log_path(&self, path: &path::Path) -> path::PathBuf {
        let mode = if self.release_mode {
            "release"
        } else {
            "debug"
        };

        let output_path = path.strip_prefix(path::PathBuf::from("src")).unwrap();
        let output_path = output_path.to_str().unwrap().replace('/', "_");
        let mut log_path = path::PathBuf::from("bin")
            .join(mode)
            .join("log")
            .join(output_path);
        log_path.set_extension("log");
        log_path
    }

    /// compiles a file and outputs the path to the object file
    fn compile(&self, path: &path::Path) -> Result<path::PathBuf> {
        let obj_path = self.convert_to_obj_path(path);

        let mut cc = process::Command::new(&self.config.borrow().cc);
        cc.arg("-c");
        cc.arg(path.to_str().unwrap());
        cc.arg("-o");
        cc.arg(&obj_path);

        for inc in &self.config.borrow().include {
            cc.arg("-I");
            cc.arg(inc);
        }

        if self.release_mode {
            cc.args(&self.config.borrow().release_flags);
        } else {
            cc.args(&self.config.borrow().debug_flags);
        }

        // compile and log
        let log_path = self.convert_to_log_path(path);
        let log_file = fs::File::create(&log_path);

        match log_file {
            Ok(file) => {
                cc.stdout(
                    file.try_clone()
                        .map_or(process::Stdio::null(), |val| val.into()),
                );
                cc.stderr(process::Stdio::from(file));
            }
            Err(_) => {
                println!(
                    "    -> failed create log for {}",
                    &log_path.to_str().unwrap()
                );
            }
        };

        let compile_output = cc.output().unwrap(); // if err then the command failed to execute

        if !compile_output.status.success() {
            println!("    -> compilation failed");
            Err(format!("failed to compile {}", path.to_str().unwrap()).into())
        } else {
            Ok(obj_path)
        }
    }

    fn link(&self) -> Result<()> {
        println!("    -> linking binary...");
        let bin_path = if self.release_mode {
            format!("bin/release/{}", &self.config.borrow().name)
        } else {
            format!("bin/debug/{}", &self.config.borrow().name)
        };

        let mut link = process::Command::new(&self.config.borrow().cc);

        for flg in &self.config.borrow().link_flags {
            link.arg(flg);
        }

        // link all objs
        for path in self.src_files.borrow().iter().filter(|path| {
            if let Some(ext) = path.extension() {
                let ext = ext.to_str().unwrap();

                (ext.contains('c') || ext.contains('i')) && !ext.contains('t')
            } else {
                false
            }
        }) {
            link.arg(self.convert_to_obj_path(path));
        }

        if self.release_mode {
            link.args(&self.config.borrow().release_flags);
        } else {
            link.args(&self.config.borrow().debug_flags);
        }

        for lib in &self.config.borrow().lib {
            link.arg("-L").arg(lib);
        }

        link.arg("-o").arg(bin_path);

        let log_path = if self.release_mode {
            "bin/release/log/linker.log"
        } else {
            "bin/debug/log/linker.log"
        };

        let log_file = fs::File::create(log_path);

        match log_file {
            Ok(file) => {
                link.stdout(
                    file.try_clone()
                        .map_or(process::Stdio::null(), |val| val.into()),
                );
                link.stderr(process::Stdio::from(file));
            }
            Err(_) => {
                println!("    -> failed to generate the linker log file",);
            }
        };

        let link_output = link.output().unwrap(); // if err then the command failed to execute

        if !link_output.status.success() {
            println!("    -> linking failed");
            Err("failed to link binary".into())
        } else {
            Ok(())
        }
    }
}

impl Action for CompileFiles {
    fn execute(&mut self) -> Result<()> {
        println!("=> Compiling...");

        // get just the source files without header files
        let code_files: Vec<_> = self.get_code_files();

        let mut compile_results = vec![];

        // compile files
        for file in code_files {
            println!("    -> compiling {}", file.to_str().unwrap());
            compile_results.push(self.compile(&file));
        }

        // check for compile errors
        if !compile_results.is_empty() {
            println!("    -> checking for compilation errors...");

            let mut had_error = false;

            for res in compile_results {
                if let Err(err) = res {
                    had_error = true;
                    println!("    -> {}, check logs for more info", err.get_msg());
                }
            }

            if had_error {
                println!("    -> compile errors detected, aborting...");
                return Err("failed to compile".into());
            }
        }

        // link to final binary
        if let Err(err) = self.link() {
            println!("    -> failed to link binary");
            Err(err)
        } else {
            let timestamp = if self.release_mode {
                "bin/release/timestamp"
            } else {
                "bin/debug/timestamp"
            };

            let mut timestamp_file = fs::File::create(timestamp).unwrap();
            timestamp_file.write_all(b"This file holds the mtime of the last successful compile\nIf you edit this file, you must delete it otherwise cproj might not detect some changes in the project").unwrap();

            Ok(())
        }
    }

    fn undo(&mut self) -> Result<()> {
        // undoing the steps here is not required since anything incorrect would
        // be overritten by editing the src files and rebuilding the project
        Ok(())
    }
}
