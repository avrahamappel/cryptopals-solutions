{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    cargo-flamegraph
    clippy
    rustc
    rustfmt
  ];
  RUST_SRC_DIR = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
