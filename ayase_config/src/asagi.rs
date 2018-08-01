#![allow(non_snake_case)]

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct PageSettings {
    pub delay: i32,
    pub pages: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct BoardSettings {
    pub engine: Option<String>,
    pub database: Option<String>,
    pub host: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub charset: Option<String>,
    pub table: Option<String>,
    pub path: Option<String>,
    pub useOldDirectoryStructure: Option<bool>,
    pub deletedThreadsThresholdPage: Option<i32>,
    pub webserverGroup: Option<String>,
    pub thumbThreads: Option<i32>,
    pub mediaThreads: Option<i32>,
    pub newThreadsThreads: Option<i32>,
    pub threadRefreshRate: Option<i32>,
    pub refreshDelay: Option<i32>,
    pub throttleAPI: Option<bool>,
    pub throttleURL: Option<String>,
    pub throttleMillisec: Option<i64>,
    pub pageSettings: Option<Vec<PageSettings>>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub dumperEngine: String,
    pub sourceEngine: String,
    pub boardSettings: HashMap<String, BoardSettings>,
}

#[derive(Debug, Deserialize)]
pub struct SettingsContainer {
    pub settings: Settings,
}

pub type Value = SettingsContainer;

//impl From<ayase::Value> for Type {
//    // add code here
//}