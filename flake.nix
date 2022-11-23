{
  description = "walls-bot-rs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
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

        rustNightly = pkgs.rust-bin.nightly."2022-11-14".default.override {
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

        tgbot = craneLib.buildPackage {
          inherit src;
          doCheck = false;
        };
      in {
        checks = { inherit tgbot; };

        # Run clippy (and deny all warnings) on the crate source,
        # again, resuing the dependency artifacts from above.
        #
        # Note that this is done as a separate derivation so that
        # we can block the CI if there are issues here, but not
        # prevent downstream consumers from building our crate by itself.
        tgbot-clippy = craneLib.cargoClippy {
          inherit cargoArtifacts src buildInputs;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        };

        tgbot-doc = craneLib.cargoDoc { inherit cargoArtifacts src; };

        # Check formatting
        tgbot-fmt = craneLib.cargoFmt { inherit src; };

        # Audit dependencies
        tgbot-audit = craneLib.cargoAudit { inherit src advisory-db; };

        # Run tests with cargo-nextest
        # Consider setting `doCheck = false` on `tgbot` if you do not want
        # the tests to run twice
        tgbot-nextest = craneLib.cargoNextest {
          inherit cargoArtifacts src buildInputs;
          partitions = 1;
          partitionType = "count";
        };

        packages.default = tgbot;

        apps.default = flake-utils.lib.mkApp { drv = tgbot; };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          nativeBuildInputs = with pkgs; [ cargo-release rustNightly ];
        };
      });
}
