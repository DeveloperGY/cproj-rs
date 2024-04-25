# CPROJ-RS
## Description
Cproj is a basic c/c++ build tool that wraps gcc/g++ (or an equivalent) to
enable incremental compilation. It was mainly meant as a fun side project but I
figured some people might be interested in it.

To use a compiler that isn't gcc/g++ it must be an equivalent to them. This
means that they must use the same flags for the same purposes as gcc/g++ since
cproj was made to interface with gcc/g++ specifically. If you wish to make cproj
work with a different compiler, feel free to fork the repo.

## Installation Instructions
Cproj can be installed in two ways: With cargo or by building from source.

### Installing with Cargo
1. Ensure you have cargo installed, it can be installed using this 
   [guide](https://www.rust-lang.org/tools/install)
2. Run `cargo install cproj-rs`

### Building from Source
1. Ensure you have cargo installed, it can be installed using this 
   [guide](https://www.rust-lang.org/tools/install)
2. Clone the repository
3. In the project root, run `cargo install --path .`

If you wish to avoid cloning the repository manually, you can instead do
`cargo install --git https://github.com/DeveloperGY/cproj-rs`

## Usage
### Creating a Project
To create a project using Cproj, run `cproj new <project_name>`

This will create a new C project in a directory named after the project.

If you wish to make a C++ project instead you can specify the `--lang` flag with
"c++" or "cpp". The `--lang` flag is case-insensitive. You may also specify that
it is a C project by specifying the `--lang` flag with "c", though Cproj creates
a C project by default.

The name of the project may optionally be marked by the `--name` flag but it
isn't necessary.

### Initializing a Project
If you already have a directory you wish to initialize with a project, run
`cproj init <project_name>`

This will create a new C project in the current directory. You can change the
project language using the `--lang` flag as detailed above.

The project name can be tagged with the `--name` flag, just as it can with
`cproj new`, or it can be omitted to tell Cproj to use the name of the current
directory instead.

### Building/Running a Project
Cproj keeps track of file changes to enable incremental compilation. It does
this through checking the last time files were modified. Some systems may not
support this according to the
[rust docs](https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.modified),
in which case Cproj will always rebuild the entire project.

Cproj will only look for changes in the src directory of the project. Cproj
expects that no other files that the project is dependent on will change. There
will be more details on this in the
[Project Configuration](#configuring-a-project) section.

To build a project, run `cproj build`

This will build the project in debug mode. If you wish to build in release mode,
you can specify the `--release` flag. Optionally, if you wish to make it more
clear that you intend to build in debug mode, you can specify the `--debug`
flag.

The project will be built in the ./bin/[debug, release] directory. In this
directory you will find the binary along with a timestamp file. 

The timestamp file is used to keep track of the time the project was last built
in that mode. If the timestamp file is edited and there were changes made to the
source code of the project since the last time the project was build, it would
be worth running `cproj clean` (documented below) or deleting the timestamp
file. This will force Cproj to rebuild the entire project next time it is built.

Within this directory you will also find two folders, `log` and `obj`. The `log`
folder holds all of the build logs for the previous build. If there were any
source files that Cproj decided did not need to be rebuilt, then their previous
log will be left untouched. The `obj` folder holds all of the object files built
by cproj. If any of them get deleted while the corresponding source file was not
it would be worth running `cproj clean` or deleting the timestamp file. This
will force Cproj to rebuild the entire project. The reason you must do this is
because Cproj currently does not check the obj folder to see if a source file
needs to be marked for compilation. This is planned to be fixed in the future.

If you want to run your project after building it, you can do two things.

1. You can run `cproj run`

This will build the project in debug mode and then run it. If you wish to run
the project in release mode you can specify the `--release` flag. Optionally, if
you wish to make it more clear that you intend to build in debug mode, you can
specify the `--debug` flag.

2. You can run `cproj build` and then run the binary yourself.

This option can be advantagous if you don't want to see the build output of
Cproj when you run your project since Cproj will always run `cproj build` before
running the project.

### Cleaning the project
If you want to rebuild the entire project or the object files/timestamp file was
messed with you can run `cproj clean`

The implementation of this command is relatively naive and just removes the
project bin directory with all of its contents then recreates the folder
structure.

If you wish to clean a specific build of the project, just go into the directory
of the build and remove all the files. Be sure to leave the `log` and `obj`
folders as Cproj will error out if it doesnt find them.

Alternatively, if you just wish to fully rebuild the project, remove the
corresponding timestamp file for the mode you wish to rebuild in.

It is planned to make the Cproj clean system a bit more robust in the future.

### The Help Command
To get some basic usage details of Cproj, you can run `cproj help`

The output of this command currently is not as thorough as this README so it's
only really worth using if you don't have an internet connection.

There are plans to make this command much better in the future.

### Configuring a Project
A Cproj project is marked by a file called `cproj.json`. The file looks like
this by default.
```json
{
  "name": "project_name",
  "cc": "gcc",
  "include": [
    "include"
  ],
  "lib": [
    "lib"
  ],
  "link_flags": [],
  "debug_flags": ["-Wall", "-Wextra"],
  "release_flags": ["-Wall", "-Wextra", "-O2"],
  "lang": "C"
}
```
The `name` field specifies the project name. It is used to name the output
binary when building the file.

The `cc` field is the program used to compile the project.

The `include` field specifies the paths to include with `-I` to the program
specified by `cc`. Cproj will not look for changes in any of the folders
specified here. This means that if you update a dependency of your project,
your source files will not recompile with those changes until those source files
themselves get changed or you clean the project.

The `lib` field specifies the paths to include with `-L` to the progran
specified by `cc`. Cproj will not look for changes in any of the folders
specified here. Unlike with the `include` field, when used properly, Cproj
should include changes to any libaries being linked to when rebuilding the
project.

The `link_flags` field specifies all the flags that should be passed to the link
stage. This is also where you should put all of your `-l` flags to link to any
libraries your project needs.

The `debug_flags` field specifies all the flags that should be passed to the
compilation stage when building the project in debug mode.

The `release_flags` field specifies all the flags that should be passed to the
compilation stage when building the project in release mode.

The `lang` field specifies the project language. It can be either "C" or "Cpp".
This field is case-sensitive. If any values other than "C" or "Cpp" are used,
then any Cproj command that needs to read the project config will fail. This
field helps Cproj to determine which source files to check for changes. The
following lists the file extensions Cproj checks for each language.
#### C
  - .c
  - .h
  - .i 
#### Cpp
  - .c
  - .cc
  - .cp
  - .cxx
  - .cpp
  - .CPP
  - .c++
  - .C
  - .i
  - .ii
  - .h
  - .hh
  - .H
  - .hp
  - .hxx
  - .hpp
  - .HPP
  - .h++
  - .tcc

These file extensions were determined based on the
[gcc manual](https://gcc.gnu.org/onlinedocs/gcc/Overall-Options.html)

## Contributing
Currently this is mainly my own side project and until I'm satisfied with my
progress/effort I'd like to keep outside contributions to a minimum. That being
said, submitting issues or forking the repo is perfectly fine and encouraged. My
wish to measure my own progress as a developer should not impact other peoples'
right to modify and improve a tool they use. I'm sorry for any inconvenience
this may cause.

Once the project is in a state where I'm happy with it I will update this
section and will be willing merge outside contributions.