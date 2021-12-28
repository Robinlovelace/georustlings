# georustlings

This project is designed to be an 'on-ramp' for people who want to start using, contributing to, and building tools in the [GeoRust](https://github.com/georust/) ecosystem.

It is inspired by [rustlings](https://github.com/rust-lang/rustlings).

## Under construction 🏗️

This project is at an early stage in its development so feedback is welcome.
Therefore the following is just a sketch and things may change.

## Running the exercises

To run the exercises you need to have installed Git and Rust.
With those installed you can follow the steps below, typing the commands into a terminal such `bash` or `zsh` in Linux or Mac, or `PowerShell` in Windows:

1. Clone this repo, e.g. with
```
git clone https://github.com/georust/rustlings
```
2. Navigate into the `georustlings` folder, e.g. with
```bash
cd georustlings # in bash
```
3. Identify the exercise you want to work on and navigate into it, e.g. by typing:
```bash
cd print_point # to work on the print_point exercise
```
4. Try running the code:
```bash
cargo run print_point # if you see output like this ⏬ it worked 🎉
#    Compiling print_point v0.1.0 (/home/u/code/georustlings/print_point)
#     Finished dev [unoptimized + debuginfo] target(s) in 0.31s
#      Running `target/debug/print_point`
# Point(Coordinate { x: 1.23, y: 4.56 })%   
```
5. Run the tests
```bash
cargo test # you *should* see errors like those below here...
# your job is to edit the code in the src/ folder to make them pass:
# Output from the command above before you've solved the exercise:
# running 1 test
# test tests::test ... FAILED
# 
# failures:
# 
# ---- tests::test stdout ----
# thread 'tests::test' panicked at 'assertion failed: `(left == right)`
#   left: `"Point(Coordinate { x: 1.23, y: 4.56 })"`,
#  right: `"y=4.56, x=1.23"`', src/lib.rs:19:9
# note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
# 
# 
# failures:
#     tests::test
# 
# test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
6. Edit the code in the exercise folder and run `cargo test` again, when the tests pass you will see output that give a postitive results like the following:
```bash
cargo test
#    Compiling print_point v0.1.0 (/home/robin/learning/rust/georustlings/print_point)
#     Finished test [unoptimized + debuginfo] target(s) in 0.33s
#      Running unittests (target/debug/deps/print_point-90c173b7cd365c33)
# 
# running 1 test
# test tests::test ... ok
# 
# test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
# 
#      Running unittests (target/debug/deps/print_point-0f66e01d6e84a5fb)
# 
# running 0 tests
# 
# test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
# 
#    Doc-tests print_point
# 
# running 0 tests
# 
# test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
7. Think about problems you want to solve, sketch them, and consider solving another exercise in this repo (go back to item 3)
8. If you've finished working on `georust` exercises, think about new challenges you'd like to add and how to contribute to the community

<!-- Todo: add a nice schematic diagram of this? -->

## Architecture

The architecture of this project is simpler than that of `rustlings`.
`rustlings` provides a command line tool that continuously watches for files in the repo that can be activated with `rustlings watch`, and allows you to see how Rust code works quickly without worrying about build commands etc.
This is a powerful learning technique and we **recommend anyone new to Rust to work through the exercises in `rustlings`**, at least completing the move_semantics exercises, before attempting the challenges in this repo.

However, the `rustlings` approach has some costs that, at the time of writing, made it non-trivial to extend.
As documented in [`rust-lang/rustlings#897`](https://github.com/rust-lang/rustlings/issues/897), it is not easy to make new exercises with dependencies, such as the [`geo`](https://github.com/georust/geo) crate, which lies at the heart of GeoRust.
Furthermore, when building new tools building on the foundations provided by the `geo` Rust crate (or any Rust crate), it makes sense to start your project by creating a new crate, e.g. by running the following command:

```bash
cargo new georustlings
```
That will create a new crate that allows you to specify not only which crates your code relies on but which versions to use and more.
Within this tried-and-tested framework, you can add dependencies by editing `Cargo.toml` manually or automatically, e.g. with:

```bash
cargo install cargo-edit # install the cargo-edit crate, enabling the following command:
cargo add geo            # add the geo create to the list of dependencies
```

Due to the advantages of packaging Rust code in crates, and the advantages of allowing others to see and hack these 'mini projects', we organise the challenges in this repo as crates, one exercise per crate.
`rustlings` organises exercise in folders `variables`, `if`, `move_semantics` etc, corresponding to key concepts in Rust.
Likewise, the plan is for exercises in `georustlings` to be organised into folders the correspond to key concepts when working with geographic data, e.g. (feedback on these especially welcome):

- Geometry types: in the folder `ex/geo`
- Data reading and writing: `ex/io`
- Spatial relations: `ex/relations`
- Geometry operations (e.g. buffers): `ex/geomops`
- Working with attribute data: `ex/attributes`

To keep things open and to prevent [over-architecting](https://www.stoutsystems.com/over-architecting/)[^1]
, rather than putting the exercises/crates into these boxes/folders now, we will put them all in the root directory, for now.
The [`print_point`](print_point) exercise demonstrates the 'point'.
Replace `some_exercise` with `print_point` (or a more original name for a new exercise) and you will have created a new exercise for `georustlings`.

[^1]: "Aside from costing more up-front, over-architecting has long term costs. More code means bugs are more likely."

Contributions in the form of issues and pull requests are welcome.

## Creating a new exercise

Each exercise is a regular library crate. You can make a new one:

```bash
cargo new --lib some_exercise
```

You can add whatever dependencies you like, but probably every exercise will at
least use `geo`. You can add it:

```bash
cd some_exercise
cargo add geo
```

Each exercise is expressed as a unit test. You can work through a particular exercise:

```bash
cd some_exercise
cargo test
# Edit src/lib.rs and fix the problem, then repeat
```
