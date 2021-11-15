{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            qemu_full
            cargo-binutils
            (rust-bin.nightly."2020-06-27".minimal.override {
              targets = [ "riscv64gc-unknown-none-elf" ];
              extensions = [ "rust-src" "clippy" "cargo" "rustfmt-preview" "llvm-tools-preview" ];
            })
          ];
        };
      }
    );
}

