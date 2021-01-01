use crate::parsers::PartiallyDeserializedOSQueryLog;
use sqs_executor::event_decoder::PayloadDecoder;
use tokio::fs;

use grapl_service::decoder::{ZstdJsonDecoder, ZstdJsonDecoderError};

pub(crate) async fn read_osquery_test_data(
    path: &str,
) -> Result<Vec<PartiallyDeserializedOSQueryLog>, ZstdJsonDecoderError> {
    let mut file_data = fs::read(format!("test_data/{}", path))
        .await
        .expect(&format!("Failed to read test data ({}).", path));
    let mut deserializer = ZstdJsonDecoder::default();

    let decoded: Vec<_> = deserializer.decode(file_data)?;
    Ok(decoded)
}
