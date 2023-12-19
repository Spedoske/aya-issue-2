{
  description = "A dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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
      with pkgs;
      {
        devShells.default = mkShell {
          nativeBuildInputs = [
            openssl
            pkg-config
            tcpdump
            llvmPackages_17.libllvm
            (rust-bin.nightly."2023-12-15".default.override {
              extensions = [ "rust-src" ];
            })
          ];
        };
        formatter = nixpkgs.legacyPackages.${system}.nixpkgs-fmt;
      }
    );
}
