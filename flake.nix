{
  description = "dot013-mdparser";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crate2nix.url = "github:nix-community/crate2nix";
  };
  nixConfig = {
    extra-trusted-public-keys = "eigenvalue.cachix.org-1:ykerQDDa55PGxU25CETy9wF6uVDpadGGXYrFNJA3TUs=";
    extra-substituters = "https://eigenvalue.cachix.org";
    allow-import-from-derivation = true;
  };
  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    , crate2nix
    ,
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      cargoNix = crate2nix.tools.${system}.appliedCargoNix {
        name = "rustnix";
        src = ./.;
      };
    in
    rec {
      checks = {
        rustnix = cargoNix.rootCrate.build.override {
          runTests = true;
        };
      };
      imports = [
        ./nix/rust-overlay/flake-module.nix
        ./nix/devshell/flake-module.nix
      ];

      packages = {
        rustnix = cargoNix.rootCrate.build;
        default = packages.rustnix;

        inherit (pkgs) rust-toolchain;

        rust-toolchain-versions = pkgs.writeScriptBin "rust-toolchain-versions" ''
          ${pkgs.rust-toolchain}/bin/cargo --version
          ${pkgs.rust-toolchain}/bin/rustc --version
        '';
      };

      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          openssl
          pkg-config
          eza
          fd
          (rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
          })
          rust-analyzer
          jq
          vhs
          ffmpeg
          ttyd
          glow
        ];
      };
    });
}
