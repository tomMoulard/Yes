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
