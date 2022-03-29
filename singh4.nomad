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
      driver = "docker"

      config {
        image      = "natto17/singh4:latest"
        force_pull = true
        volumes = [ "/tmp:/tmp" ]
      }

      template {
        data = <<EOF
{{with secret "kv/data/singh4/discord"}}
DISCORD_TOKEN="{{.Data.data.token}}"
{{end}}
RUST_BACKTRACE=1
EOF

        destination = "${NOMAD_SECRETS_DIR}/data.env"
        env         = true
      }
    }
  }
}
