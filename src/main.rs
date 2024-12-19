use iced::{
    event,
    widget::{Button, Column, Container, Row, Scrollable, Text, TextInput},
    window, Alignment, Event, Length, Size, Subscription, Theme,
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::{collections::HashMap, fs, path::PathBuf};
use std::{fs::read_dir, path::Path};
use sysinfo::Disks;
use walkdir::WalkDir;

fn main() -> iced::Result {
    iced::application("File Explorer", FileExplorer::update, FileExplorer::view)
        .theme(FileExplorer::theme)
        .window(window::Settings {
            size: Size::new(800.0, 600.0),
            ..Default::default()
        })
        .subscription(FileExplorer::subscription)
        .run()
}

fn create_file_cache() -> FileCache {
    let mut cache = FileCache::load_from_file(
        r"C:\Users\yslll\OneDrive\Documents\rust\file-explorer\file_cahce.json",
    )
    .unwrap_or_else(|| FileCache::new());
    if cache.file_hashmap.is_empty() {
        println!("Cache is empty caching...");
        cache.update_cache(r"C:\");
        cache.save_to_file("file_cahce.json");
    }
    cache
}

#[derive()]
struct FileExplorer {
    window_size: Size,
    cards_to_make: Vec<String>,
    path: PathBuf,
    file_or_dir_to_search_for: String,
    file_cache: FileCache,
}

#[derive(Debug, Clone)]
enum Message {
    Clicked(String),
    WindowResize(Size),
    Back,
    TextInputChanged(String),
    TextInputSubmited(String),
}

impl Default for FileExplorer {
    fn default() -> Self {
        Self {
            window_size: Size::new(800.0, 600.0),
            cards_to_make: create_drives(),
            path: PathBuf::new(),
            file_or_dir_to_search_for: String::default(),
            file_cache: create_file_cache(),
        }
    }
}

impl FileExplorer {
    fn theme(_: &Self) -> Theme {
        // You can define a custom theme here
        Theme::Dracula // or use `Theme::Light`, or other options depending on what you want
    }
}

impl FileExplorer {
    fn update(&mut self, message: Message) {
        match message {
            Message::Clicked(name) => {
                println!("clicked {}", name);
                self.path.push(Path::new(&name));
                if self.path.is_file() == true {
                    println!("Cant open files yet");
                    self.path.pop();
                } else {
                    self.cards_to_make = self.create_card_info(PathBuf::from(name));
                }
                println!("path: {}", self.path.display());
            }
            Message::WindowResize(window_size) => {
                self.window_size = window_size;
                println!("window resized: {:?}", self.window_size);
            }
            Message::Back => {
                println!("{:?}", self.path);
                if self.path == PathBuf::from("Drives") {
                    println!("At start")
                } else if self.path.pop() == false {
                    self.cards_to_make = create_drives();
                    self.path = PathBuf::from("Drives");
                } else {
                    self.cards_to_make = self.create_card_info(self.path.clone());
                    println!("{:?}", self.path);
                }
            }

            Message::TextInputChanged(user_input) => {
                println!("{}", user_input);
                self.file_or_dir_to_search_for = user_input;
            }

            Message::TextInputSubmited(user_input) => {
                println!("Searching for {}", user_input);
                let start = Instant::now();
                let result = self.file_cache.file_hashmap.get_key_value(&user_input);
                self.path = PathBuf::from("Loading...");
                if user_input.chars().next().unwrap() == '.' {
                    let result = self
                        .file_cache
                        .extension_hashmap
                        .get_key_value(&user_input.replace(".", ""));
                    match result {
                        Some(found_file) => {
                            let mut vec_of_files = Vec::new();
                            for path in found_file.1 {
                                vec_of_files.push(path.to_str().unwrap().to_string());
                            }
                            self.cards_to_make = vec_of_files;
                            self.path = PathBuf::from("Search Results");
                        }
                        None => {
                            self.cards_to_make = create_drives();
                            self.path = PathBuf::from("No extensions with that name");
                        }
                    }
                } else {
                    match result {
                        Some(found_file) => {
                            let mut vec_of_files = Vec::new();
                            for path in found_file.1 {
                                vec_of_files.push(path.to_str().unwrap().to_string());
                            }
                            self.cards_to_make = vec_of_files;
                            self.path = PathBuf::from("Search Results");
                        }
                        None => {
                            self.cards_to_make = create_drives();
                            self.path = PathBuf::from("No files with that name");
                        }
                    }
                }
                let duration = start.elapsed();
                println!("Loading time was: {:?}", duration);
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let display_path = format!("{}", self.path.display());

        // let cards_per_row = self.window_size.width as u32 / 150;
        let mut column = Column::new().align_x(Alignment::Center).spacing(10); // Spacing between rows
        for card in &self.cards_to_make {
            let trimmed_card = card.trim_end_matches("\\");
            if let Some(file_name) = trimmed_card.split("\\").last() {
                let button_display =
                    if self.path == PathBuf::from("Drives") || self.path == PathBuf::default() {
                        format!("Name: {} -> Path: {}\\", file_name, file_name)
                    } else if self.path == PathBuf::from("C:\\") {
                        format!(
                            "Name: {} -> Path: {}{}",
                            file_name,
                            self.path.display(),
                            file_name
                        )
                    } else if self.path == PathBuf::from("Search Results"){
                        format!("Name: {} -> Path: {}", file_name, card)
                    } 
                    else {
                        format!(
                            "Name: {} -> Path: {}\\{}",
                            file_name,
                            self.path.display(),
                            file_name,
                        )
                    };

                column = column.push(
                    Row::new().push(
                        Button::new(Text::new(button_display).size(20))
                            .width(Length::Fill)
                            .on_press(Message::Clicked(card.to_owned())),
                    ),
                );
            }
        }

        // Wrap the grid in a centered container
        let search_bar = Column::new()
            .spacing(5)
            .push(Text::new(display_path).size(20))
            .padding(5)
            .push(
                Row::new()
                    .align_y(Alignment::Center)
                    .spacing(10)
                    .push(
                        Button::new(Text::new("Back").center())
                            .height(45)
                            .on_press(Message::Back),
                    )
                    .push(
                        TextInput::new("Search all files", &self.file_or_dir_to_search_for)
                            .on_input(|input| Message::TextInputChanged(input))
                            .on_submit(Message::TextInputSubmited(
                                self.file_or_dir_to_search_for.clone(),
                            ))
                            .size(25)
                            .width(Length::Fill),
                    ),
            );

        let file_area = Scrollable::new(column.align_x(Alignment::Start).width(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill);

        let layout = Column::new()
            .push(search_bar) // Top search bar
            .push(file_area) // Remaining content
            .height(Length::Fill);

        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _id| match event {
            Event::Window(event) => match event {
                window::Event::Resized(size) => Some(Message::WindowResize(size)),
                _ => None,
            },
            _ => None,
        })
    }
}

fn create_drives() -> Vec<String> {
    let mut drives = Vec::new();
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        drives.push(
            disk.mount_point()
                .to_str()
                .expect("Error changing file to str")
                .to_string(),
        );
    }
    println!("{:?}", drives);
    drives
}

impl FileExplorer {
    fn create_card_info(&self, file_path: PathBuf) -> Vec<String> {
        println!("{:?}", file_path);
        let dirs = read_dir(self.path.clone());
        let mut all_files = Vec::new();
        match dirs {
            Ok(dir_name) => {
                for dir in dir_name {
                    match dir {
                        Ok(dir_name) => {
                            all_files.push(dir_name.file_name().to_string_lossy().into_owned());
                        }
                        _ => {
                            println!("Error reading directory")
                        }
                    }
                }
            }
            Err(err) => {
                println!("Unexpected error {}", err);
            }
        }
        all_files
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FileCache {
    file_hashmap: HashMap<String, Vec<PathBuf>>,
    extension_hashmap: HashMap<String, Vec<PathBuf>>,
}

impl FileCache {
    fn new() -> Self {
        FileCache {
            file_hashmap: HashMap::new(),
            extension_hashmap: HashMap::new(),
        }
    }

    fn update_cache(&mut self, _root_dir: &str) {
        let start = Instant::now();
        for entry in WalkDir::new(r"C:\").into_iter().filter_map(|e| e.ok()) {
            println!("{}", entry.path().display());
            let path = entry.path();

            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                    if let Some(file_extension) =
                        path.extension().and_then(|extension| extension.to_str())
                    {
                        let path_to_file = path.to_path_buf();
                        let file_name = file_name.to_string();
                        let file_extenstion = file_extension.to_string();
                        self.extension_hashmap
                            .entry(file_extenstion)
                            .or_insert_with(Vec::new)
                            .push(path_to_file.clone());
                        self.file_hashmap
                            .entry(file_name)
                            .or_insert_with(Vec::new)
                            .push(path_to_file.clone());
                    }
                }
            }
        }
        let duration = start.elapsed();
        println!("Cache time was: {:?}", duration)
    }

    fn save_to_file(&self, path: &str) {
        if let Ok(data) = serde_json::to_string(&self) {
            fs::write(path, data).expect("Error writing to file");
        }
    }

    fn load_from_file(path: &str) -> Option<Self> {
        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(cache) = serde_json::from_str(&data) {
                return Some(cache);
            }
        }
        None
    }
}
