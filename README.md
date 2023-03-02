<!-- markdownlint-disable MD024 -->
# wiwipaccer, aka, Pack Builder&trade; <!-- omit from toc -->

The Pack Builder&trade; is a desktop app that provides the ability to create a customised resource pack, offering extremely fine grained options, like the ability to choose what to include on as fine as a per-texture level. It does this by providing a new format for resource pack "source code", which seperates out textures seperately. It also allows you to maintain different variations of the same texture, for different minecraft versions, and allowing you to build a resource pack for any pack version / minecraft version that supports resource packs, provided the "source code" provides the necessary textures.

## Table of Contents <!-- omit from toc -->

- [Feature list (nonexhaustive, pack authors)](#feature-list-nonexhaustive-pack-authors)
- [Feature list (nonexhaustive, pack consumers)](#feature-list-nonexhaustive-pack-consumers)
- [Documentation](#documentation)
- [Building from source](#building-from-source)
  - [Dependencies](#dependencies)
    - [All](#all)
    - [macOS](#macos)
    - [Linux](#linux)
    - [Windows](#windows)
  - [Steps](#steps)
    - [macOS (Apple Silicon, aarch64)](#macos-apple-silicon-aarch64)
    - [macOS (Intel, x64)](#macos-intel-x64)
    - [macOS (Universal)](#macos-universal)
    - [Linux](#linux-1)
    - [Windows](#windows-1)

## Feature list (nonexhaustive, pack authors)

- a new way to organise pack textures in a "source code" format (datasource)
- ability to provide different (configurable) options for textures
- ability to maintain textures for different versions of minecraft side by side (giving your pack wider MC version support)
- easier, more straightforward, less tedious methods to make textures in common patterns (random textures, etc)
<!-- - in dev mode: Will rebuild the pack on save, and with an installed mod, will auto reload the textures ingame after build success -->

## Feature list (nonexhaustive, pack consumers)

- ability to choose which textures to include, for custom builds
- ability to choose textures from and make a custom pack with textures from multiple different pack projects
- ability to export and share presets with friends (coming soon)

## Documentation

Documentation will be available in-app once written (Coming Soon™).

## Building from source

The app is built using a nightly Rust toolchain to allow to build Rust's standard library from source. This has a few benefits, including smaller binary size, better optimisations, and faster/more efficient binaries. The tradeoff is longer compile times, but this is a tradeoff we (Meadowsys) were willing to make.

### Dependencies

#### All

- [PnPm] v7.28.0
  - Install standalone on macOS/Linux: run `curl -fsSL https://get.pnpm.io/install.sh | PNPM_VERSION=7.28.0 sh`
  - windows instructions Coming Soon™
- [Node.js] v18.14.2: if pnpm is installed using the standalone method, the right version will be fetched/used automatically (will not touch global config)
- [Rust] nightly-2023-02-26: if rustup is installed, the right version will be fetched/used automatically (will not touch global config)
  - Install rustup on macOS/Linux: run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`, and follow the prompts.
    - If only building, the `minimal` profile is enough. If developing, you likely want the `default` profile. Everything else should be okay to leave at default.
  - Add std-src component: `rustup component add rust-src`

#### macOS

- XCode Developer Tools (also includes git): install by running `xcode-select --install`

#### Linux

documentation Coming Soon™

#### Windows

documentation Coming Soon™

### Steps

#### macOS (Apple Silicon, aarch64)

Install node dependencies:

```sh
pnpm i
```

Run build:

```sh
pnpm tauri build --target aarch64-apple-darwin -- -Z build-std
```

#### macOS (Intel, x64)

Install node dependencies:

```sh
pnpm i
```

Run build:

```sh
pnpm tauri build --target x86_64-apple-darwin -- -Z build-std
```

#### macOS (Universal)

This is for the people who aren't sure which processor architecture/vendor their Mac has. It is preferred to use one of the other two methods. Build time and app size are both nearly double when building for this target (as it is quite literally both aarch64 and x64 builds glued together).

Install compilation targets:

```sh
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
```

Install node dependencies:

```sh
pnpm i
```

Run build:

```sh
pnpm tauri build --target universal-apple-darwin -- -Z build-std
```

#### Linux

Coming Soon™

#### Windows

Coming Soon™

<!-- links -->

[Node.js]: https://nodejs.org/
[PnPm]: https://pnpm.io
[Rust]: https://www.rust-lang.org
