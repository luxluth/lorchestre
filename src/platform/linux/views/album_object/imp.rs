use glib::subclass::prelude::*;
use gtk::glib;
use std::cell::RefCell;

use crate::track::Album;

pub struct AlbumObject {
    pub album: RefCell<Option<Album>>,
}

impl Default for AlbumObject {
    fn default() -> Self {
        Self {
            album: RefCell::new(None),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumObject {
    const NAME: &'static str = "AlbumObject";
    type Type = super::AlbumObject;
    type ParentType = glib::Object;
}

impl ObjectImpl for AlbumObject {}
