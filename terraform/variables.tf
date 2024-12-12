variable "region" {
  description = "The AWS region to deploy to"
  type        = string
  default     = "us-west-2"
}

variable "lambda_function_name" {
  description = "The name of the Lambda function"
  type        = string
  default     = "backend"
}

variable "api_gateway_name" {
  description = "The name of the API Gateway"
  type        = string
  default     = "backend_api"
}

variable "stage_name" {
  description = "The name of the deployment stage"
  type        = string
  default     = "prod"
}

variable "lambda_role_name" {
  description = "The name of the Lambda execution role"
  type        = string
  default     = "lambda_exec_role"
}

variable "lambda_runtime" {
  description = "The runtime environment for the Lambda function"
  type        = string
  default     = "provided.al2"
}

variable "lambda_handler" {
  description = "The handler for the Lambda function"
  type        = string
  default     = "bootstrap"
}

variable "lambda_environment_variables" {
  description = "Environment variables for the Lambda function"
  type        = map(string)
  default     = {
    RUST_BACKEND = "true"
  }
}
