use super::extractor_prelude::*;

pub struct Extractor;

impl ExtractorTrait for Extractor {

    fn extract<S: Into<String>>(files: &mut Files, file_path: S, item: &mut ZipFile) -> LLResult<()> {

        generic_extractor("eagle", files, file_path, item)

    }

}
