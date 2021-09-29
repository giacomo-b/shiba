<h1 align="center">
  <a href="https://github.com/giacomo-b/shiba"><img src="https://cdn.shibe.online/shibes/94cb90933e179375608c5c58b3d8658ef136ad3c.jpg" alt="shiba" width="300"></a>
  <br>
  Shiba CLI
</h1>
<h4 align="center">Command-line interface (CLI) to display a random Shiba Inu whenever needed, by just running <em>shiba</em> on your terminal.</h4>
<p align="center">
  <a href="https://github.com/giacomo-b/shiba">
    <img src="https://img.shields.io/badge/SO-MUCH-blue?style=for-the-badge"
         alt="SoMuch">
  </a>
  <a href="https://github.com/giacomo-b/shiba">
    <img src="https://img.shields.io/badge/SUCH-COOL-red?style=for-the-badge"
         alt="SuchCool">
  </a>
  <a href="https://github.com/giacomo-b/shiba">
    <img src="https://img.shields.io/badge/MUCH-WOW-green?style=for-the-badge"
         alt="MuchWow">
  </a>
</p>

<p align="center">
  <a href="#how-to-use">How To Use</a> •
  <a href="#how-does-it-work">How Does It Work?</a> •
  <a href="#why">Why?</a> •
  <a href="#build-manually">Build Manually</a> •
  <a href="#todos">TODOs</a>
</p>

## How to use

> **Note:** Pre-compiled binaries will be available soon. Currently, you need to compile the program on your machine. Fortunately, Rust makes this process extremely easy.

### Install Rust

If you don't have [Rust](https://www.rust-lang.org/) installed on your system, you will need to install it.

<details>
<summary>Linux, macOS, and Unix-based systems</summary>

- Run the following in your terminal

```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Follow the on-screen instructions
</details>

<details>
<summary>Windows</summary>

- Download the installer from [here](https://www.rust-lang.org/tools/install)
- Run the installer and follow the on-screen instructions
</details>

### Install Shiba

```console
cargo install --git https://github.com/giacomo-b/shiba.git
```

You should be able to just run `shiba` from anywhere on your system, and that should present you with a new picture every time.

## How does it work?

`shiba CLI` is based on [shibe.online](https://shibe.online/), which provides a free, public API to get random shibes.
  
## Why?

Why not?

## Build manually

**TL;DR:** compile with Rust and move the binary/executable to a folder in your path.

If you don't have Rust on your system, follow the [instructions above](#install-rust) to install it.

### Build the program

#### Get the code
  You can clone the repo using SSH by running:

  ```console
  git clone git@github.com:giacomo-b/shiba.git
  ```
  <details>
  <summary>Alternative methods to get the code</summary>
  
  You may also:
  - Clone using HTTPS

    ```console
    git clone git@github.com:giacomo-b/shiba.git
    ```
  - Download the [zip](https://github.com/giacomo-b/shiba/archive/refs/heads/master.zip).
  </details>

#### Compile with Rust

```console
cd shiba && cargo build --release
```

### Run

To run the code from the `shiba` folder, run:
```console
cargo run --release
```
or just launch the binary generated in `shiba/target/release`.

If you want to be able to run `shiba` from any location on your system, you will have to move the generated binary to the appropriate folder depending on your system:

<details>
<summary>Linux and most Unix-based systems</summary>

- Run the following from within the `shiba` folder
```console
sudo mv ./target/release/shiba /bin/
```
- Restart terminal
</details>

<details>
<summary>macOS</summary>

- Run the following from within the `shiba` folder
```console
sudo mv ./target/release/shiba /usr/local/bin/
```
- Restart terminal
</details>

<details>
<summary>Windows</summary>

- Place `shiba.exe` (found in `shiba/target/release/`) in a directory of your choice (such as `C:\your\path\here\`)
- Run the following:
```console
set PATH=%PATH%;C:\your\path\here\
```
- Restart terminal
</details>

Now you should be able to just run `shiba` from anywhere on your system, and that should present you with a new picture every time.

## TODOs

- [ ] Add CI workflows
  - [ ] Perform compilation checks
  - [ ] Automatically generate builds
- [ ] Reduce binary size (?)
- [ ] Add command-line options
  - [ ] shibe.online's [API](https://shibe.online/) accepts `count`, maybe more than one Shiba could be display on demand?
