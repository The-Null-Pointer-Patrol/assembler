#[cfg(test)]
mod tests {

    use assembler::naive_assembler::NaiveAssembler;
    use assembler::Assembler;
    use messages::ChatRequest;
    use messages::ChatResponse;
    use messages::DroneSend;
    use messages::MediaRequest;
    use messages::MediaResponse;
    use messages::TextRequest;
    use messages::TextResponse;
    #[test]
    fn fragment_and_reassemble_text_response_with_text() {
        // Construct test message content
        let message = TextResponse::Text("This as a test string.".to_string());
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = TextResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_text_response_with_text_list() {
        // Construct test message content
        let message = TextResponse::TextList(vec![35, 987, 55, 68, 77, 33, 56, 7]);
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = TextResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_text_response_with_not_found() {
        // Construct test message content
        let message = TextResponse::NotFound;
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = TextResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_very_long_text_response() {
        let long_string = "Hello, World".repeat(100_000);
        // Construct test message content
        let message = TextResponse::Text(long_string);
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = TextResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_long_text_response() {
        let long_string = "Hello, World".repeat(1000);
        // Construct test message content
        let message = TextResponse::Text(long_string);
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = TextResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_text_request_text() {
        // Construct test message content
        let message = TextRequest::Text(74);
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = TextRequest::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_text_request_list() {
        // Construct test message content
        let message = TextRequest::TextList;
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = TextRequest::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_media_response_list() {
        // Construct test message content
        let message = MediaResponse::MediaList(vec![56, 87, 97, 66]);
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = MediaResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_media_response() {
        // Construct test message content
        let message = MediaResponse::Media(vec![4, 7, 9, 6, 4, 6, 6, 6, 6, 7, 7, 8]);
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = MediaResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_very_long_media_response() {
        // Construct test message content
        let message = MediaResponse::Media(vec![4; 200_000]);
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = MediaResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_long_media_response() {
        // Construct test message content
        let message = MediaResponse::Media(vec![4; 24000]);
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = MediaResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_media_request_list() {
        // Construct test message content
        let message = MediaRequest::MediaList;
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = MediaRequest::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_media_request_media() {
        // Construct test message content
        let message = MediaRequest::Media(57);
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = MediaRequest::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_chat_request_list() {
        // Construct test message content
        let message = ChatRequest::ClientList;
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = ChatRequest::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_chat_request_register() {
        // Construct test message content
        let message = ChatRequest::Register(8);
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = ChatRequest::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_chat_request_send_message() {
        // Construct test message content
        let message = ChatRequest::SendMessage {
            from: 8,
            to: 3,
            message: String::from("Hello this is a test"),
        };
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = ChatRequest::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_chat_response_list() {
        // Construct test message content
        let message = ChatResponse::ClientList(vec![87, 7, 55, 43, 4, 8]);
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = ChatResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_chat_response_message_from() {
        // Construct test message content
        let message = ChatResponse::MessageFrom {
            from: 8,
            message: vec![16; 204],
        };
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = ChatResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }

    #[test]
    fn fragment_and_reassemble_chat_response_message_sent() {
        // Construct test message content
        let message = ChatResponse::MessageSent;
        // Clone the original content for comparison
        let original_message = message.clone();
        // Create JSON
        let json = message.stringify();
        // Convert JSON to byte vector
        let bytes = json.as_bytes();
        // Create fragments
        let fragments = NaiveAssembler::disassemble(bytes);
        // Reassemble fragments into byte vector
        let bytes = NaiveAssembler::reassemble(&fragments);
        // Convert bytes to String
        let json = String::from_utf8(bytes).unwrap();
        // Deserialize JSON back to original type
        let deserialized = ChatResponse::from_string(json).unwrap();

        assert_eq!(original_message, deserialized);
    }
}
