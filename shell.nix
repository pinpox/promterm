{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [

    # Rust toolchain
    rustup
    # rust-analyzer

    # SSL
    openssl

    # Other dependencies
    pkg-config
    # zlib
    # sqlite
    delta

    # pre-commit
    # mysql-client
    # postgresql
    # mariadb
    # mariadb-connector-c
    # zlib
  ];
}
