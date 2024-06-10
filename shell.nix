{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell
{
  nativeBuildInputs = with pkgs; [
    # Rust tools
    cargo
    rustc
    rust-analyzer
    rustfmt

    # Other tools
    git
  ];



  shellHook = ''
  '';
}