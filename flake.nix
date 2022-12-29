{
  description = "linkleaner";

  inputs = {
    nixpkgs = {url = "github:NixOS/nixpkgs/nixpkgs-unstable";};

    flake-utils = {url = "github:numtide/flake-utils";};

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

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    advisory-db,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      rustNightly = pkgs.rust-bin.nightly."2022-12-15".default.override {
        extensions = ["rust-src"];
        targets =
          pkgs.lib.optionals pkgs.stdenv.isDarwin ["aarch64-apple-darwin"]
          ++ pkgs.lib.optionals pkgs.stdenv.isLinux
          ["x86_64-unknown-linux-gnu"];
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
        tag = "latest-${system}";
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

        nativeBuildInputs = with pkgs; [cargo-release nil rustNightly];
      };
    });
}
