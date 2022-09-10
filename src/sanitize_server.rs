mod generated;

use std::{fs::File, io::{BufReader, BufRead}};

use aho_corasick::AhoCorasick;
use generated::sanitize::{sanitize_service_server::{SanitizeServiceServer, SanitizeService}, SanitizeRequest, SanitizeResponse};
use anyhow::Result;

struct AhoCorasickSanitizor {
    ac: AhoCorasick,
}

#[tonic::async_trait]

impl SanitizeService for AhoCorasickSanitizor {
    async fn sanitize(&self, request: tonic::Request<SanitizeRequest>) -> Result<tonic::Response<SanitizeResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut matches = vec![];
        for mat in self.ac.find_iter(&req.text) {
            matches.push((mat.pattern(), mat.start(), mat.end()));
        }
        let ret = matches.iter().map(|(pattern, start, end)| {
            format!("{}, {}, {}", pattern, start, end)
        }).collect::<Vec<String>>();
        let resp = SanitizeResponse {
            filtered_text: ret.join(":")
        };
        Ok(tonic::Response::new(resp))
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let patterns = load_patterns();
    
    let ac = AhoCorasick::new(patterns);
    let sanitizor = AhoCorasickSanitizor { ac };
    let addr = "[::1]:50051".parse()?;
    let server = SanitizeServiceServer::new(sanitizor);
    tonic::transport::Server::builder()
        .add_service(server)
        .serve(addr)
        .await?
        ;
    Ok(())
}

fn load_patterns() -> Vec<String> {
    let mut patterns = vec![];
    let file = File::open("patterns.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        patterns.push(line);
    }
    patterns
}