use reqwest;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
struct PlaylistItem {
    url: String,
    duration: f64,
}

impl Ord for PlaylistItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.duration.partial_cmp(&other.duration).unwrap()
    }
}

impl PartialOrd for PlaylistItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://example.com/playlist.m3u8"; // replace with your playlist URL
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let mut lines = body.lines();
    let mut items = VecDeque::new();

    while let Some(line) = lines.next() {
        if line.starts_with("#EXTINF:") {
            let duration = line.trim_start_matches("#EXTINF:").trim_end_matches(",");
            let url = lines.next().unwrap();
            items.push_back(PlaylistItem {
                url: url.to_string(),
                duration: duration.parse().unwrap(),
            });
        }
    }

    items.make_contiguous().sort();

    let mut file = File::create(Path::new("sorted_playlist.m3u8")).unwrap();
    writeln!(file, "#EXTM3U").unwrap();
    for item in items {
        writeln!(file, "#EXTINF:{},", item.duration).unwrap();
        writeln!(file, "{}", item.url).unwrap();
    }

    Ok(())
}
