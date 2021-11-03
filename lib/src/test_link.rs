use surf::{StatusCode};
use async_recursion::async_recursion;

#[derive(Debug, PartialEq)]
enum TestLinkResult {
    Ok,
    TrialLimitExceeded,
    RespondedWithOutOk,
    FailedToConnect,
    Error
}

const TRIAL_LIMIT: u8 = 5;
#[async_recursion]
async fn test_link(url: &str, count: u8) -> TestLinkResult {
    if count > TRIAL_LIMIT {
        return TestLinkResult::TrialLimitExceeded;
    }

    let res = surf::get(url).await;

    if res.is_err() {
        return TestLinkResult::FailedToConnect;
    }

    let res = res.unwrap();

    match res.status() {
        StatusCode::Ok => TestLinkResult::Ok,
        StatusCode::MovedPermanently | StatusCode::SeeOther => {
            let location = res.header("location").unwrap().get(0);

            if location == None {
                return TestLinkResult::Error
            }

            let url = location.unwrap().as_str();

            test_link(url, count + 1).await
        },
        _ => TestLinkResult::RespondedWithOutOk
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn return_ok_when_responded_with_200() {
        let server = MockServer::start();

        let response_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/");

            then.status(200)
                .header("content-type", "text/html");
        });

        let result = test_link(server.url("/").as_str(), 0).await;

        response_mock.assert();

        assert_eq!(result, TestLinkResult::Ok)
    }

    #[tokio::test]
    async fn return_trial_limit_exceeded_when_responded_301_forever() {
        let server = MockServer::start();
        let server_url = server.url("/");

        let response_mock = server.mock(|when, then| {
           when.method(GET)
               .path("/");

            then.status(301)
                .header("location", &server_url);
        });

        let result = test_link(server.url("/").as_str(), 0).await;

        response_mock.assert_hits(6);

        assert_eq!(result, TestLinkResult::TrialLimitExceeded)
    }

    #[tokio::test]
    async fn return_failed_to_connect_when_server_is_dead() {
        let result = test_link("http://127.0.0.1:65535", 0).await;

        assert_eq!(result, TestLinkResult::FailedToConnect)
    }

    #[tokio::test]
    async fn return_responded_without_ok_when_responded_with404() {
        let server = MockServer::start();

        let response_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/");

            then.status(404);
        });

        let result = test_link(server.url("/").as_str(), 0).await;

        response_mock.assert();

        assert_eq!(result, TestLinkResult::RespondedWithOutOk)
    }
}