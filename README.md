# Word of the day

This is a simple widget application for Macos displaying today's Persian date like a widget and also in the menubar.

## How to install?

Head over to [Releases](https://github.com/callmearta/word-of-the-day/releases) section and download latest release. 
For now since I don't have an Apple developer account you need to disable gatekeeper in order to install and run the application, or you can also download the source code and run it for yourself if you're worried there's some funny business going on behind the scenes.
To disable Macos gatekeeper open a new terminal and write the following in it, be sure to replace `/path/to/word-of-the-day` with the actual location of the downloaded `.app` application.
`xattr -c /path/to/word-of-the-day`

## How to develop?
First of all assure that you already have installed `Rust` and `cargo` in order to be able to run it.
After that just clone the repo and do an npm install as follows:
`git clone https://github.com/callmearta/word-of-the-day.git`
`cd word-of-the-day`
`npm install`
`npm run tauri dev`

Any pull request is highly appreciated but please keep in mind it should not be containing some new feature irrelative to the main purpose of the application.