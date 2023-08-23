{
  description = "Solutions to cryptopals exercises";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import rust-overlay.inputs.nixpkgs { inherit system overlays; };

        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo-flamegraph
            pkgs.rust-analyzer
            rust
          ];
        };
      });
}
