with import <nixpkgs> { };
mkShell { buildInputs = [ cargo openssl pkgconfig ]; }
