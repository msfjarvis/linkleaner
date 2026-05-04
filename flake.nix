{
  description = "zeppelinker";

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

  inputs.flake-compat.url = "git+https://git.lix.systems/lix-project/flake-compat";
  inputs.flake-compat.flake = false;

  outputs =
    {
      nixpkgs,
      advisory-db,
      crane,
      devshell,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlays.default ];
        };

        rustNightly = (import fenix { inherit pkgs; }).fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-vra6TkHITpwRyA5oBKAHSX0Mi6CBDNQD+ryPSpxFsfg=";
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustNightly;
        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          buildInputs = [ ];
          nativeBuildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.libiconv ];
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        zeppelinker-fmt = craneLib.cargoFmt (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );
        zeppelinker-clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );
        zeppelinker = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            doCheck = false;
          }
        );
        zeppelinker-nextest = craneLib.cargoNextest (
          commonArgs
          // {
            cargoArtifacts = zeppelinker;
            partitions = 1;
            partitionType = "count";
          }
        );
        zeppelinker-audit = craneLib.cargoAudit (
          commonArgs
          // {
            inherit advisory-db cargoArtifacts;
          }
        );
      in
      {
        checks = {
          inherit
            zeppelinker
            zeppelinker-audit
            zeppelinker-clippy
            zeppelinker-fmt
            zeppelinker-nextest
            ;
        };

        # Expose the flyctl and skopeo packages for use in CI
        packages = { inherit (pkgs) flyctl skopeo; };
        packages.default = zeppelinker;
        packages.container = pkgs.dockerTools.buildImage {
          name = "registry.fly.io/zeppelinker";
          tag = "latest";
          created = "now";
          copyToRoot = pkgs.buildEnv {
            name = "zeppelinker";
            paths = [ zeppelinker ];
            pathsToLink = [ "/bin" ];
          };
          config.Cmd = [ "${zeppelinker}/bin/zeppelinker" ];
        };
        packages.ghContainer = pkgs.dockerTools.buildLayeredImage {
          name = "ghcr.io/msfjarvis/zeppelinker";
          tag = "latest";
          created = "now";
          config.Cmd = [ "${zeppelinker}/bin/zeppelinker" ];
        };

        apps.default = flake-utils.lib.mkApp { drv = zeppelinker; };

        devShells.default = pkgs.devshell.mkShell {
          bash = {
            interactive = "";
          };

          env = [
            {
              name = "DEVSHELL_NO_MOTD";
              value = 1;
            }
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            {
              name = "LIBRARY_PATH";
              prefix = pkgs.lib.makeLibraryPath commonArgs.nativeBuildInputs;
            }
          ];

          packages = with pkgs; [
            bacon
            cargo-nextest
            cargo-release
            fenix.packages.${system}.rust-analyzer
            flyctl
            git-cliff
            rustNightly
            skopeo
            stdenv.cc
          ];
        };
      }
    );
}
