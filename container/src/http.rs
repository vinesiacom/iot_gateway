use candid::candid_method;
use ic_cdk_macros::query;

use crate::http_types::{HttpRequest, HttpResponse, HttpResponseBuilder};

#[query]
#[candid_method(query)]
fn http_request(req: HttpRequest) -> HttpResponse {
    if req.path() == "/health" {
        // let mut writer =
        //     ic_metrics_encoder::MetricsEncoder::new(vec![], ic_cdk::api::time() as i64 / 1_000_000);
        // match encode_metrics(&mut writer) {
        //     Ok(()) =>
        HttpResponseBuilder::ok()
            .header("Content-Type", "text/plain; version=0.0.4")
            //         .with_body_and_content_length(writer.into_inner())
            .build()
        //     Err(err) => {
        //         HttpResponseBuilder::server_error(format!("Failed to encode metrics: {}", err))
        //             .build()
        //     }
        // }
    } else if req.path() == "/api/v2/query" {
        // let dashboard: Vec<u8> = build_dashboard();
        HttpResponseBuilder::ok()
            .header("Content-Type", "text/html; charset=utf-8")
            //     .with_body_and_content_length(dashboard)
            .build()
    } else {
        HttpResponseBuilder::not_found().build()
    }
}
