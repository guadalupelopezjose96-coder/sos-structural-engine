terraform {
  required_version = ">= 1.6.0"
}

locals {
  organization_id     = "320717657398"
  organization_handle = "guadalupelopezjose96-org"

  structural_folders = [
    "01-Shared-Services",
    "02-Production",
    "03-Non-Production",
    "04-Sandbox"
  ]

  corporate_domains = {
    "AuraTetrax-com"       = "877771073297"
    "Fourphonix-com"       = "964521086569"
    "AuraIntelCore-com"    = "188680247367"
    "AuraIntel-Global-com" = "1087754464927"
    "Standard1979-com"     = "282230018778"
    "Prophonix-com"        = "358233233"
  }

  plane_cidr_allocations = {
    control_plane       = "10.100.0.0/18"
    data_plane_vpp      = "10.100.64.0/18"
    fiscal_plane_aaufsa = "10.100.128.0/18"
  }
}
