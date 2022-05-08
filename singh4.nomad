job "singh4" {
  region      = "global"
  datacenters = ["nazrin"]
  type        = "service"

  group "svc" {
    count = 1

    network {
      mode = "bridge"
    }

    vault {
      policies = ["singh4-policy"]
    }

    task "bot" {
      driver = "raw_exec"

      config {
        command = "/bin/sh"
        args = [ "-c", <<EOF
nix-store --realise {{+.storepath+}}
nix shell nixpkgs#ffmpeg
{{+.storepath+}}/bin/{{+.binary+}}
EOF
]
      }

      template {
        data = <<EOF
{{with secret "kv/data/singh4/discord"}}
DISCORD_TOKEN="{{.Data.data.token}}"
{{end}}
RUST_BACKTRACE=full
PATH=/run/current-system/sw/bin/
EOF

        destination = "${NOMAD_SECRETS_DIR}/data.env"
        env         = true
      }
    }
  }
}
