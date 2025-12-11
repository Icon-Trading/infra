variable "aws_region" {
  description = "AWS region to deploy shared resources"
  type        = string
}

variable "project_name" {
  description = "Project name for resource tagging"
  type        = string
  default     = "binance-latency-test"
}
