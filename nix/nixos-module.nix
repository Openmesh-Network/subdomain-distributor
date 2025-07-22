{
  config,
  pkgs,
  lib,
  ...
}:
let
  cfg = config.services.subdomain-distributor;
  subdomain-distributor = pkgs.callPackage ./package.nix { };
in
{
  options = {
    services.subdomain-distributor = {
      enable = lib.mkEnableOption "Enable the rust app";

      hostname = lib.mkOption {
        type = lib.types.str;
        default = "0.0.0.0";
        example = "127.0.0.1";
        description = ''
          The hostname under which the app should be accessible.
        '';
      };

      port = lib.mkOption {
        type = lib.types.port;
        default = 42923;
        example = 42923;
        description = ''
          The port under which the app should be accessible.
        '';
      };

      verbosity = lib.mkOption {
        type = lib.types.str;
        default = "warn";
        example = "info";
        description = ''
          The logging verbosity that the app should use.
        '';
      };

      domain = lib.mkOption {
        type = lib.types.str;
        example = "openmesh.cloud";
        description = ''
          The domain to issue subdomains of.
        '';
      };

      ttl = lib.mkOption {
        type = lib.types.str;
        default = "3600";
        description = ''
          A 32 bit signed integer that specifies the time interval that the resource record may be cached before the source of the information should again be consulted. Zero values are interpreted to mean that the RR can only be used for the transaction in progress, and should not be cached.  For example, SOA records are always distributed with a zero TTL to prohibit caching.  Zero values can also be used for extremely volatile data.
        '';
      };

      soa = {
        nameserver = lib.mkOption {
          type = lib.types.str;
          example = "dns.openmesh.network";
          description = ''
            The nameserver pointing to this machine.
          '';
        };

        mailbox = lib.mkOption {
          type = lib.types.str;
          example = "samuel.mens@openmesh.network";
          description = ''
            The mailbox of the person responsible for this domain (zone).
          '';
        };

        refresh = lib.mkOption {
          type = lib.types.str;
          default = "7200";
          description = ''
            A 32 bit time interval before the zone should be refreshed.
          '';
        };

        retry = lib.mkOption {
          type = lib.types.str;
          default = "3600";
          description = ''
            A 32 bit time interval that should elapse before a failed refresh should be retried.
          '';
        };

        expire = lib.mkOption {
          type = lib.types.str;
          default = "1209600";
          description = ''
            A 32 bit time value that specifies the upper limit on the time interval that can elapse before the zone is no longer authoritative.
          '';
        };

        minimumTTL = lib.mkOption {
          type = lib.types.str;
          default = cfg.ttl;
          description = ''
            The unsigned 32 bit minimum TTL field that should be exported with any RR from this zone.
          '';
        };
      };

      dataDir = lib.mkOption {
        type = lib.types.path;
        default = "/var/lib/subdomain-distributor";
        example = "/var/lib/subdomain-distributor";
        description = ''
          The main directory to store data.
        '';
      };

      zonesDir = lib.mkOption {
        type = lib.types.path;
        default = "${cfg.dataDir}/zones";
        example = "/var/lib/subdomain-distributor/zones";
        description = ''
          The directory to store claimed zones.
        '';
      };

      forwardDNS = {
        enable = lib.mkOption {
          type = lib.types.bool;
          default = false;
          description = ''
            Whether to forward internal request to a local DNS server.
          '';
        };

        localDNS = lib.mkOption {
          type = lib.types.nullOr (
            lib.types.enum [
              "dnsmasq"
            ]
          );
          default = null;
          example = "dnsmasq";
          description = ''
            Local DNS server implementation to move to a different port for forwarding.
          '';
        };
      };

      openFirewall = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = ''
          Whether to open ports in the firewall for this application.
        '';
      };
    };
  };

  config = lib.mkIf cfg.enable {
    users.groups.subdomain-distributor = { };
    users.users.subdomain-distributor = {
      isSystemUser = true;
      group = "subdomain-distributor";
    };

    systemd.services.subdomain-distributor = {
      wantedBy = [ "multi-user.target" ];
      description = "Allow anyone to claim *.subdomain.tld records.";
      after = [ "network.target" ];
      environment = {
        HOSTNAME = cfg.hostname;
        PORT = toString cfg.port;
        RUST_LOG = cfg.verbosity;
        DOMAIN = cfg.domain;
        TTL = cfg.ttl;
        SOANAMESERVER = cfg.soa.nameserver;
        SOAMAILBOX = cfg.soa.mailbox;
        SOARETRY = cfg.soa.retry;
        SOAREFRESH = cfg.soa.refresh;
        SOAEXPIRE = cfg.soa.expire;
        SOAMINIMUMTTL = cfg.soa.minimumTTL;
        DATADIR = cfg.dataDir;
        ZONESDIR = cfg.zonesDir;
      };
      serviceConfig = {
        ExecStart = "${lib.getExe subdomain-distributor}";
        User = "subdomain-distributor";
        Group = "subdomain-distributor";
        StateDirectory = "subdomain-distributor";
      };
    };

    services.dnsmasq.settings.port = lib.mkIf (cfg.forwardDNS.localDNS == "dnsmasq") 5353;
    services.coredns = {
      enable = true;
      config =
        (
          if cfg.forwardDNS.enable then
            ''
              . {
                acl {
                  allow net 192.168.0.0/16 127.0.0.1 ::1
                  block
                }
                forward . 127.0.0.1:5353
              }

            ''
          else
            ''''
        )
        + ''
          ${cfg.domain} {
            auto {
              directory ${cfg.zonesDir}
            }
          }
        '';
    };
    systemd.services.coredns.serviceConfig.DynamicUser = lib.mkForce false;
    systemd.services.coredns.serviceConfig.User = "subdomain-distributor";
    systemd.services.coredns.serviceConfig.Group = "subdomain-distributor";

    networking.firewall = lib.mkIf cfg.openFirewall {
      allowedTCPPorts = [ cfg.port ];
      allowedUDPPorts = [ 53 ];
    };
  };
}
