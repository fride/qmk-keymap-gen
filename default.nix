let
    # pkgs = import <nixpkgs> {};
    sources = import ./nix/sources.nix;
    nixpkgs-mozilla = import sources.nixpkgs-mozilla;
    pkgs = import <nixpkgs> {
      overlays =
      [
        nixpkgs-mozilla
        (self: super:
            {
              rustc = self.latest.rustChannels.nightly.rust;
              cargo = self.latest.rustChannels.nightly.rust;
            }
        )
      ];
    };
    naersk = pkgs.callPackage sources.naersk {};
in 
    naersk.buildPackage {
        # This dot at the end is .... strange!
        src = ./.;
        name="kbtools";
    }