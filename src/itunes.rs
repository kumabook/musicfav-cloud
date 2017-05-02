use std::io::Read;
use serde_json;

use http;

static BASE_URL: &'static str = "https://itunes.apple.com/";

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    pub wrapperType:            String, // track
    pub kind:                   Option<String>,
    pub artistId:               i32,
    pub collectionId:           i32,
    pub trackId:                Option<i32>,
    pub artistName:             String,
    pub collectionName:         String,
    pub trackName:              Option<String>,
    pub collectionCensoredName: String,
    pub trackCensoredName:      Option<String>,
    pub artistViewUrl:          String,
    pub collectionViewUrl:      String,
    pub trackViewUrl:           Option<String>,
    pub previewUrl:             Option<String>,
    pub artworkUrl30:           Option<String>,
    pub artworkUrl60:           Option<String>,
    pub artworkUrl100:          Option<String>,
    pub collectionPrice:        f32,
    pub trackPrice:             Option<f32>,
    pub releaseDate:            String,
    pub collectionExplicitness: String,
    pub trackExplicitness:      Option<String>,
    pub discCount:              Option<i32>,
    pub discNumber:             Option<i32>,
    pub trackCount:             i32,
    pub trackNumber:            Option<i32>,
    pub trackTimeMillis:        Option<i32>,
    pub country:                String,
    pub currency:               String,
    pub primaryGenreName:       String,
    pub isStreamable:           Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    pub wrapperType:            String, // collection
    pub kind:                   String,
    pub artistId:               i32,
    pub collectionId:           i32,
    pub artistName:             String,
    pub collectionName:         String,
    pub collectionCensoredName: String,
    pub artistViewUrl:          String,
    pub collectionViewUrl:      String,
    pub collectionPrice:        String,
    pub releaseDate:            String,
    pub collectionExplicitness: String,
    pub trackCount:             i32,
    pub country:                String,
    pub currency:               String,
    pub primaryGenreName:       String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LookupResponse<T> {
    pub resultCount: i32,
    pub results:     Vec<T>,
}

pub fn fetch_songs(id: &str, country: &str) -> serde_json::Result<LookupResponse<Track>> {
    let url = format!("{}/lookup/?id={}&country={}&entity=song", BASE_URL, id, country);
    let mut res = http::client().get(&url)
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    serde_json::from_str(&body)
}

#[cfg(test)]
mod test {
    use super::fetch_songs;
    #[test]
    fn test_fetch_songs() {
        let response = fetch_songs("1160715126", "jp").unwrap();
        assert_eq!(response.resultCount, 14);
    }
}
