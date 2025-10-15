use std::{
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::BufWriter,
    ops::{Deref, DerefMut},
};

use clap::error::ErrorKind;
use serde::{Deserialize, Serialize};

use crate::{command::BookmarkCommand, io::data_dir_file, notify::notify};

pub struct BookmarkSpec {
    silent: bool,
    bookmarks: Bookmarks,
}

#[derive(Deserialize, Serialize)]
struct Bookmark(String);

#[derive(Deserialize, Serialize)]
struct Bookmarks(Vec<Bookmark>);

impl BookmarkSpec {
    pub fn new(silent: bool) -> Result<Self, Box<dyn Error>> {
        Ok(BookmarkSpec {
            silent,
            bookmarks: Bookmarks::new()?,
        })
    }

    pub fn run(&mut self, modifier: BookmarkCommand) -> Result<(), Box<dyn Error>> {
        match modifier {
            BookmarkCommand::Stdout => println!("{}", self.bookmarks),
            BookmarkCommand::Add { bookmark } => {
                self.bookmarks.push(Bookmark(bookmark.clone()));
                self.bookmarks.save()?;
                notify(self.silent, "Bookmark Added", bookmark)?;
            }
            BookmarkCommand::Remove { index } => {
                if index >= self.bookmarks.len() {
                    notify(
                        self.silent,
                        "Error",
                        &format!("{} is not a valid bookmark index", index),
                    )?;
                    return Err(Box::new(clap::Error::new(ErrorKind::ValueValidation)));
                }

                let removed = self.bookmarks.remove(index);
                self.bookmarks.save()?;
                notify(self.silent, "Bookmark Removed", removed.deref())?;
            }
        }

        Ok(())
    }
}

impl Bookmarks {
    fn new() -> Result<Bookmarks, Box<dyn Error>> {
        let path = data_dir_file("bookmarks.json")?;
        match fs::read_to_string(path) {
            Ok(string) => Ok(serde_json::from_str::<Bookmarks>(&string)?),
            Err(_) => Ok(Bookmarks(Vec::new())),
        }
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = data_dir_file("bookmarks.json")?;
        let file = File::create(path)?;
        Ok(serde_json::to_writer_pretty(BufWriter::new(file), self)?)
    }
}

impl Display for Bookmarks {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format.write_str(&self.iter().enumerate().fold(
            String::new(),
            |mut result, (i, bookmark)| {
                result.push_str("(");
                result.push_str(&i.to_string());
                result.push_str(") ");
                result.push_str(&bookmark);
                result.push_str("  ");
                result
            },
        ))
    }
}

impl Deref for Bookmark {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Bookmarks {
    type Target = Vec<Bookmark>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bookmarks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
