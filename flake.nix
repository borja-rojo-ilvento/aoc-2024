/*# Advent of Code 2024

This flake manages the development of the advent of code 2024 solutions.

## Description

I am using this to learn more about nix and rust development. This file will
be in flux as I make changes to aspects of this projet in leu of being a novice
in these two tools.

## Features

- Deploy a dev shell with all the tools needed to develop this project
- Builds binaries in the `src/bin` directory
- Run the binaries built using the build tool
*/
{
  description = "Advent of Code 2024";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }: 
	flake-utils.lib.eachDefaultSystem (system:
	let
		overlays = [
			rust-overlay.overlays.default
			(final: prev: {
				rustToolchain = final.rust-bin.stable.latest.default.override {
					extensions = [ "rust-src" "rust-analyzer" ];
					targets = [ "x86_64-unknown-linux-gnu" ];
				};
			})
		];
	
		pkgs = import nixpkgs {
			inherit system overlays;
			config.allowUnfree = true;
		};

		rustToolChain = pkgs.rustToolchain;
		buildRustPackage = name:
			pkgs.rustPlatform.buildRustPackage {
				inherit name;
				src = ./.;
				cargoLock.lockFile = ./Cargo.lock;
				buildType = "release";
				buildInputs = /*with pkgs;*/ [];
				cargoLock.outputHashes = {};
		};

		execs = {
			day-1 = buildRustPackage "day-1";
			day-2 = buildRustPackage "day-2";
		};
	in
	{
		devShells.default = pkgs.mkShell {
			buildInputs = with pkgs; [
				code-cursor
				rustToolChain
				cargo
				rust-analyzer
				rustfmt
				clippy
				nil
			];
		};

		packages = execs;
		apps = builtins.mapAttrs (name: package: {
				type = "app";
				program = "${package}/bin/${name}";
		}) execs;
	});
}
