use std::{process::ExitCode, sync::Arc};

use adw::{ApplicationWindow, prelude::*};
use gtk::Application;
// use gtk::prelude::*;

use crate::track::MusicCollection;

use super::Frontend;

const APP_ID: &str = "dev.luxluth.lorchestre";

pub struct Lorchestre {
    collection: MusicCollection,
    app: Application,
}

impl Lorchestre {}

fn activate(mut lorchestre: Arc<Lorchestre>) {
    let window = ApplicationWindow::builder()
        .application(&lorchestre.app)
        .title("L'orchestre")
        .css_name("window")
        .resizable(true)
        .default_width(800)
        .default_height(600)
        .build();

    window.present();
}

impl Frontend for Lorchestre {
    fn init(collection: crate::track::MusicCollection) -> Self {
        gtk::init().expect("Unable to init gtk");
        let application = Application::new(Some(APP_ID), Default::default());

        Self {
            collection,
            app: application,
        }
    }

    fn start(self) -> ExitCode {
        let app_clone = self.app.clone();
        let lorch_arc = Arc::new(self);
        app_clone.connect_activate(move |_| {
            activate(lorch_arc.clone());
        });

        ExitCode::from(app_clone.run().value() as u8)
    }
}
