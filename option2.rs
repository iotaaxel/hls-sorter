use reqwest;
use std::collections::Vec;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Playlist {
    segments: Vec<String>,
}

impl Playlist {
    fn new() -> Self {
        Playlist { segments: Vec::new() }
    }

    fn add_segment(&mut self, segment: String) {
        self.segments.push(segment);
    }

    fn sort(&mut self) {
        self.segments.sort();
    }
}

fn fetch_playlist(url: &str) -> Result<Playlist, reqwest::Error> {
    let mut response = reqwest::get(url)?;
    let mut playlist = Playlist::new();
    let mut lines = response.text()?.lines();
    for line in lines {
        if line.starts_with("#EXTINF:") {
            let segment = line.trim_start_matches("#EXTINF:").trim().to_string();
            playlist.add_segment(segment);
        }
    }
    playlist.sort();
    Ok(playlist)
}

fn main() {
    let url = "https://example.com/playlist.m3u8";
    let playlist = fetch_playlist(url).unwrap();
    println!("Sorted playlist:");
    for segment in playlist.segments {
        println!("{}", segment);
    }
}
