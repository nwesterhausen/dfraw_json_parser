use crate::traits::RawObject;

use super::unprocessed_raw::UnprocessedRaw;

#[allow(dead_code)]
/// Results from parsing a file. Contains a list of parsed raws and a list of unprocessed raws.
///
/// The unprocessed raws need to be resolved so that they can become parsed raws. This is done
/// by calling `resolve` on an `UnprocessedRaw` object. That requires the entirety of the parsed
/// raws to be passed in, so that it can find the raws it needs to resolve against.
pub struct FileParseResult {
    /// The parsed raws from the file.
    pub parsed_raws: Vec<Box<dyn RawObject>>,
    /// The unprocessed raws from the file. These need to be resolved into parsed raws.
    pub unprocessed_raws: Vec<UnprocessedRaw>,
}
