{rustypkgs, name, path, bin ? name }:
rustypkgs.rustPlatform.buildRustPackage {
  inherit name;
  src = ./${path};
  cargoLock.lockfile = ./${path}/Cargo.lock;
  buildType = "release";
  buildInputs = with pkgs; []
}

