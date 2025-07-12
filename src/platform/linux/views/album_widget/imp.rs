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
    #[template_child]
    inner_box: TemplateChild<gtk::Box>,

    pub album: RefCell<Option<AlbumObject>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumWidget {
    const NAME: &'static str = "AlbumWidget";
    type Type = super::AlbumWidget;

    type ParentType = gtk::Widget;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
        UtilityCallbacks::bind_template_callbacks(class);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

struct UtilityCallbacks {}

#[gtk::template_callbacks]
impl UtilityCallbacks {}

impl ObjectImpl for AlbumWidget {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().connect_notify(Some("album"), |obj, _| {
            if let Some(album_obj) = obj.property::<Option<AlbumObject>>("album") {
                let imp = obj.imp();
                if let Some(album) = album_obj.album() {
                    imp.album_name.get().set_text(&album.name);
                } else {
                    imp.album_name.get().set_text("");
                }

                if let Some(path) = album_obj.cover_path() {
                    imp.cover.get().set_from_file(Some(path));
                }

                if let Some(artist_name) = album_obj.artist_name() {
                    imp.artist_name.get().set_text(&artist_name);
                }

                *imp.album.borrow_mut() = Some(album_obj);
            }
        });
    }

    fn dispose(&self) {
        self.dispose_template();
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: once_cell::sync::Lazy<Vec<ParamSpec>> =
            once_cell::sync::Lazy::new(|| {
                vec![
                    glib::ParamSpecObject::builder::<AlbumObject>("album")
                        .readwrite()
                        .build(),
                ]
            });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "album" => {
                let album_obj = value.get::<Option<AlbumObject>>().unwrap();
                *self.album.borrow_mut() = album_obj;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "album" => self.album.borrow_mut().to_value(),
            e => {
                eprintln!("{e} - not unimplemented");
                unreachable!()
            }
        }
    }
}

impl WidgetImpl for AlbumWidget {}
