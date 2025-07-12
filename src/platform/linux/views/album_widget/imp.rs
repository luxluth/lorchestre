use std::cell::RefCell;

use crate::platform::linux::views::AlbumObject;
use gtk::{
    glib::{self, ParamSpec, Value},
    prelude::*,
    subclass::prelude::*,
};

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "widget.ui")]
pub struct AlbumWidget {
    #[template_child]
    cover: TemplateChild<gtk::Image>,
    #[template_child]
    album_name: TemplateChild<gtk::Label>,
    #[template_child]
    artist_name: TemplateChild<gtk::Label>,

    pub album: RefCell<Option<AlbumObject>>,
    pub artist: RefCell<Option<String>>,
    pub cover_path: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumWidget {
    const NAME: &'static str = "AlbumWidget";
    type Type = super::AlbumWidget;

    type ParentType = gtk::Widget;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
        class.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl AlbumWidget {}

impl ObjectImpl for AlbumWidget {
    fn constructed(&self) {
        self.cover
            .get()
            .set_from_file(self.cover_path.clone().into_inner());
        if let Some(album_obj) = self.album.clone().into_inner() {
            if let Some(album) = album_obj.album().clone() {
                self.album_name.get().set_text(&album.name);
            }
        }

        if let Some(artist_name) = self.artist.clone().into_inner() {
            self.artist_name.get().set_text(&artist_name);
        }
    }

    fn dispose(&self) {
        self.dispose_template();
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: once_cell::sync::Lazy<Vec<ParamSpec>> =
            once_cell::sync::Lazy::new(|| {
                vec![
                    glib::ParamSpecString::builder("artist")
                        .readwrite()
                        .default_value(None)
                        .build(),
                    glib::ParamSpecString::builder("cover-path")
                        .readwrite()
                        .default_value(None)
                        .build(),
                    glib::ParamSpecObject::builder::<AlbumObject>("album")
                        .readwrite()
                        .build(),
                ]
            });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "artist" => {
                let artist: Option<String> = value.get().unwrap();
                *self.artist.borrow_mut() = artist;
            }
            "cover-path" => {
                let path: Option<String> = value.get().unwrap();
                *self.cover_path.borrow_mut() = path;
            }
            "album" => {
                let album_obj = value.get::<AlbumObject>().unwrap();
                *self.album.borrow_mut() = Some(album_obj);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "artist" => self.artist.borrow().to_value(),
            "cover-path" => self.cover_path.borrow().to_value(),
            "album" => self.album.borrow_mut().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for AlbumWidget {}
