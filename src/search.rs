use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::skim::SkimScoreConfig;

use crate::track::MusicCollection;

#[derive(Default)]
pub struct SearchIndex {}

impl SearchIndex {
    pub fn index(collection: &MusicCollection) -> SearchIndex {
        let mut index = SearchIndex::default();
        todo!()
    }
}
