use crate::action::{Action, Result};

pub struct Help {}

impl Help {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Action for Help {
    fn execute(&mut self) -> Result<()> {
        println!("CProj - A C/C++ project manager\n");
        println!("Some of the folders created by cproj have a few rules, namely the lib and");
        println!("include folders.\n");
        println!("lib is for holding library files to link to with -l");
        println!("include is for the header files for those libraries\n");
        println!("This is important because cproj does not check lib or include for changes when");
        println!("recompiling. So if you store your own header files in it instead of in src then");
        println!("any of the dependents of the header files will not recompile if the header");
        println!("changed but the src file did not.");
        println!("On top of this, cproj will only check for changes in headers included by");
        println!("#include \"header_name\" statements as it presumes that headers included by");
        println!("#include <header_name> statements wont change.\n");
        println!(
            "If you need to ensure that the entire project gets recompiled, run 'cproj clean'"
        );
        println!("before building/running\n");
        println!("Any headers included via a #include\"header_name\" directive must be in the");
        println!("project src folder");
        println!("\nAnother rule you need to follow is that cc must be a gcc or g++");
        println!("This is due to how cproj works internally");
        println!("\ncproj new - creates a new c/c++ project");
        println!("\t--name: The name of the project, '--name' can optionally be ommitted");
        println!("\t--lang: The language for the project [c, cpp, c++] (ignores casing), defaults");
        println!("\t        to '--lang c'");
        println!("\ncproj init - initializes the current folder with a c/c++ project");
        println!("\t--name: The name of the project, can optionally be ommited for cproj to use");
        println!("\t        the name of the current folder");
        println!("\t--lang: The language for the project [c, cpp, c++] (ignores casing), defaults");
        println!("\t        to '--lang c'");
        println!("\ncproj run - builds and runs the project");
        println!("\t--release: runs the project in release mode instead of debug mode");
        println!("\ncproj build - builds the project");
        println!("\t--release: builds the project in release mode instead of debug mode");
        println!("\ncproj clean - clears the bin directory");
        println!("\ncproj help - prints this dialogue");

        Ok(())
    }

    fn undo(&mut self) -> Result<()> {
        Ok(())
    }
}
