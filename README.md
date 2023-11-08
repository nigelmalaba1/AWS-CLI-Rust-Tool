

# LLM Rust CLI Tool

- AWS S3 CLI using Rust which supports CRUD operations for buckets and objects.
- Rust CLI that downloads LLM binaries as artifacts from Github and runs them locally
- Rust CLI that spins up an AWS spot instance to run a Large Language Model


![image](./assets/s3-cli.png)

## AWS CLI Setup

1. Create an [AWS IAM User Policy for S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/security-iam-awsmanpol.html)

2. Configure your [~/.aws/credentials file](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html#cli-configure-files-where) with environment variables: `aws_access_key_id`, `aws_secret_access_key` and `region`

## Usage

## Download LLM binary as an artifact in a Github Repository
```
cargo run -- download --repo <REPO>
```

**Create a Deep Learning Base AMI Spot Instance**
```
$ cargo run -- launch-instance
```
**Launch AWS Spot Instance with User Data**
```
cargo run -- launch-instance --user-data "$(cat userdata.b64)" 
```

**List all S3 buckets**
```
$ cargo run list
```

**List all objects in a specified S3 bucket**
```
$ cargo run list --bucket <bucket_name>
# ex: cargo run list --bucket ids721
```

**Create new S3 bucket**
```
$ cargo run create --bucket <bucket_name>
# ex: cargo run create --bucket ids721
```

**Upload an object to an S3 bucket**

*NB: Will create bucket if DNE*
```
$ cargo run upload --bucket <bucket_name> --filepath <path_to_file>
# ex: cargo run upload --bucket ids721 --filepath ./test/test.png
```

**Delete an object from an S3 bucket**
```
$ cargo run delete --bucket <bucket_name> --key <object_key>
# ex: cargo run delete --bucket ids721 --key test.png
```

**Delete an empty S3 bucket**
```
$ cargo run delete --bucket <bucket_name>
# ex: cargo run delete --bucket ids721
```

**Get an object from an S3 bucket**
```
$ cargo run get --bucket <bucket_name> --key <object_key>
# ex: cargo run get --bucket ids721 --key test.jpg
```


## CI/CD

Github Actions configured in [.github/workflows/rust.yml](.github/workflows/rust.yml](https://github.com/nigelmalaba1/AWS-CLI-Rust-Tool/blob/master/.github/workflows/deploy.yml)

**Build Executable**
```
$ make release
```


## Progress Log

- [x] Create an [AWS IAM User Policy for S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/security-iam-awsmanpol.html)
- [x] Configure Github Codespaces with [AWS Toolkit Credential Profile](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/setup-credentials.html)
- [x] Initialise Rust project with [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust)
- [x] Establish basic AWS client connection to list S3 buckets
- [x] Add clap command line parsing for arguments (bucket name, local file name)
- [x] Bucket commands: list, create new, check if exists, delete if empty
- [x] Object commands: list objects in bucket, upload to existing bucket, upload to new bucket, delete
- [x] CI/CD with Github Actions

## References

* [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust)
* [AWS Toolkit Credential Profile](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html#cli-configure-files-where)
* [AWS Credentials for VS Code](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/setup-credentials.html)
* [AWS IAM User Policy for S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/security-iam-awsmanpol.html)
* https://github.com/athletedecoded/rust-s3-cli 
