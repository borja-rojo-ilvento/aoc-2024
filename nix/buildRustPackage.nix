{rustypkgs, name, path, bin ? name }:
rustypkgs.rustPlatform.buildRustPackage {
  inherit name;
  src = ./${path};
  cargoLock.lockFile = ./${path}/Cargo.lock;
  buildType = "release";
  buildInputs = with rustypkgs; [];
}

