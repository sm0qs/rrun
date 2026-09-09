{ pkgs, ... }:

{
  env = {
    RUST_LOG = "trace";
    CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L native=${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib";
  };

  packages = [
    pkgs.pkgsCross.mingwW64.stdenv.cc
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "x86_64-pc-windows-gnu" ];
  };
}
