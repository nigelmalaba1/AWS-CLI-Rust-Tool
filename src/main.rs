use clap::Parser;
use std::process::Command;


#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    about = "AWS S3 CLI in Rust",
    after_help = "Example: aws-cli"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    List {
        #[clap(short, long)]
        bucket: Option<String>,
    },
    Create {
        #[clap(short, long)]
        bucket: String,
    },
    Upload {
        #[clap(short, long)]
        bucket: String,
        #[clap(short, long)]
        filepath: String,
    },
    Delete {
        #[clap(short, long)]
        bucket: String,
        #[clap(short, long)]
        key: Option<String>,
    },
    Get {
        #[clap(short, long)]
        bucket: String,
        #[clap(short, long)]
        key: String,
    },
    LaunchInstance,
    Download {
        #[clap(short, long)]
        repo: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let client = s3cli::client().await.unwrap();
    // Match on subcommand
    match args.command {
        Some(Commands::Create { bucket }) => {
            let bucket_region = s3cli::bucket_region().await.unwrap();
            s3cli::create_bucket(&client, &bucket, &bucket_region)
                .await
                .unwrap();
        }
        Some(Commands::List { bucket }) => match bucket {
            Some(bucket) => {
                s3cli::list_objects(&client, &bucket).await.unwrap();
            }
            None => {
                s3cli::list_buckets(&client).await.unwrap();
            }
        },
        Some(Commands::Upload { bucket, filepath }) => {
            s3cli::upload_object(&client, &bucket, &filepath)
                .await
                .unwrap();
        }
        Some(Commands::Delete { bucket, key }) => match key {
            Some(key) => {
                s3cli::delete_object(&client, &bucket, &key).await.unwrap();
            }
            None => {
                s3cli::delete_bucket(&client, &bucket).await.unwrap();
            }
        },
        Some(Commands::Get { bucket, key }) => {
            s3cli::get_object(&client, &bucket, &key).await.unwrap();
        }
        Some(Commands::LaunchInstance) => {
            s3cli::request_spot_instance().await.unwrap();
        }

        Some(Commands::Download { repo }) => {
            // Use wget to download the binary from GitHub
            let output = Command::new("wget")
                .arg("-O")
                .arg("quantized-cpu")
                //.arg(&format!("https://github.com/athletedecoded/cookbook-binaries/raw/main/binaries/cpu/quantized?download=", repo))
                .arg(&format!("https://github.com/athletedecoded/cookbook-binaries/raw/main/binaries/cpu/quantized?download={}", repo))
                .output()
                .expect("Failed to download binary from GitHub");
            println!("{:?}", output);
let stdout = String::from_utf8_lossy(&output.stdout);
let stderr = String::from_utf8_lossy(&output.stderr);

println!("STDOUT: {}", stdout);
println!("STDERR: {}", stderr);
        }

        None => {
            println!("No subcommand was used");
        }
        // None => {
        //     println!("No subcommand was used");
        // }
    }
}
