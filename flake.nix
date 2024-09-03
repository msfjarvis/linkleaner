{
  description = "linkleaner";

  inputs.nixpkgs.url = "github:msfjarvis/nixpkgs/nixpkgs-unstable";

  inputs.systems.url = "github:msfjarvis/flake-systems";

  inputs.advisory-db.url = "github:rustsec/advisory-db";
  inputs.advisory-db.flake = false;

  inputs.crane.url = "github:ipetkov/crane";

  inputs.devshell.url = "github:numtide/devshell";
  inputs.devshell.inputs.nixpkgs.follows = "nixpkgs";

  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.flake-utils.inputs.systems.follows = "systems";

  inputs.flake-compat.url = "github:nix-community/flake-compat";
  inputs.flake-compat.flake = false;

  outputs = {
    self,
    nixpkgs,
    advisory-db,
    crane,
    devshell,
    fenix,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [devshell.overlays.default];
      };

      rustNightly = (import fenix {inherit pkgs;}).fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-JZmAwvZMqxCL1f5jzPeho7ZoaaesUvIvXEf4gAtC5Mg=";
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustNightly;
      commonArgs = {
        src = craneLib.cleanCargoSource ./.;
        buildInputs = [];
        nativeBuildInputs = [];
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      linkleaner-fmt = craneLib.cargoFmt (commonArgs
        // {
          inherit cargoArtifacts;
        });
      linkleaner-clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit cargoArtifacts;
        });
      linkleaner = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
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
          inherit advisory-db cargoArtifacts;
        });
    in {
      checks = {
        inherit linkleaner linkleaner-audit linkleaner-clippy linkleaner-fmt linkleaner-nextest;
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
      packages.ghContainer = pkgs.dockerTools.buildLayeredImage {
        name = "ghcr.io/msfjarvis/linkleaner";
        tag = "latest";
        created = "now";
        config.Cmd = ["${linkleaner}/bin/linkleaner"];
      };

      apps.default = flake-utils.lib.mkApp {drv = linkleaner;};

      devShells.default = pkgs.devshell.mkShell {
        bash = {interactive = "";};

        env = [
          {
            name = "DEVSHELL_NO_MOTD";
            value = 1;
          }
        ];

        packages = with pkgs; [
          cargo-nextest
          cargo-release
          fenix.packages.${system}.rust-analyzer
          flyctl
          nil
          rustNightly
          stdenv.cc
        ];
      };
    });
}
