{
  description = "Meson hello world example";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixcode.url = "github:Dessera/nixcode";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-parts,
      nixcode,
      rust-overlay,
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      perSystem =
        { system, ... }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          nixcodeLib = nixcode.lib;
          nixcodePkgs = nixcode.legacyPackages.${system};

          nixcodeInst = nixcodeLib.mkCode {
            inherit pkgs;
            deriveFrom = [
              nixcodePkgs.c_cpp
              nixcodePkgs.rust
              nixcodePkgs.python
            ];
          };

          corssPkgs = pkgs.pkgsCross.riscv64;

          rsPkg = pkgs.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-analyzer"
              "rust-src"
            ];
            targets = [ "riscv64gc-unknown-none-elf" ];
          };
        in
        {
          devShells.default = corssPkgs.mkShell {
            packages = with pkgs; [
              nixd
              nixfmt-rfc-style
              bear
              clang-tools_19
              nixcodeInst.package

              gnumake
              gcc
              rsPkg
              python3
            ];
          };
        };
    };
}
