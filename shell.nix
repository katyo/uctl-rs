{
  pkgs ? import <nixpkgs> {
    overlays = [
      (import <rust-overlay>)
    ];
  }
}:
with pkgs;

let
rust-cfg = {
  extensions = [
    "rust-src"
    "rustfmt"
    "llvm-tools"
    #"rust-analyzer"
    "clippy"
    #"miri"
  ];
  targets = [
    #"thumbv7em-none-eabi"
    "thumbv7m-none-eabi"
    "thumbv6m-none-eabi"
    #"thumbv7em-none-eabihf"
    #"thumbv8m.main-none-eabihf"
    #"riscv32imac-unknown-none-elf"
    #"wasm32-unknown-unknown"
  ];
};

rust-stable = rust-bin.stable.latest.default.override rust-cfg;
rust-nightly-latest = rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override rust-cfg);

in mkShell {
  buildInputs = [
    rust-stable
    probe-rs
  ];
}
