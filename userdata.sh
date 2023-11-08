#!/bin/bash

# Update the package database and install the AWS CLI
apt-get update -y
apt-get install -y awscli

# Replace 'your-bucket-name' with the name of your S3 bucket and 'your-file-name' with the name of your file
# Replace 'your-region' with the region of your S3 bucket if necessary
# The home directory for the default 'ubuntu' user on Ubuntu instances is '/home/ubuntu'
aws s3 cp s3://rustcandle/whisper /home/ubuntu/whisper --region us-east-1

# Ensure the ownership is correct (replace 'ubuntu' with the appropriate username if different)
chown ubuntu:ubuntu /home/ubuntu/whisper
