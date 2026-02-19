let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  packages = with pkgs; [
    rustup
    cargo
    redocly
    openapi-generator-cli
    python312
    stdenv.cc
  ];
  nativeBuildInputs =
    with pkgs;
    [
      # Rust toolchain
      rustc
      cargo
      rustup
      pkg-config
    ];
  buildInputs =
    with pkgs;
    [ 
      openssl
      # Add these dependencies for bundling
    ]
    ++ lib.optionals stdenv.hostPlatform.isLinux [
      # Required for most applications
      glib-networking
    ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [
      darwin.apple_sdk.frameworks.WebKit
    ];
  
  shellHook = ''
    rustup default 1.93.0
    rustup component add rust-src
    export PATH=/home/$(whoami)/.cargo/bin:$PATH
  '';
}
