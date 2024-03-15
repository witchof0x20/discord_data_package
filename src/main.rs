// Copyright 2024 witchof0x20
// This file is part of discord_data_package.
//
// discord_data_package is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// discord_data_package is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with discord_data_package. If not, see <https://www.gnu.org/licenses/>.

mod plot;
mod schema;

use crate::schema::messages::Message;
use chrono::{DateTime, Timelike, Utc};
use std::fs::File;
use std::io::{self, BufReader};
use zip::result::ZipError;
use zip::ZipArchive;

fn main() -> Result<(), MainError> {
    // Open the file
    let zip_file = File::open("package.zip")
        // TODO: check if double buffering
        .map(BufReader::new)
        .map_err(MainError::OpenFile)?;
    // Open the archive
    let mut archive = ZipArchive::new(zip_file).map_err(MainError::OpenZip)?;
    // Used to store times of day
    let mut times = Vec::new();
    // Iterate over the zip's files
    for index in 0..archive.len() {
        // Open the file
        let file = archive.by_index(index).map_err(MainError::OpenSubFile)?;
        let name = file.name();
        if name.starts_with("messages") && name.ends_with("messages.json") {
            // Read file as json
            let messages: Vec<Message> =
                serde_json::from_reader(file).map_err(MainError::ParseMessages)?;
            // Store timestamps
            times.extend(messages.into_iter().map(|m| m.timestamp));
        }
    }
    crate::plot::message_activity(&times);
    // Generate the graph
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum MainError {
    #[error("Error opening file: {0}")]
    OpenFile(io::Error),
    #[error("Error opening zip: {0}")]
    OpenZip(ZipError),
    #[error("Error opening file in zip: {0}")]
    OpenSubFile(ZipError),
    #[error("Error parsing messages json: {0}")]
    ParseMessages(serde_json::Error),
}
