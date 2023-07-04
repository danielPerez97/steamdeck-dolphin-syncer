use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use dolphinsavesync_fs::SaveFileSource;

/// [DropboxSaveFileSource] is an implementation of [SaveFileSource] to orchestrate pushing and pulling
/// .gci files as a source for Dolphin saves.
///
/// api_key: This is the API key needed to make HTTP requests to the Dropbox API.
/// file_path: This is the file path within Dropbox to the file that will be downloaded.
/// save_dir: This is the folder where the file will be downloaded.
/// save_file_name: This is the full name(with extension) of the file to be downloaded.
/// client: This is the [Client] from the reqwest Crate that [DropboxSaveFileSource] will use internally.
pub struct DropboxSaveFileSource {
    api_key: String,
    file_path: String,
    save_dir: PathBuf,
    save_file_name: String,
    client: Client,
}

/// [DropboxFileRequest] is a simple representation of the following JSON:
///
/// {
///     "path": "/Homework/math/Prime_Numbers.txt"
/// }
///
/// This is the format needed to interact with the Dropbox API to retrieve a file.
#[derive(Serialize, Deserialize)]
pub struct DropboxFileRequest {
    path: String,
}


impl DropboxSaveFileSource {

    /// Creates a [DropboxSaveFileSource].
    pub fn new(
        api_key: &str,
        file_path: &str,
        save_dir: &str,
        save_file_name: &str,
    ) -> DropboxSaveFileSource {
        DropboxSaveFileSource {
            api_key: String::from(api_key),
            file_path: String::from(file_path),
            save_dir: PathBuf::from(save_dir),
            save_file_name: String::from(save_file_name),
            client: Default::default(),
        }
    }
}

/// Implementation of [SaveFileSource] to interact with the Dropbox API.
impl SaveFileSource for DropboxSaveFileSource {

    /// See: https://www.dropbox.com/developers/documentation/http/documentation#files-download
    fn retrieve_save_file(&self) -> File {
        let file_path = DropboxFileRequest { path: self.file_path.clone() };
        let file_path_json = serde_json::to_string(&file_path).unwrap();

        let authorization_header_value = format!("Bearer {}", &self.api_key);
        let client = Client::new();
        let request = client.get("https://content.dropboxapi.com/2/files/download")
            .header("Authorization", authorization_header_value)
            .header("Dropbox-API-Arg", file_path_json.clone())
            .build().unwrap();

        let response = self.client.execute(request).unwrap();
        let response_bytes = response.bytes().unwrap();

        let mut file = File::create(&self.save_dir.join(self.save_file_name.as_str())).unwrap();
        file.write_all(&response_bytes).unwrap();

        file
    }
}