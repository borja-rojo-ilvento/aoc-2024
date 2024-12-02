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
	
		pkgs = import nixpkgs { inherit system overlays; };

		rustToolChain = pkgs.rustToolchain;

		execs = {
			day-1 = import (self + "/nix/buildRustPackage.nix") {
				rustypkgs = pkgs;
				name = "day-1";
				path = "bin/day-1";
			};
		};
	in
	{
		devShells.default = pkgs.mkShell {
			buildInputs = with pkgs; [
				rustToolChain
			];
		};

		packages.default = execs.day-1;
	}
	);
}