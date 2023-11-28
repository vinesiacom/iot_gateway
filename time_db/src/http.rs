use std::{io::Write};

use candid::candid_method;
use flate2::{write::GzEncoder, Compression};
use ic_cdk_macros::query;

use crate::{
    http_types::{HttpRequest, HttpResponse, HttpResponseBuilder},
    timedb::Entry,
    TIME_DB,
};

fn gzip_string(s: &str) -> std::io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(s.as_bytes())?;
    encoder.finish()
}

#[query]
#[candid_method(query)]
fn http_request(req: HttpRequest) -> HttpResponse {
    if req.path() == "/health" {
        // let mut writer =
        //     ic_metrics_encoder::MetricsEncoder::new(vec![], ic_cdk::api::time() as i64 / 1_000_000);
        // match encode_metrics(&mut writer) {
        //     Ok(()) =>
        let mut response = HttpResponseBuilder::ok();
        response.header("Content-Type", "application/json; charset=utf-8");
        //         .with_body_and_content_length(writer.into_inner())
        response.build()
        //     Err(err) => {
        //         HttpResponseBuilder::server_error(format!("Failed to encode metrics: {}", err))
        //             .build()
        //     }
        // }
    } else if req.path() == "/query" {
        let measurement = req.raw_query_param("measurement");

        let mut body = String::new();

        match measurement {
            Some(measurement) => {
                body = format!("{{\"measurement\":\"{}\"}}", measurement);

                let items = TIME_DB.with(|m| {
                    let mut db = m.borrow_mut();
                    let measure = db.get_measurement(&measurement);

                    let my_data: Vec<Entry> = measure
                        .list_entries()
                        .into_iter()
                        .map(|x| (*x).clone())
                        .collect();

                    // let my_data: Vec<Entry> = vec![];

                    serde_json::to_string(&my_data)
                });

                match items {
                    Ok(items) => {
                        body = format!(
                            "{{\"measurement\":\"{}\", \"data\": {}}}",
                            measurement, items
                        );
                    }
                    _ => body = "Error while processing data".to_string(),
                }
            }
            _ => {}
        }

        let mut response = HttpResponseBuilder::ok();
        response.header("Content-Type", "application/json; charset=utf-8");

        if body.len() > 100 {
            let gziped = gzip_string(&body);

            match gziped {
                Ok(data) => {
                    response.header("Content-Encoding", "gzip");
                    response.with_body_and_content_length(data);
                }
                _ => {
                    response
                        .with_body_and_content_length("Error while compressing data".as_bytes());
                }
            }
        }

        // let dashboard: Vec<u8> = build_dashboard();
        response.build()
    } else {
        HttpResponseBuilder::not_found().build()
    }
}
