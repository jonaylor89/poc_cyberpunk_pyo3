
use rocket::debug;
use rocket::form::FromForm;

use uuid::Uuid;

use crate::storage::AudioStorage;


#[derive(Debug, Default, FromForm)]
pub struct Transformations<'r> {
    pub format: Option<&'r str>,
    // pub reverse: Option<ReverseInput>,
    // pub repeat: Option<SliceInput>,
    // pub slice: Option<SliceInput>,
    // pub fade_in: Option<FadeInInput>,
    // pub fade_out: Option<FadeOutInput>,
    // pub concat: Option<ConcatInput>,
}

#[derive(Debug, Default)]
pub struct CyberpunkEndpoint<'r> {
    pub audio: &'r str,
    pub hash: &'r str,
    pub transformations: Transformations<'r>,
}

impl<'r> CyberpunkEndpoint<'r> {
    pub fn process(&self, request_id: Uuid) -> (String, String) {
        debug!("processing endpoint");

        let audio_storage =  AudioStorage::new();

        let (segment, _) = audio_storage.get_segment(self.audio).unwrap();

        // TODO: do processing and transformations

        let supported_formats = vec!["mp3", "wav", "flac"];

        let file_format = match self.transformations.format { 
            Some(file_format) if supported_formats.contains(&file_format) => file_format,
            _ => "mp3",
        };

        let id_str = &request_id.to_string();

        let processed_key = audio_storage.save_segment(id_str, segment, file_format);

        (processed_key, "mp3".to_string())
    }
}
