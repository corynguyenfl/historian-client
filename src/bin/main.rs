use std::{io, time::Duration};

use historian_client::historian::{
    ess_chart_data_request::TimeUnit, historian_service_client::HistorianServiceClient,
    EssChartDataRequest,
};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // wait for input from keyboard
    let mut input = String::new();

    let url =
        std::env::var("HISTORIAN_SERVICE_URL").unwrap_or("http://localhost:50052".to_string());

    let mrid = std::env::var("MRID").unwrap_or("3bda2cb0-6e39-40ca-84de-d58b99e7e40e".to_string());

    let mut client = HistorianServiceClient::connect(url).await.unwrap();

    print_usage();

    loop {
        if let Ok(_c) = io::stdin().read_line(&mut input) {
            if input.trim() == "last24" {
                let mut value = String::new();
                println!("Enter granularity (0=second, 1=minute, 2=hour):");
                if let Ok(_c) = io::stdin().read_line(&mut value) {
                    if let Ok(granularity) = value.trim().parse::<u32>() {
                        let granularity = match granularity {
                            0 => TimeUnit::Second,
                            1 => TimeUnit::Minute,
                            2 => TimeUnit::Hour,
                            _ => {
                                println!("Invalid granularity: {}", granularity);
                                continue;
                            }
                        };
                        // start_time: 24 hours ago in unix timestamp
                        let start_time = chrono::Utc::now().timestamp() - 24 * 60 * 60;

                        let _ =
                            get_ess_data(&mut client, mrid.clone(), start_time as u32, granularity)
                                .await;
                    } else {
                        println!("Invalid value: {}", value);
                    }
                }
            } else if input.trim() == "stream1sec" {
                let mut value = String::new();
                println!("Enter granularity (0=second, 1=minute, 2=hour):");
                if let Ok(_c) = io::stdin().read_line(&mut value) {
                    if let Ok(granularity) = value.trim().parse::<u32>() {
                        let granularity = match granularity {
                            0 => TimeUnit::Second,
                            1 => TimeUnit::Minute,
                            2 => TimeUnit::Hour,
                            _ => {
                                println!("Invalid granularity: {}", granularity);
                                continue;
                            }
                        };
                        // start_time: 24 hours ago in unix timestamp
                        let start_time = chrono::Utc::now().timestamp() - 24 * 60 * 60;

                        let _ = get_ess_data_stream(
                            &mut client,
                            mrid.clone(),
                            start_time as u32,
                            granularity,
                            1,
                        )
                        .await;
                    } else {
                        println!("Invalid value: {}", value);
                    }
                }
            } else if input.trim() == "exit" {
                println!("Quitting...");
                break;
            } else {
                println!("Unknown command: {}", input.trim());
                print_usage();
            }

            input.clear();

            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Enter:");
    println!("  last24:         Get last 24 hours data");
    println!("  stream1sec:     Stream data every second");
    println!("  exit:           Quit");
}

async fn get_ess_data(
    client: &mut HistorianServiceClient<tonic::transport::Channel>,
    mrid: String,
    start_time: u32,
    granularity: TimeUnit,
) -> Result<(), Box<dyn std::error::Error>> {
    let req = EssChartDataRequest {
        mrid: mrid.clone(),
        start_time,
        end_time: None,
        limit: None,
        granularity: Some(granularity as i32),
        progress: None,
    };

    let request = Request::new(req);

    let response = client.get_ess_chart_data(request).await.unwrap();

    let mut stream = response.into_inner();

    loop {
        match stream.message().await {
            Ok(message) => {
                if let Some(message) = message {
                    println!("{:?}", message);
                } else {
                    println!("No more data");
                    break;
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

async fn get_ess_data_stream(
    client: &mut HistorianServiceClient<tonic::transport::Channel>,
    mrid: String,
    start_time: u32,
    granularity: TimeUnit,
    refresh_interval: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut count = 0;

    let req = async_stream::stream! {
        let mut interval = tokio::time::interval(Duration::from_secs(refresh_interval));

        let mut start_time = start_time;

        loop {
            let r = EssChartDataRequest {
                mrid: mrid.clone(),
                start_time,
                end_time: None,
                limit: None,
                granularity: Some(granularity as i32),
                progress: Some(count <= 10),
            };

            let _time = interval.tick().await;
            start_time = chrono::Utc::now().timestamp() as u32;

            count += 1;

            yield r;
        }
    };

    let request = Request::new(req);

    let response = client.get_ess_chart_data_stream(request).await.unwrap();

    let mut stream = response.into_inner();

    loop {
        match stream.message().await {
            Ok(message) => {
                if let Some(message) = message {
                    println!("{:?}", message);
                } else {
                    println!("No more data");
                    break;
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
