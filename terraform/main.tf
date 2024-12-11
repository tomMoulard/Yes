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
