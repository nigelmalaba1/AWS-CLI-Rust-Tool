// AWS S3 Configuration
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::model::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Error};
use std::path::Path;
use std::process;
use tokio::fs::File;
use tokio::io::copy;
use rusoto_core::{Region, RusotoError};
use rusoto_ec2::{Ec2, Ec2Client, RequestSpotInstancesRequest, RequestSpotLaunchSpecification};
use rusoto_ec2::RequestSpotInstancesError;
use base64::encode;
//use rusoto_ec2::Instance;
//use aws_sdk_s3::{Client as S3Client};


// Determine AWS region
pub async fn bucket_region() -> Result<String, Error> {
    let region_provider = RegionProviderChain::first_try(None)
        .or_default_provider()
        .or_else("us-west-2");
    let region = region_provider.region().await.unwrap();
    Ok(region.to_string())
}

// Create AWS client
pub async fn client() -> Result<Client, Error> {
    let region_provider = RegionProviderChain::first_try(None)
        .or_default_provider()
        .or_else("us-west-2");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    // println!("{:?}", client);
    Ok(client)
}

// EC2 Spot Instance Configuration
pub async fn request_spot_instance(user_data_script: &str) -> Result<(), RusotoError<RequestSpotInstancesError>> {
    let region = Region::UsEast1;
    let ec2_client = Ec2Client::new(region);

    // Base64 encode the user data script
    let base64_script = encode(user_data_script);
    
    let launch_specification = RequestSpotLaunchSpecification {
        image_id: Some("ami-0c47a507d2c485dff".to_string()), // Replace with your AMI ID.
        instance_type: Some("t2.micro".to_string()),
        user_data: Some(base64_script), // Add the user data script here
        ..Default::default()
    };

    let spot_request = RequestSpotInstancesRequest {
        spot_price: Some("0.01".to_string()), // Max price per instance hour.
        instance_count: Some(1),
        launch_specification: Some(launch_specification),
        ..Default::default()
    };

    // Send the Spot instance request.
    match ec2_client.request_spot_instances(spot_request).await {
        Ok(response) => {
               /* // Extract instance ID from the response
               let spot_instance_request = &response.spot_instance_requests.unwrap()[0];
               let instance_id = spot_instance_request.instance_id.as_ref().unwrap();
   
               // Retrieve the instance details
               let instance = get_instance_details(&ec2_client, instance_id).await?;
               let  public_dns = instance.public_dns_name.unwrap_or_default();*/
             
            println!("Spot instance requested successfully: {:?}", response);
            Ok(())
        } 
        Err(err) => {
            eprintln!("Failed to request Spot instance: {:?}", err);
            Err(err)
        }
    }
}

/* async fn get_instance_details(ec2_client: &Ec2Client, instance_id: &str) -> Result<Instance, RusotoError<RequestSpotInstancesError>> {
    use rusoto_ec2::{DescribeInstancesRequest, Filter};

    let request = DescribeInstancesRequest {
        instance_ids: Some(vec![instance_id.to_string()]),
        ..Default::default()
    };

    let result = ec2_client.describe_instances(request).await
    .map_err(|e| RusotoError::from(e))?;
    let reservations = result.reservations.unwrap_or_default();
    let instances = &reservations[0].instances.unwrap_or_default();
    Ok(instances[0].clone())
} */

/* -----------------------------
    BUCKET FNXNS
-------------------------------- */

// List all buckets
pub async fn list_buckets(client: &Client) -> Result<(), Error> {
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();
    let num_buckets = buckets.len();
    println!("Found {num_buckets} buckets.");
    println!();
    for bucket in buckets {
        println!("{}", bucket.name().unwrap_or_default());
    }

    Ok(())
}

// Check if bucket exists
pub async fn bucket_exists(client: &Client, bucket_name: &str) -> Result<bool, Error> {
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();
    for bucket in buckets {
        if bucket.name().unwrap_or_default() == bucket_name {
            return Ok(true);
        }
    }
    Ok(false)
}

// Create new bucket
pub async fn create_bucket(client: &Client, bucket: &str, region: &str) -> Result<(), Error> {
    // Check if bucket exists
    let exists = bucket_exists(client, bucket).await?;
    if exists {
        println!("Bucket {bucket} already exists.");
        process::exit(1);
    }
    let constraint = BucketLocationConstraint::from(region);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    let _resp = client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket)
        .send()
        .await?;
    println!("Creating bucket named: {bucket} in region: {region}");
    Ok(())
}

// Delete empty bucket
pub async fn delete_bucket(client: &Client, bucket: &str) -> Result<(), Error> {
    let exists = bucket_exists(client, bucket).await?;
    if !exists {
        println!("Bucket {bucket} does not exist.");
        process::exit(1);
    }
    let resp = client.list_objects_v2().bucket(bucket).send().await?;
    let objects = resp.contents().unwrap_or_default();
    let num_objects = objects.len();
    if num_objects != 0 {
        println!("Bucket {bucket} is not empty. Cannot delete.");
        process::exit(1);
    }
    client.delete_bucket().bucket(bucket).send().await?;
    println!("Empty bucket {bucket} deleted.");

    Ok(())
}

/* -----------------------------
    OBJECT FNXNS
--------------------------------*/

// List objects in bucket
pub async fn list_objects(client: &Client, bucket: &str) -> Result<(), Error> {
    // Check if bucket exists
    let exists = bucket_exists(client, bucket).await?;
    if !exists {
        println!("Bucket {bucket} does not exist.");
        process::exit(1);
    }

    // If exists, list objects
    let resp = client.list_objects_v2().bucket(bucket).send().await?;
    let objects = resp.contents().unwrap_or_default();
    let num_objects = objects.len();

    println!("Found {num_objects} objects in bucket {bucket}");
    for object in objects {
        println!("{}", object.key().unwrap_or_default());
    }

    Ok(())
}

// Put object in bucket
pub async fn upload_object(client: &Client, bucket: &str, filepath: &str) -> Result<(), Error> {
    // if bucket doesn't exist, create it
    if !bucket_exists(client, bucket).await? {
        let bucket_region = bucket_region().await.unwrap();
        create_bucket(client, bucket, &bucket_region).await?;
    }

    let body = ByteStream::from_path(Path::new(filepath)).await;
    let key = Path::new(filepath).file_name().unwrap().to_str().unwrap();
    match body {
        Ok(b) => {
            let _resp = client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(b)
                .send()
                .await?;
            println!("Uploaded {key} to {bucket}");
        }
        Err(e) => {
            println!("Got an error uploading object:");
            println!("{e}");
            process::exit(1);
        }
    }

    Ok(())
}

// Delete object from bucket
pub async fn delete_object(client: &Client, bucket: &str, key: &str) -> Result<(), Error> {
    // Check if bucket exists
    let exists = bucket_exists(client, bucket).await?;
    if !exists {
        println!("Bucket {bucket} does not exist.");
        process::exit(1);
    }

    // Check key exists in bucket
    let resp = client.list_objects_v2().bucket(bucket).send().await?;
    let objects = resp.contents().unwrap_or_default();
    let mut key_exists = false;
    for object in objects {
        if object.key().unwrap_or_default() == key {
            key_exists = true;
        }
    }
    if !key_exists {
        println!("Key {key} does not exist in bucket {bucket}");
        process::exit(1);
    }
    // Delete object
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    println!("Object {key} deleted from bucket {bucket}.");

    Ok(())
}

pub async fn get_object(client: &Client, bucket: &str, key: &str) -> Result<(), Error> {
    // Check key exists in bucket
    let resp = client.list_objects_v2().bucket(bucket).send().await?;
    let objects = resp.contents().unwrap_or_default();
    let mut key_exists = false;
    for object in objects {
        if object.key().unwrap_or_default() == key {
            key_exists = true;
        }
    }
    if !key_exists {
        println!("Key {key} does not exist in bucket {bucket}");
        process::exit(1);
    }
    // Get object
    let resp = client.get_object().bucket(bucket).key(key).send().await?;
    // Get image as byte stream from response body
    let fpath = format!("./test/{}", key);
    let mut img_stream = resp.body.into_async_read();
    // Create a file to write the image data to
    let mut tmp_file = File::create(&fpath).await.unwrap();
    // Copy the image data into the file
    let _file_msg = copy(&mut img_stream, &mut tmp_file).await.unwrap();
    println!("Object downloaded to {fpath}.");
    Ok(())
}
