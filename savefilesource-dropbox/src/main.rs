use std::env;
use dolphinsavesync_fs::SaveFileSource;
use dropbox_cloudhandler::DropboxSaveFileSource;

/// This program reads arguments from the command line to test the [DropboxSaveFileSource].
/// Args in order:
/// 1) A Dropbox API key
/// 2) The path to the file that needs to be downloaded from Dropbox.
/// 3) What folder to save the file in on the local machine.
/// 4) The name of the file(NO PATH) to be saved on the local machine, with or without an extension.
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let api_key = &args[1];
    let file_path = &args[2];
    let save_dir = &args[3];
    let save_file_name = &args[4];
    let handler = DropboxSaveFileSource::new(
        api_key.as_str(),
        file_path.as_str(),
        save_dir.as_str(),
        save_file_name.as_str(),
    );

    handler.retrieve_save_file();
}