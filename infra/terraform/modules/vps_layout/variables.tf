variable "vps_host" {
  type = string
}

variable "vps_port" {
  type    = number
  default = 22
}

variable "vps_user" {
  type = string
}

variable "private_key_path" {
  type = string
}

variable "layout_version" {
  type    = string
  default = "v1"
}
