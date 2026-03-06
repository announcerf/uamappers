terraform {
  required_version = ">= 1.6.0"
}

module "vps_layout" {
  source = "../../modules/vps_layout"

  vps_host         = var.vps_host
  vps_port         = var.vps_port
  vps_user         = var.vps_user
  private_key_path = var.private_key_path
  layout_version   = var.layout_version
}
