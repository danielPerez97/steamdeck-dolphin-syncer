use std::fs::File;

/// [SaveFileSource] defines the API for syncing the .gci files between the Cloud and local Steam Deck filesystem.
///
/// Before a Dolphin session, if needed, the user should request the latest .gci file from Dropbox.
/// This file will get written to the correct folder for Dolphin Emulator to use on the Steam Deck.
///
/// After a session, the user should come back and upload their .gci file after a session by running
/// these functions as an executable from Steam.
///
pub trait SaveFileSource {

    /// This function should retrieve the file, save it to the local disk if needed, and return the [File]
    /// handle to interact with it.
    fn retrieve_save_file(&self) -> File;
}