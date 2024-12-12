output "lambda_function_name" {
  description = "The name of the Lambda function"
  value       = var.lambda_function_name
}

output "api_gateway_name" {
  description = "The name of the API Gateway"
  value       = var.api_gateway_name
}

output "stage_name" {
  description = "The name of the deployment stage"
  value       = var.stage_name
}

output "api_url" {
  description = "The URL of the API Gateway"
  value       = aws_api_gateway_deployment.api.invoke_url
}
