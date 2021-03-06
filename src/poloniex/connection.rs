


use iron::prelude::*;
use iron::status;
use url::form_urlencoded;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::digest::Digest;
use crypto::sha2::Sha512;

use std::io;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use std::io::{BufRead};
use std::ptr::null;

use rustc_serialize::hex::ToHex;
use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use hyper::Client;
use hyper::header::Connection;
use hyper::header::Headers;







pub fn apiConnect(apikey: String, secretkey: &str, parameters: String) -> String {
  let the_secret_bytes = secretkey.as_bytes();

  let param_string = parameters;
  let params_clone = param_string.clone();
  //hmac-sha512 signature of uri
  let the_sha = Sha512::new();
  let the_base_key = the_secret_bytes;
  let mut the_new_mac = Hmac::new(the_sha, the_base_key);
  the_new_mac.input(param_string.as_bytes());
  let the_signature_string =  the_new_mac.result().code().to_hex().to_string();
  println!("{:?}", &the_signature_string);

  
    let mut headers = Headers::new();

    headers.set_raw("Sign", vec![the_signature_string.as_bytes().to_vec()]);
    headers.set_raw("Key", vec![apikey.as_bytes().to_vec()]);
    headers.set_raw("Content-Type", vec![b"application/x-www-form-urlencoded".to_vec()]);

    println!("{:?}", headers.len());
  let mut client = Client::new();


    // Creating an outgoing request.

     let mut res = client.post("https://poloniex.com/tradingApi")
                    .body(&params_clone)
                    .headers(headers)
                    .send()
                    .unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
    body
}