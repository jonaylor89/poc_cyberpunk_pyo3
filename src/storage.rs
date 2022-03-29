
use crate::pydub::{self, AudioSegment};

pub trait AudioStore {

    fn contains(&self, key: &str) -> bool;

    fn get_segment(&self, key: &str) -> Result<(AudioSegment, String), String>;
}

pub struct LocalStorage<'l> {
    pub base_dir: &'l str,
}

impl<'l> LocalStorage<'l> {
    pub fn new() -> Box<Self> {
        Box::new(LocalStorage {
            base_dir: "testdata/",
        })
    }
}

impl<'l> AudioStore for LocalStorage<'l> {
    fn contains(&self, key: &str) -> bool {
        println!("{}", key);
        true
    }

    fn get_segment(&self, key: &str) -> Result<(AudioSegment, String), String> {
        let segment = pydub::from_file(&format!("{}{}", self.base_dir, key)).unwrap();

        Ok((segment, key.to_string()))
    }
}

#[derive(Default)]
pub struct AudioStorage {
    // local_storage_base_dir: Option<&'s str>,
    // local_results_base_dir: Option<&'s str>,
    // http_loader: HttpLoader,
    audio_stores: Vec<Box<dyn AudioStore>>,
}

impl AudioStorage {
    pub fn new() -> Self {
        Self {
            audio_stores: vec![LocalStorage::new()]
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        for store in &self.audio_stores {
            if store.contains(key) { return true; }
        }

        false
    }

    pub fn get_segment(&self, key: &str) -> Result<(AudioSegment, String), String> {

        for store in &self.audio_stores {
            if store.contains(key) { return store.get_segment(key); }
        }

        Err("key not found".to_string())
    }

    pub fn save_segment(&self, key: &str, segment: AudioSegment, file_format: &str) -> String {
        let processed_key = format!("{}.{}", key, file_format);

        pydub::export(
            segment,
            &format!("testdata/{}", processed_key),
            file_format,
        ).unwrap();

        processed_key
    }
}
