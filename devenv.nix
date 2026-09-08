{ ... }:

{
  env.RUST_LOG = "trace";

  languages.rust = {
    enable = true;
  };
}
