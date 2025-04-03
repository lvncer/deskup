// src/main.rs
// メインエントリーポイント

mod app;
mod config;
mod models;
mod services;
mod ui;

use app::MyApp;
// use eframe::NativeOptions;

fn main() {
    // アプリケーションの初期化
    let app = MyApp::default();

    // デフォルトのネイティブオプションを使用
    let native_options = eframe::NativeOptions::default();

    // アプリケーションを実行
    eframe::run_native(Box::new(app), native_options);
}
