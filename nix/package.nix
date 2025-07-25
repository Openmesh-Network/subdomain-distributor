{ rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "subdomain-distributor";
  version = "1.0.0";
  src = ../rust-app;

  cargoLock = {
    lockFile = ../rust-app/Cargo.lock;
  };

  doDist = false;

  meta = {
    mainProgram = "subdomain-distributor";
  };
}
