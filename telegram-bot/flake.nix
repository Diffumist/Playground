{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nmattia/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-secrets = {
      url = "/home/diffumist/Documents/Project/nix-secrets";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, naersk, flake-utils, nix-secrets, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlay ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        naersk-lib = pkgs.callPackage naersk { };
        name = "dmist-bot";
        inherit (nix-secrets.secrets.telegram-bot) token;
      in
      rec {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
            pkgconfig
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "clippy" "cargo" "rustfmt" ];
            })
          ];
          shellHook = ''
            export TELOXIDE_TOKEN=${token}
          '';
        };

        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          buildInputs = with pkgs; [ pkg-config openssl ];
        };

        defaultApp = flake-utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        nixosModule = { config, ... }:
          with pkgs.lib;
          {
            options = {
              services.${name} = {
                enable = mkEnableOption "enable telegram bot";
                configFile = mkOption {
                  type = types.path;
                  default = null;
                  example = ''
                    writeTextFile '''
                      TELOXIDE_TOKEN="telegram token"
                    ''';
                  '';
                };
              };
            };
            config = mkIf config.services.${name}.enable {
              systemd.services.${name} = {
                wantedBy = [ "multi-user.target" ];
                after = [ "network.target" ];
                script = ''
                  . ${config.services.${name}.config}
                  ${self.defaultPackage."${system}"}/bin/${name}
                '';
              };

            };
          };
      }
    );
}

