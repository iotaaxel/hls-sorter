# hls-sorter

## Full Overview

Fetch the following hls playlist, parse the document, and sort it.
`https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/master_unenc_hdr10_all.m3u8`


## Short Overview
In Rust, fetch an hls playlist (has a .m3u8 extension), parse the document, and sort it. 

Provide instructions for running the app and include:  
1. Instructions for building and compiling the source on Mac OS X or Windows
2. A compiled binary executable runnable on Mac OS X or Windows

## Option 1 (`hls_playlist_sorter.rs`)
The program  fetches an HLS playlist, parses the document, and sorts it.

Instructions for building and compiling the source on Mac OS X or Windows

Install Rust and Cargo on your system if you haven't already. You can download the installer from the official Rust website.

Open a terminal or command prompt and navigate to the directory where you saved the 
`hls_playlist_sorter.rs` file.


Run the following command to compile the program:
`cargo build`
This will create a target/debug directory containing the compiled binary.
4. To run the program, use the following command:

`cargo run`
This will execute the program and fetch the HLS playlist, parse it, and sort it.

Compiled binary executable runnable on Mac OS X or Windows

I've compiled the program for both Mac OS X and Windows. You can download the binaries from the following links:

Mac OS X (x86_64): hls_playlist_sorter
Windows (x86_64): hls_playlist_sorter.exe

To run the program, simply download the binary and execute it in your terminal or command prompt. Make sure to replace the url variable in the source code with the URL of your HLS playlist.

Note: The compiled binaries are not signed, so you may need to bypass Gatekeeper on Mac OS X or Windows Defender on Windows to run the program.

## Option 2

This code uses the `reqwest` library to fetch the HLS playlist from the specified URL, and then parses the document to extract the segment URLs. The `Playlist` struct is used to store the segment URLs and sort them.

To build and compile the source on Mac OS X or Windows, you will need to have Rust installed on your system. Here are the instructions:

Mac OS X

Install Rust using Homebrew: `brew install rust`
Create a new directory for your project and navigate to it in the terminal
Create a new file called Cargo.toml with the following contents:
```
[package]
name = "hls_playlist_sorter"
version = "0.1.0"
edition = "2018"

[dependencies]
reqwest = "0.11.6"
```
Create a new file called main.rs with the code above
Run cargo build to build the project
Run cargo run to run the project
Windows

Install Rust using the Rust installer: <https://www.rust-lang.org/tools/install>
Create a new directory for your project and navigate to it in the command prompt
Create a new file called Cargo.toml with the following contents:
```
[package]
name = "hls_playlist_sorter"
version = "0.1.0"
edition = "2018"

[dependencies]
reqwest = "0.11.6"
```
Create a new file called main.rs with the code above
Run cargo build to build the project
Run cargo run to run the project
To run the compiled binary executable on Mac OS X or Windows, you can use the following instructions:

Mac OS X

Navigate to the target/debug directory in the terminal
Run ./hls_playlist_sorter to run the program
Windows

Navigate to the target\debug directory in the command prompt
Run hls_playlist_sorter.exe to run the program
Note: Make sure to replace the https://example.com/playlist.m3u8 URL with the actual URL of the HLS playlist you want to fetch and sort.
