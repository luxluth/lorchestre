use std::{process::ExitCode, sync::Arc};

use adw::{ApplicationWindow, HeaderBar, prelude::*};
use gtk::{Application, Label};
// use gtk::prelude::*;

use crate::track::MusicCollection;

use super::Frontend;

const APP_ID: &str = "dev.luxluth.lorchestre";
const BASE_CSS: &str = include_str!("style.css");

pub struct Lorchestre {
    collection: MusicCollection,
    app: Application,
}

impl Lorchestre {}

fn load_css(css: &str, previous_provider: Option<gtk::CssProvider>) {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(&css);

    if let Some(previous_provider) = previous_provider {
        gtk::style_context_remove_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not connect to a display."),
            &previous_provider,
        );
    }

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn activate(mut lorchestre: Arc<Lorchestre>) {
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(0)
        .css_classes(["view"])
        .build();

    let title_widget = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(4)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    let title = Label::builder().css_classes(["bold"]).build();
    title.set_text("L'orchestre");

    title_widget.append(&title);

    let header = HeaderBar::builder()
        .css_classes(["flat"])
        .title_widget(&title_widget)
        .build();

    content.append(&header);

    let window = ApplicationWindow::builder()
        .application(&lorchestre.app)
        .title("L'orchestre")
        .css_name("window")
        .resizable(true)
        .default_width(800)
        .default_height(600)
        .content(&content)
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
            load_css(&BASE_CSS, None);
            activate(lorch_arc.clone());
        });

        ExitCode::from(app_clone.run().value() as u8)
    }
}
