extern crate rusoto_core;
extern crate futures;
extern crate rusoto_credential;
extern crate rusoto_s3;

use rusoto_core::Region;
use rusoto_s3::{S3Client, GetObjectRequest, S3};
use rusoto_credential::{EnvironmentProvider};
use futures::stream::Stream;

fn main() {
    let client = S3Client::simple(Region::UsEast1); 
    let request = GetObjectRequest { 
        bucket: "eden-wingit-test".to_string(),
        key: "tmux.conf".to_string(),
        ..Default::default()
    };
    println!("{}",request.bucket);
    let buf = client
        .get_object(&request)
        .sync()
        .unwrap()
        .body
        .unwrap()
        .wait()
        .last()
        .unwrap()
        .unwrap();

    let output = String::from_utf8(buf).unwrap();

    println!("{}",output);
    println!("Hello, world!");
}
