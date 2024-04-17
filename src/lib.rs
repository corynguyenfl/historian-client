mod generated;

pub use generated::*;

// tests
#[cfg(test)]
mod tests {
    use self::historian::historian_service_client::HistorianServiceClient;
    use super::*;
    use crate::historian::*;
    use prost::Message;

    // test encode of HistorianProfileRequest
    #[test]
    fn test_historian_profile_request() {
        let historian_profile_request = HistorianProfileRequest {
            mrid: uuid::Uuid::new_v4().hyphenated().to_string(),
            begin_time_stamp: 1234,
            end_time_stamp: 5678,
            limit: 1000,
        };

        let encoded = historian_profile_request.encode_to_vec();

        let deserialized = HistorianProfileRequest::decode(&encoded[..]).unwrap();

        assert_eq!(historian_profile_request, deserialized);
    }

    // test HistorianServiceClient gRPC service
    #[tokio::test]
    async fn test_historian_service_client() {
        // get gRPC connection from env
        let url =
            std::env::var("HISTORIAN_SERVICE_URL").unwrap_or("http://localhost:50052".to_string());

        let mut client = HistorianServiceClient::connect(url).await.unwrap();

        // begin_time_stamp: 24 hours ago in unix timestamp
        let begin_time_stamp = chrono::Utc::now().timestamp() - 24 * 60 * 60;

        // end_time_stamp: 24 hours from now
        let end_time_stamp = chrono::Utc::now().timestamp() + 24 * 60 * 60;

        let request = tonic::Request::new(HistorianProfileRequest {
            mrid: "fd32c1f5-7cb1-4fa4-b578-0e5ac023c47f".to_string(),
            begin_time_stamp: begin_time_stamp as u32,
            end_time_stamp: end_time_stamp as u32,
            limit: 2,
        });

        let response = client.get_resource_status(request).await.unwrap();

        let mut stream = response.into_inner();

        // read from breaker_reading_profile
        while let Some(message) = stream.message().await.unwrap() {
            println!("{:#?}", message);
        }
    }

    // test HistorianServiceClient gRPC service GetResourceValueByKey
    #[tokio::test]
    async fn test_historian_service_client_get_resource_value_by_key() {
        // get gRPC connection from env
        let url =
            std::env::var("HISTORIAN_SERVICE_URL").unwrap_or("http://localhost:50052".to_string());

        let mut client = HistorianServiceClient::connect(url).await.unwrap();

        // begin_time_stamp: 24 hours ago in unix timestamp
        let begin_time_stamp = chrono::Utc::now().timestamp() - 24 * 60 * 60;

        // end_time_stamp: 24 hours from now
        let end_time_stamp = chrono::Utc::now().timestamp() + 24 * 60 * 60;

        let requests = vec![
            ("SCHEDULE_STATE", historian_ggio_request::GgioType::String),
            ("APP_STATE", historian_ggio_request::GgioType::Bool),
            ("TEMPERATURE", historian_ggio_request::GgioType::Analog),
            ("HUMIDITY", historian_ggio_request::GgioType::Integer),
        ];

        for kvp in requests {
            let request = tonic::Request::new(HistorianGgioRequest {
                mrid: "fd32c1f5-7cb1-4fa4-b578-0e5ac023c47f".to_string(),
                begin_time_stamp: begin_time_stamp as u32,
                end_time_stamp: end_time_stamp as u32,
                identified_object_name: kvp.0.to_string(),
                typ: kvp.1.into(),
                limit: 10,
            });

            match client.get_resource_value_by_key(request).await {
                Ok(response) => {
                    let mut stream = response.into_inner();

                    while let Some(message) = stream.message().await.unwrap() {
                        println!(
                            "Resource By Key: {}: {} - {}",
                            message.timestamp, message.tag, message.val
                        );
                    }
                }
                Err(e) => {
                    println!("Error::Failed to get resource value by key {:?}", e);
                    assert!(false);
                }
            }
        }
    }
}
