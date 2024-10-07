{
  default,
  mkShell,
  toolchain,
}:
mkShell {
  env =
    default.env
    // {
      # Rust Analyzer needs to be able to find the path to default crate
      # sources, and it can read this environment variable to do so. The
      # `rust-src` component is required in order for this to work.
      RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
    };

  # Development tools
  nativeBuildInputs =
    [
      toolchain
    ]
    ++ (with default;
      nativeBuildInputs
      ++ propagatedBuildInputs
      ++ buildInputs);
}
