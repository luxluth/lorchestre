use glib::subclass::prelude::*;
use gtk::glib;
use std::cell::RefCell;

use crate::track::Album;

#[derive(Default)]
pub struct AlbumObject {
    pub album: RefCell<Option<Album>>,
    pub cover_path: RefCell<Option<String>>,
    pub artist_name: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AlbumObject {
    const NAME: &'static str = "AlbumObject";
    type Type = super::AlbumObject;
    type ParentType = glib::Object;
}

impl ObjectImpl for AlbumObject {}
