{
  inputs = {
    xnode-manager.url = "github:Openmesh-Network/xnode-manager";
    subdomain-distributor.url = "github:Openmesh-Network/subdomain-distributor";
    nixpkgs.follows = "subdomain-distributor/nixpkgs";
  };

  outputs = inputs: {
    nixosConfigurations.container = inputs.nixpkgs.lib.nixosSystem {
      specialArgs = {
        inherit inputs;
      };
      modules = [
        inputs.xnode-manager.nixosModules.container
        {
          services.xnode-container.xnode-config = {
            host-platform = ./xnode-config/host-platform;
            state-version = ./xnode-config/state-version;
            hostname = ./xnode-config/hostname;
          };
        }
        inputs.subdomain-distributor.nixosModules.default
        (
          { pkgs, lib, ... }:
          {
            environment.systemPackages = [
              pkgs.dig
            ];

            services.resolved.enable = lib.mkForce false;

            services.subdomain-distributor = {
              enable = true;
              domain = "openmesh.cloud";
              soa = {
                nameserver = "dns.openmesh.network";
                mailbox = "samuel.mens@openmesh.network";
              };
            };
          }
        )
      ];
    };
  };
}
