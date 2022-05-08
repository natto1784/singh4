{
  description = "singh4 discord bot";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixos-unstable;
    utils.url = github:numtide/flake-utils;
    rust-overlay.url = github:oxalica/rust-overlay;
    cargo2nix.url = github:cargo2nix/cargo2nix;
  };

  outputs = { self, nixpkgs, utils, rust-overlay, cargo2nix }:
    utils.lib.eachDefaultSystem
      (system:
        let
          overlays =
            [
              (import "${cargo2nix}/overlay")
              rust-overlay.overlay
            ];

          pkgs = import nixpkgs {
            inherit system overlays;
          };

          rustPkgs = pkgs.rustBuilder.makePackageSet' {
            rustChannel = "latest";
            packageFun = import ./Cargo.nix;
            packageOverrides = pkgs: pkgs.rustBuilder.overrides.all ++ [
              (pkgs.rustBuilder.rustLib.makeOverride {
                name = "singh4";
                overrideAttrs = drv: {
                  propagatedNativeBuildInputs = drv.propagatedNativeBuildInputs or [ ] ++ [
                    pkgs.openssl
                    pkgs.archiver
                    pkgs.pkg-config
                  ];
                };
              })
            ];
          };

        in
        rec {
          devShell = with pkgs; mkShell {
            buildInputs = [
              rust-bin.nightly.latest.default
              rust-analyzer
              postgresql
            ];
          };

          packages = {
            default = (rustPkgs.workspace.singh4 { }).bin;
          };

          defaultPackage = packages.default;
        }
      );
}
