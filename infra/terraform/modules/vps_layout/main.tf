terraform {
  required_version = ">= 1.6.0"
  required_providers {
    null = {
      source  = "hashicorp/null"
      version = "~> 3.2"
    }
  }
}

resource "null_resource" "bootstrap" {
  triggers = {
    layout_version = var.layout_version
  }

  connection {
    type        = "ssh"
    host        = var.vps_host
    port        = var.vps_port
    user        = var.vps_user
    private_key = file(var.private_key_path)
  }

  provisioner "remote-exec" {
    inline = [
      "set -eu",
      "sudo mkdir -p /opt/uamappers/prod/.state",
      "sudo chown -R ${var.vps_user}:${var.vps_user} /opt/uamappers",
      "docker network inspect traefik-public >/dev/null 2>&1 || docker network create traefik-public"
    ]
  }
}
