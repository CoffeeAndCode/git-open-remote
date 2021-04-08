{
  inputs = {
    naersk.url = "github:nmattia/naersk";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in rec {
        # `nix build`
        packages.git-remote-open = naersk-lib.buildPackage {
          pname = "git-remote-open";
          root = ./.;
        };
        defaultPackage = packages.git-remote-open;

        # `nix run`
        apps.git-remote-open =
          utils.lib.mkApp { drv = packages.git-remote-open; };
        defaultApp = apps.git-remote-open;

        # `nix develop`
        devShell =
          pkgs.mkShell { nativeBuildInputs = with pkgs; [ cargo rustc ]; };
      });
}
