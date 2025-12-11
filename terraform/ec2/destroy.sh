#!/bin/bash
# Destroy all terraform workspaces and their resources
# run from `terraform/ec2` directory

for ws in $(terraform workspace list | grep -v default | tr -d ' *'); do
    echo "========================================"
    echo "Destroying workspace: $ws"
    echo "========================================"
    terraform workspace select "$ws"
    terraform destroy -auto-approve
    echo "Deleting workspace: $ws"
    terraform workspace select default
    terraform workspace delete "$ws"
done

echo ""
echo "========================================"
echo "EC2 workspaces destroyed."
echo "Shared resources (S3, IAM) are still intact."
echo "To destroy shared resources: cd shared && terraform destroy"
echo "========================================"