mod time;
use Client;
use api;
use error;


const ENV_KEY: &str = "PROXER_API_KEY";


#[test]
fn api_response()
{
	// is the api structured as we want it to be?
	info!("creating client");
	let mut client = match Client::with_env_key(ENV_KEY) {
		None => return,
		Some(client) => client
	};


	info!("build request");
	let req = api::info::GetFullEntry { id: 53 };


	info!("send request");
	let res = client.execute(req);
	info!("request sent");



	match res
	{
		Err(e) => {
			error!("error...");
			eprintln!("response {:#?}", e);

			match e
			{
				error::Error::Api(e) => {
					info!("api error");

					match e.error()
					{
						error::api::Errcode::NoApiPermissions => return,
						_ => panic!("unexpected api error: {}", e),
					}
				}
				error::Error::Json(e) => panic!("can't parse json: {}", e),
				error::Error::Unknown => panic!("unknown error"),
				_ => return,
			}
		}

		Ok(r) => {
			info!("Ok");
			assert_eq!(r.medium, "animeseries");
		}
	}
}
