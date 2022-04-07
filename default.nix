let
    pkgs = import <nixpkgs> {};
    sources = import ./nix/sources.nix;
    naersk = pkgs.callPackage sources.naersk {};
in 
    naersk.buildPackage {
        # This dot at the end is .... strange!
        src = ./.;
        name="kbtools"  
    }