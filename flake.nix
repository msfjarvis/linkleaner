{
  description = "linkleaner";

  inputs = {
    nixpkgs = { url = "github:NixOS/nixpkgs/nixpkgs-unstable"; };

    flake-utils = { url = "github:numtide/flake-utils"; };

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-compat.follows = "flake-compat";
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    { self, nixpkgs, crane, flake-utils, advisory-db, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustNightly = pkgs.rust-bin.nightly."2022-12-15".default.override {
          extensions = [ "rust-src" ];
          targets =
            pkgs.lib.optionals pkgs.stdenv.isDarwin [ "aarch64-apple-darwin" ]
            ++ pkgs.lib.optionals pkgs.stdenv.isLinux
            [ "x86_64-unknown-linux-gnu" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustNightly;
        src = craneLib.cleanCargoSource ./.;
        cargoArtifacts = craneLib.buildDepsOnly { inherit src buildInputs; };
        buildInputs = [ ];

        linkleaner = craneLib.buildPackage {
          inherit src;
          doCheck = false;
        };
        linkleaner-clippy = craneLib.cargoClippy {
          inherit cargoArtifacts src buildInputs;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        };
        linkleaner-fmt = craneLib.cargoFmt { inherit src; };
        linkleaner-audit = craneLib.cargoAudit { inherit src advisory-db; };
        linkleaner-nextest = craneLib.cargoNextest {
          inherit cargoArtifacts src buildInputs;
          partitions = 1;
          partitionType = "count";
        };
      in {
        checks = {
          inherit linkleaner linkleaner-audit linkleaner-clippy linkleaner-fmt
            linkleaner-nextest;
        };

        packages.default = linkleaner;
        packages.container = pkgs.dockerTools.buildImage {
          name = "registry.fly.io/linkleaner";
          tag = "latest-${system}";
          created = "now";
          copyToRoot = pkgs.buildEnv {
            name = "linkleaner";
            paths = [ linkleaner ];
            pathsToLink = [ "/bin" ];
          };
          config.Cmd = [ "${linkleaner}/bin/linkleaner" ];
        };

        apps.default = flake-utils.lib.mkApp { drv = linkleaner; };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          nativeBuildInputs = with pkgs; [ cargo-release nil rustNightly ];
        };
      });
}
