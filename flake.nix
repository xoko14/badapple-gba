{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        stableToolchain = pkgs.rust-bin.nightly.latest.minimal.override {
          extensions = [ "rust-src" "rust-analyzer"];
        };
        agb-gbafix = pkgs.rustPlatform.buildRustPackage rec {
    pname = "agb-gbafix";
    version = "0.19.1";

    src = pkgs.fetchFromGitHub {
      owner = "agbrs";
      repo = pname;
      rev = version;
      hash = "sha256-+s5RBC3XSgb8omTbUNLywZnP6jSxZBKSS1BmXOjRF8M=";
    };
    cargoHash = "sha256-VZLgRng4Y6ZP57TtrkFGZQYEHciMhC4GG4Jzp9sqQok=";
    doCheck = false;
  };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            eza
            fd
            stableToolchain
            ffmpeg-full
            llvmPackages_11.clang
            mgba
            agb-gbafix
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
          LIBCLANG_PATH = "${pkgs.llvmPackages_11.libclang.lib}/lib";
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
