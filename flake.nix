# in flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        has_toolchain_file = builtins.pathExists ./rust-toolchain.toml;

        rustToolchain =
          if has_toolchain_file then
            pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
          else
            pkgs.rust-bin.selectLatestNightlyWith (
              toolchain:
              toolchain.default.override {
                extensions = [
                  "rust-src"
                  "rust-std"
                  "rustc"
                  "cargo"
                ];
                targets = [ "arm-unknown-linux-gnueabihf" ];
              }
            );

        nativeBuildInputs = with pkgs; [
          rustToolchain
          rustfmt
          pkg-config
        ];

        buildInputs = with pkgs; [
          udev
          vulkan-loader
          nasm
          pkg-config
          openssl.dev
          clang
          cmake
          gcc
          glib
          libxkbcommon.dev
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          inherit buildInputs nativeBuildInputs;
          LD_LIBRARY_PATH = lib.makeLibraryPath (
            [
              # add some stuffs here
            ]
            ++ buildInputs
            ++ nativeBuildInputs
          );
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          OPENSSL_DIR = "${pkgs.openssl.out}";
          OPENSSL_LIB_DIR = lib.makeLibraryPath [ pkgs.openssl ];
        };
      }
    );
}
