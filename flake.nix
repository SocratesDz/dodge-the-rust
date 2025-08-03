{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-unknown-emscripten" ];
            }))
            rustfmt
            pre-commit
            rustPackages.clippy
            rust-analyzer
            tmux
            watchexec
            lazygit
            eza
            fd
            godot
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;

          shellHook = ''
              echo "🦀 Dodge the Rust! 🦀"
            '';
        };
      }
    );
}
