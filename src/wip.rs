extern crate proxer;

use proxer::Client;
use proxer::parameter;
use proxer::Endpoint;


fn main() {

    let prxr = Client::with_env_key("PROXER_API_KEY").unwrap();


    let res = prxr.api().info().get_fullentry(parameter::InfoGetFullEntry {id: 53}).send();

    eprintln!("{:#?}", res);
}
