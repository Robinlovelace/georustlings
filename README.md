# georustlings

This project is designed to be an 'on-ramp' for people who want to start using, contributing to, and building tools in the [GeoRust](https://github.com/georust/) ecosystem.

It is inspired by [rustlings](https://github.com/rust-lang/rustlings).

## Under construction 🏗️

This project is at an early stage in its development so feedback is welcome.
Therefore the following is just a sketch and things may change.

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
Due to the advantages of packaging Rust code in crates, and the advantages of allowing others to see and hack these 'mini projects', we organise the challenges in this repo as crates, one exercise per crate.
`rustlings` organises exercise in folders `variables`, `if`, `move_semantics` etc, corresponding to key concepts in Rust.
Likewise, the plan is for exercises in `georustlings` to be organised into folders the correspond to key concepts when working with geographic data, e.g. (feedback on these especially welcome):

- Geometry types: in the folder `ex/geo`
- Data reading and writing: `ex/io`
- Spatial relations: `ex/relations`
- Geometry operations (e.g. buffers): `ex/geomops`
- Working with attribute data: `ex/attributes`

To keep things open and to prevent [over-architecting](https://www.stoutsystems.com/over-architecting/)^["Aside from costing more up-front, over-architecting has long term costs. More code means bugs are more likely."], rather than putting the exercises/crates into these boxes/folders now, we will put them all in the root directory, for now.
The [`print_point`](print_point) exercise demonstrates the 'point'.
Replace `some_exercise` with `print_point` (or a more original name for a new exercise) and you will have created a new exercise for `georustlings`.

Contributions in the form of issues and pull requests are welcome.

##

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
