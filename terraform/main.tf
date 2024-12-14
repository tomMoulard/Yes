terraform {
  required_version = ">= 0.12"
  backend "s3" {
    bucket = "my-terraform-state"
    key    = "path/to/my/key"
    region = "us-west-2"
  }
}

provider "aws" {
  region = var.region
}

module "lambda" {
  source  = "terraform-aws-modules/lambda/aws"
  version = "~> 2.0"

  function_name = var.lambda_function_name
  handler       = "bootstrap"
  runtime       = "provided.al2"
  role          = module.lambda_role.arn
  filename      = "${path.module}/../backend/target/lambda/release/bootstrap.zip"
  source_code_hash = filebase64sha256("${path.module}/../backend/target/lambda/release/bootstrap.zip")

  environment_variables = {
    RUST_BACKEND = "true"
  }
}

module "lambda_role" {
  source  = "terraform-aws-modules/iam/aws//modules/iam-assumable-role"
  version = "~> 4.0"

  name = "lambda_exec_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      }
    ]
  })

  attach_policy_arns = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
}

module "api_gateway" {
  source  = "terraform-aws-modules/apigateway-v2/aws"
  version = "~> 1.0"

  name        = var.api_gateway_name
  description = "API for the backend"

  routes = [
    {
      path        = "/bidding"
      method      = "POST"
      integration = {
        type = "AWS_PROXY"
        uri  = module.lambda.invoke_arn
      }
    }
  ]
}

resource "aws_api_gateway_deployment" "api" {
  depends_on = [module.api_gateway]
  rest_api_id = module.api_gateway.id
  stage_name  = var.stage_name
}

output "api_url" {
  value = aws_api_gateway_deployment.api.invoke_url
}

resource "aws_db_instance" "default" {
  allocated_storage    = 20
  engine               = "postgres"
  engine_version       = "13.3"
  instance_class       = "db.t2.micro"
  name                 = "mydb"
  username             = "foo"
  password             = "bar"
  parameter_group_name = "default.postgres13"
}

resource "aws_security_group" "rds_sg" {
  name        = "rds_sg"
  description = "Allow traffic to RDS"
  vpc_id      = "vpc-123456"

  ingress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

module "rds_proxy" {
  source  = "terraform-aws-modules/rds-proxy/aws"
  version = "~> 1.0"

  name               = "rds_proxy"
  engine_family      = "POSTGRESQL"
  db_instance_identifiers = [aws_db_instance.default.id]
  iam_auth           = true
  role_arn           = module.lambda_role.arn
  vpc_security_group_ids = [aws_security_group.rds_sg.id]
}

resource "aws_security_group" "rds_proxy_sg" {
  name        = "rds_proxy_sg"
  description = "Allow traffic to RDS Proxy"
  vpc_id      = "vpc-123456"

  ingress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
