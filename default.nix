with import <nixpkgs> { };
mkShell { buildInputs = [ clang_10 lld_10 rustup pkgconfig openssl ]; }
