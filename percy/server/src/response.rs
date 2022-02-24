use crate::mime::MimeType;
use crate::ssr::ServerSideRender;
use crate::HTTP_ROOT;
use anyhow::Result;
use parsed::http::{Header, Response};
use std::fs::{read_to_string, read};
use std::path::Path;

/// Get raw file content
pub fn file(path: &str, req_path: &str, ssr: u8, init: Option<u32>) -> Result<(Response, Option<Vec<u8>>)> {
    let path_str = format!("{}{}", HTTP_ROOT, path);
    let path = Path::new(&path_str);
    if path.exists() {
        if let Some(ext) = path.extension() {
            if ext.to_str().unwrap().eq("wasm") {
                let content = read(path)?;
                return Ok((Response {
                    code: 200,
                    message: "OK".to_string(),
                    protocol: "HTTP/1.0".to_string(),
                    headers: vec![
                        Header {
                            name: "content-type".to_string(),
                            value: "application/wasm".to_string(),
                        },
                        Header {
                            name: "content-length".to_string(),
                            value: content.len().to_string(),
                        },
                    ],
                    content: vec![],
                }, Some(content)));
            }
        }
        
        let content_type: String = match path.extension() {
            Some(ext) => MimeType::from_ext(ext.to_str().get_or_insert("")),
            None => MimeType::from_ext(""),
        }
        .get();
        let content = read_to_string(path)?;
        let content = match ssr {
            1 => ServerSideRender::percy(content, req_path.to_string(), init),
            _ => content,
        };
        Ok((Response {
            code: 200,
            message: "OK".to_string(),
            protocol: "HTTP/1.0".to_string(),
            headers: vec![
                Header {
                    name: "content-type".to_string(),
                    value: content_type,
                },
                Header {
                    name: "content-length".to_string(),
                    value: content.len().to_string(),
                },
            ],
            content: content.into_bytes(),
        }, None))
    } else {
        Ok((Response {
            protocol: "HTTP/1.0".to_string(),
            code: 404,
            message: "Not Found".to_string(),
            headers: vec![Header {
                name: "Content-Type".to_string(),
                value: "text/plain".to_string(),
            }],
            content: vec![],
        }, None))
    }
}

/// Bad Request
pub fn bad_request() -> Response {
    Response {
        protocol: "HTTP/1.0".to_string(),
        code: 400,
        message: "Bad Request".to_string(),
        headers: vec![],
        content: vec![],
    }
}

// /// Request Entity Too Large
// pub fn too_big() -> Response {
//     Response {
//         protocol: "HTTP/1.0".to_owned(),
//         code: 413,
//         message: "Request Entity Too Large".to_owned(),
//         headers: vec![],
//         content: vec![],
//     }
// }

/// Internal Server Error
pub fn internal_error() -> (Response, Option<Vec<u8>>) {
    (Response {
        protocol: "HTTP/1.0".to_owned(),
        code: 500,
        message: "Internal Server Error".to_owned(),
        headers: vec![],
        content: vec![],
    }, None)
}
