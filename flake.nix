{
  description = "Rust implementation of the Commune server.";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    rust-manifest.url = "https://static.rust-lang.org/dist/channel-rust-nightly.toml";
    rust-manifest.flake = false;

    # fenix.url = "github:nix-community/fenix";
    # fenix.inputs.nixpkgs.follows = "nixpkgs";
    # fenix.inputs.rust-analyzer-src.follows = "";

    crane.url = "github:ipetkov/crane";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nix-filter.url = "github:numtide/nix-filter";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs: let
    makeScope = pkgs:
      pkgs.lib.makeScope pkgs.newScope (self: {
        inherit inputs;

        craneLib =
          (inputs.crane.mkLib pkgs).overrideToolchain self.toolchain;

        toolchain =
          (pkgs.rust-bin.fromRustupToolchainFile
            ./rust-toolchain.toml)
          .override {
            extensions =
              [
                "rustc"
                "cargo"
                "rust-docs"
                "rustfmt"
                "clippy"
              ]
              ++ (pkgs.lib.importTOML ./rust-toolchain.toml).toolchain.components;
          };

        default = self.callPackage ./nix/default.nix {};

        shell = self.callPackage ./nix/shell.nix {};
      });
  in
    inputs.flake-parts.lib.mkFlake {
      inherit inputs;
    } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem = {
        system,
        pkgs,
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;

          overlays = [
            (import inputs.rust-overlay)
          ];
          config = {};
        };

        packages = {
          default =
            (makeScope pkgs).default;
        };
        devShells.default =
          (makeScope pkgs).shell;
      };
    };
}
