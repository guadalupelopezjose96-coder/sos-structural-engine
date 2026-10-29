# =============================================================================
# ORGANIZACIÓN: guadalupelopezjose96-org (ID: 320717657398)
# ARQUITECTURA: Master Blueprint OpenTofu - Infraestructura Zero-Trust Multi-Plano
# NORMATIVA: NIST SP 800-53 Rev. 5 (SC-2, SC-3, SC-7), SOC 2 Tipo II, AAUFSA 2026
# AUTOR: José Emanuel Guadalupe López <arquitectura@guadalupelopezjose96-org.internal>
# =============================================================================

terraform {
  required_version = ">= 1.6.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.5"
    }
  }
}

variable "aws_region" {
  type        = string
  default     = "us-east-1"
  description = "Región AWS primaria de despliegue para el System-of-Systems"
}

variable "environment" {
  type        = string
  default     = "production"
  description = "Entorno de ejecución inmutable"
}

provider "aws" {
  region = var.aws_region
  default_tags {
    tags = {
      Organization = "guadalupelopezjose96-org"
      OrgID        = "320717657398"
      Environment  = var.environment
      Engine       = "sos-structural-engine"
      Compliance   = "AAUFSA-IRC-56-NIST-800-53"
      ManagedBy    = "OpenTofu"
    }
  }
}

# -----------------------------------------------------------------------------
# 1. TOPOLOGÍA DE RED SEGREGADA POR PLANOS (NIST SC-7 BULKHEAD)
# -----------------------------------------------------------------------------
resource "aws_vpc" "sos_core_vpc" {
  cidr_block           = "10.100.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    Name = "sos-core-vpc-320717657398"
  }
}

# Subred Plano de Control (Tier 0)
resource "aws_subnet" "subnet_control_plane" {
  vpc_id            = aws_vpc.sos_core_vpc.id
  cidr_block        = "10.100.1.0/24"
  availability_zone = "${var.aws_region}a"

  tags = {
    Name  = "sos-subnet-control-plane"
    Plane = "CONTROL_PLANE"
  }
}

# Subred Plano de Datos VPP (Tier 1)
resource "aws_subnet" "subnet_data_plane_vpp" {
  vpc_id            = aws_vpc.sos_core_vpc.id
  cidr_block        = "10.100.2.0/24"
  availability_zone = "${var.aws_region}a"

  tags = {
    Name  = "sos-subnet-data-plane-vpp"
    Plane = "DATA_PLANE_VPP"
  }
}

# Subred Plano Fiscal AAUFSA (Tier 2 - Aislamiento Crítico)
resource "aws_subnet" "subnet_fiscal_plane" {
  vpc_id            = aws_vpc.sos_core_vpc.id
  cidr_block        = "10.100.3.0/24"
  availability_zone = "${var.aws_region}a"

  tags = {
    Name  = "sos-subnet-fiscal-plane-aaufsa"
    Plane = "FISCAL_PLANE_AAUFSA"
  }
}

# -----------------------------------------------------------------------------
# 2. SEGURIDAD DE RED Y AISLAMIENTO DE TRÁFICO INVERSO (NIST SC-2 / SC-3)
# -----------------------------------------------------------------------------
resource "aws_security_group" "sg_db_fiscal_plane" {
  name        = "sos-sg-db-fiscal-ledger"
  description = "Aislamiento estricto: Solo permite trafico fiscal y control, rechaza acoplamiento inverso"
  vpc_id      = aws_vpc.sos_core_vpc.id

  ingress {
    description = "Acceso PostgreSQL desde Plano de Control"
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = [aws_subnet.subnet_control_plane.cidr_block]
  }

  ingress {
    description = "Acceso PostgreSQL desde Plano Fiscal Interno"
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = [aws_subnet.subnet_fiscal_plane.cidr_block]
  }

  egress {
    description = "Bloqueo estricto: No permite salida no autorizada a internet"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["10.100.0.0/16"]
  }

  tags = {
    Name = "sos-sg-fiscal-ledger"
  }
}

# -----------------------------------------------------------------------------
# 3. GITHUB ACTIONS OIDC FEDERATION (ZERO-TRUST / SIN CLAVES ESTÁTICAS)
# -----------------------------------------------------------------------------
resource "aws_iam_openid_connect_provider" "github_actions" {
  url             = "https://token.actions.githubusercontent.com"
  client_id_list  = ["sts.amazonaws.com"]
  thumbprint_list = ["6938fd4d98bab03faadb97b34396831e3780aea1", "1c58a3a8518e8759bf075b76b750d4f8d70468ff"]
}

resource "aws_iam_role" "github_actions_ci_role" {
  name = "sos-gh-actions-ci-role-320717657398"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Federated = aws_iam_openid_connect_provider.github_actions.arn
        }
        Action = "sts:AssumeRoleWithWebIdentity"
        Condition = {
          StringEquals = {
            "token.actions.githubusercontent.com:aud" = "sts.amazonaws.com"
          }
          StringLike = {
            "token.actions.githubusercontent.com:sub" = "repo:guadalupelopezjose96-coder/sos-structural-engine:*"
          }
        }
      }
    ]
  })

  tags = {
    Purpose = "OIDC CI Deployment Role"
  }
}

# -----------------------------------------------------------------------------
# 4. SALIDAS ESTRUCTURALES EXPORTADAS
# -----------------------------------------------------------------------------
output "vpc_id" {
  value       = aws_vpc.sos_core_vpc.id
  description = "Identificador de la VPC núcleo"
}

output "github_oidc_role_arn" {
  value       = aws_iam_role.github_actions_ci_role.arn
  description = "ARN del rol OIDC federado para GitHub Actions"
}

output "subnet_fiscal_cidr" {
  value       = aws_subnet.subnet_fiscal_plane.cidr_block
  description = "CIDR del plano fiscal inmutable AAUFSA"
}
