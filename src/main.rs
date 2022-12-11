mod discogs;
mod models;
use discogs::DiscogsClient;

fn main() {
    let discogs_client: DiscogsClient =
        DiscogsClient::new("gqvzVtgoghLkXbwsvkyXgmdoVeLZSebShZFpORVx");

    // println!("{}", body);
    let releases = discogs_client.query("Nirvana").get_releases();

    println!("{:#?}", &releases)
}
