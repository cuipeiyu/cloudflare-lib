use cloudflare_lib::abuse_reports::*;
use cloudflare_lib::*;
use std::collections::HashMap;

// RUST_LOG=error,cloudflare_lib=trace,abuse_reports=trace cargo test --test abuse_reports --features with-abuse-reports
#[tokio::test]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = ClientBuilder::new(ApiAuth::default()).build().unwrap();

    let account_id = ::std::env::var("CLOUDFLARE_ACCOUNT_ID").unwrap_or_default();

    test_abuse_reports(&client, account_id).await;

    tracing::info!("all done");
}

async fn test_abuse_reports(client: &Client, account_id: String) {
    let new_one = client
        .abuse_reports_new(
            &account_id,
            "report_param",
            &AbuseReportNewParamsBodyAbuseReportsDmcaReport {
                act: AbuseReportNewParamsBodyAbuseReportsDmcaReportAct::AbuseDmca,
                address1: "x".to_owned(),
                agent_name: "x".to_owned(),
                agree: 1,
                city: "x".to_owned(),
                country: "x".to_owned(),
                email: "email".to_owned(),
                email2: "email2".to_owned(),
                host_notification:
                    AbuseReportNewParamsBodyAbuseReportsDmcaReportHostNotification::Send,
                name: "x".to_owned(),
                original_work: "x".to_owned(),
                owner_notification:
                    AbuseReportNewParamsBodyAbuseReportsDmcaReportOwnerNotification::Send,
                signature: "signature".to_owned(),
                state: "x".to_owned(),
                urls: "urls".to_owned(),
                comments: Some("x".to_owned()),
                company: Some("x".to_owned()),
                reported_country: Some("xx".to_owned()),
                reported_user_agent: Some("x".to_owned()),
                tele: Some("x".to_owned()),
                title: Some("x".to_owned()),
            },
        )
        .await
        .unwrap();
    tracing::info!("new_one result: {:?}", new_one);

    let list = client.abuse_reports_list(&account_id, None).await.unwrap();
    tracing::info!("abuse_reports list: {:?}", list);

    if let Some(abuse_report) = list.result.reports.first() {
        let detail = client
            .abuse_reports_get(&account_id, &abuse_report.id)
            .await
            .unwrap();
        tracing::info!("abuse_reports {} detail: {:?}", &abuse_report.id, detail);

        let mitigations_list = client
            .abuse_reports_mitigations_list(&account_id, &abuse_report.id, None)
            .await
            .unwrap();
        tracing::info!(
            "abuse_reports {} mitigations_list: {:?}",
            &abuse_report.id,
            mitigations_list
        );

        if let Some(mitigation) = mitigations_list.result.mitigations {
            if let Some(item) = mitigation.first() {
                let review = client
                    .abuse_reports_mitigations_review(
                        &account_id,
                        &abuse_report.id,
                        &MitigationReviewBody {
                            appeals: vec![MitigationReviewParamsAppeal {
                                id: item.id.clone(),
                                reason: MitigationReviewParamsAppealsReason::Removed,
                            }],
                        },
                    )
                    .await
                    .unwrap();
            }
        }
    }
}
