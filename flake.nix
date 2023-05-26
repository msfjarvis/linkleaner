{
  description = "linkleaner";

  inputs = {
    nixpkgs = {url = "github:NixOS/nixpkgs/nixpkgs-unstable";};

    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    flake-utils = {url = "github:numtide/flake-utils";};

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-compat.follows = "flake-compat";
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    crane,
    flake-utils,
    advisory-db,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      rustNightly = (import fenix {inherit pkgs;}).fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-1elQHBWEQRZ5qrEtJi6uhvwUNKedyQusyhATdBywep0=";
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustNightly;
      commonArgs = {
        src = craneLib.cleanCargoSource ./.;
        buildInputs = [];
        nativeBuildInputs = [];
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      };
      cargoArtifacts = craneLib.buildDepsOnly (commonArgs
        // {
          pname = "linkleaner-deps";
        });
      linkleaner-fmt = craneLib.cargoFmt (commonArgs
        // {
          inherit cargoArtifacts;
        });
      linkleaner-clippy = craneLib.cargoClippy (commonArgs
        // {
          cargoArtifacts = linkleaner-fmt;
        });
      linkleaner = craneLib.buildPackage (commonArgs
        // {
          cargoArtifacts = linkleaner-clippy;
          doCheck = false;
        });
      linkleaner-nextest = craneLib.cargoNextest (commonArgs
        // {
          cargoArtifacts = linkleaner;
          partitions = 1;
          partitionType = "count";
        });
      linkleaner-audit = craneLib.cargoAudit (commonArgs
        // {
          inherit advisory-db;
          cargoArtifacts = linkleaner;
        });
    in {
      checks = {
        inherit
          linkleaner
          linkleaner-audit
          linkleaner-clippy
          linkleaner-fmt
          linkleaner-nextest
          ;
      };

      packages.default = linkleaner;
      packages.container = pkgs.dockerTools.buildImage {
        name = "registry.fly.io/linkleaner";
        tag = "latest";
        created = "now";
        copyToRoot = pkgs.buildEnv {
          name = "linkleaner";
          paths = [linkleaner];
          pathsToLink = ["/bin"];
        };
        config.Cmd = ["${linkleaner}/bin/linkleaner"];
      };

      apps.default = flake-utils.lib.mkApp {drv = linkleaner;};

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        nativeBuildInputs = with pkgs; [cargo-nextest cargo-release flyctl nil rustNightly];

        CARGO_REGISTRIES_CRATES_IO_PROTOCOL = "sparse";
      };
    });
}
