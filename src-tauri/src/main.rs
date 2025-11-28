// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::menu::{AboutMetadata, MenuBuilder, SubmenuBuilder};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let metadata = AboutMetadata {
                name: Some("LogisimDoma".into()),
                version: Some("0.1.0".into()),
                copyright: Some(
                    "Copyright © 2025 Artemii Matiazh.\nLicensed under the GNU AGPL v3.".into(),
                ),
                license: Some("Licensed under the GNU AGPL v3.".into()),
                authors: Some(vec!["Zevobla".into()]),
                website: Some("https://github.com/zevobla/logisim-doma".into()),
                website_label: Some("LogisimDoma Source Code".into()),
                comments: Some("A modern port of Logisim".into()),
                ..Default::default()
            };
            let app_menu = SubmenuBuilder::new(app, "LogisimDoma")
                .about_with_text("About LogisimDoma", Some(metadata))
                .separator()
                .services()
                .separator()
                .hide()
                .hide_others()
                .show_all()
                .separator()
                .close_window()
                .build()?;
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("open", "Open...")
                .text("quit", "Quit")
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&app_menu, &file_menu])
                .build()?;
            app.set_menu(menu)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
