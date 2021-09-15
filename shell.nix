with import <nixpkgs> { };
mkShell { buildInputs = [ clang_11 cargo glibc lld_11 pkgconfig rustc ]; }
