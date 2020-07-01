with import <nixpkgs> { }; mkShell { buildInputs = [ rustup pkgconfig openssl ]; }
