use std::collections::HashSet;

use wg_2024::packet::{Fragment, MessageContent, ServerType};

const FRAGMENT_DATA_LENGTH: usize = 80;

#[must_use]
pub fn split_message_into_fragments(message_content: MessageContent) -> Vec<Fragment> {
    let serialized = serialize_message_to_bytes(message_content);
    fragmentize_bytes_into_fragments(&serialized)
}

/// # Errors
///
/// Will return `Err` if the deserialized message content has an ID that is not known. In this case you need to add mapping to serialization and deserialization.
pub fn reassemble_fragments_into_message(
    fragments: &[Fragment],
) -> Result<MessageContent, Box<dyn std::error::Error>> {
    let serialized = reassemble_fragments_into_bytes(fragments);
    deserialize_bytes_into_message(&serialized)
}

/// # Panics
///
/// Will panic if message content cannot be serialized
#[must_use]
fn serialize_message_to_bytes(message_content: MessageContent) -> Vec<u8> {
    let (id, byte_vector) = match message_content {
        MessageContent::ReqServerType => (0, vec![]),
        MessageContent::ReqFilesList => (1, vec![]),
        MessageContent::ReqFile(id) => (2, bincode::serialize(&id).unwrap()),
        MessageContent::ReqMedia(id) => (3, bincode::serialize(&id).unwrap()),
        MessageContent::ReqClientList => (4, vec![]),
        MessageContent::ReqRegistrationToChat => (5, vec![]),
        MessageContent::ReqMessageSend { to, message } => {
            (6, bincode::serialize(&(to, message)).unwrap())
        }
        MessageContent::RespServerType(server_type) => match server_type {
            ServerType::Chat => (7, vec![]),
            ServerType::Text => (8, vec![]),
            ServerType::Media => (9, vec![]),
        },
        MessageContent::RespFilesList(list) => (10, bincode::serialize(&list).unwrap()),
        MessageContent::RespFile(file) => (11, bincode::serialize(&file).unwrap()),
        MessageContent::RespMedia(media) => (12, bincode::serialize(&media).unwrap()),
        MessageContent::ErrUnsupportedRequestType => (13, vec![]),
        MessageContent::ErrRequestedNotFound => (14, vec![]),
        MessageContent::RespClientList(client_list) => {
            (15, bincode::serialize(&client_list).unwrap())
        }
        MessageContent::RespMessageFrom { from, message } => {
            (16, bincode::serialize(&(from, message)).unwrap())
        }
        MessageContent::ErrWrongClientId => (17, vec![]),
    };

    let mut result = vec![id];
    result.extend(byte_vector);
    result
}

/// # Panics
///
/// Will panic if `fragments` is empty or contains duplicate fragment indexes.
#[must_use]
fn reassemble_fragments_into_bytes(fragments: &[Fragment]) -> Vec<u8> {
    // a quick check that the indexes are unique
    let mut existing = HashSet::new();
    for fragment in fragments {
        assert!(
            existing.insert(fragment.fragment_index),
            "Fragments cannot be reassemble due to duplicate fragment(s)"
        );
    }

    // sorted will make life easier
    let mut fragments = fragments.to_vec();
    fragments.sort_by_key(|f| f.fragment_index);

    let size_for_alloc = if fragments.len() == 1 {
        /*
        In the case there is only one fragment, then the size
        shall be equal to the size of the fragment data.
        Panic if no first element exists.
         */
        fragments.first().unwrap().length as usize
    } else {
        /*
        In the case there are multiple fragments, then the size
        shall be equal to amount of fragments - 1 multiplied by
        the fragment data length constant plus the size of the
        last fragment data.
        */
        let size_of_last_fragment = fragments.last().unwrap().length as usize;
        let size_of_full_fragments = (fragments.len() - 1) * FRAGMENT_DATA_LENGTH;
        size_of_last_fragment + size_of_full_fragments
    };

    // it's efficient to initialize with zeros:
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#examples
    let mut byte_vector = vec![0; size_for_alloc];

    for (index, fragment) in fragments.iter().enumerate() {
        let offset = index * FRAGMENT_DATA_LENGTH;
        if fragment.length == 80 {
            byte_vector.splice(offset.., fragment.data);
        } else {
            byte_vector.splice(
                offset..,
                fragment.data[..fragment.length as usize].iter().copied(),
            );
        }
    }

    byte_vector
}

#[must_use]
fn fragmentize_bytes_into_fragments(byte_vector: &[u8]) -> Vec<Fragment> {
    byte_vector
        .chunks(FRAGMENT_DATA_LENGTH)
        .enumerate()
        .map(|(index, chunk)| {
            // not the most efficient way to deal with this but it works
            let mut byte_array = [0; 80];
            byte_array[..chunk.len()].copy_from_slice(chunk);

            Fragment {
                fragment_index: index as u64,
                total_n_fragments: byte_vector.len().div_ceil(FRAGMENT_DATA_LENGTH) as u64,
                #[allow(clippy::cast_possible_truncation)]
                length: chunk.len() as u8, // chunk's length is max FRAGMENT_DATA_LENGTH
                data: byte_array,
            }
        })
        .collect()
}

/// # Errors
///
/// Will return `Err` if the deserialized message content has an ID that is not known.
fn deserialize_bytes_into_message(
    byte_vector: &[u8],
) -> Result<MessageContent, Box<dyn std::error::Error>> {
    let id = &byte_vector[0];
    let mut serialized_data = &byte_vector[1..];

    let message_content = match id {
        0 => MessageContent::ReqServerType,
        1 => MessageContent::ReqFilesList,
        2 => MessageContent::ReqFile(bincode::deserialize_from(&mut serialized_data)?),
        3 => MessageContent::ReqMedia(bincode::deserialize_from(&mut serialized_data)?),
        4 => MessageContent::ReqClientList,
        5 => MessageContent::ReqRegistrationToChat,
        6 => {
            let (to, message) = bincode::deserialize_from(&mut serialized_data)?;
            MessageContent::ReqMessageSend { to, message }
        }
        7 => MessageContent::RespServerType(ServerType::Chat),
        8 => MessageContent::RespServerType(ServerType::Text),
        9 => MessageContent::RespServerType(ServerType::Media),
        10 => MessageContent::RespFilesList(bincode::deserialize_from(&mut serialized_data)?),
        11 => MessageContent::RespFile(bincode::deserialize_from(&mut serialized_data)?),
        12 => MessageContent::RespMedia(bincode::deserialize_from(&mut serialized_data)?),
        13 => MessageContent::ErrUnsupportedRequestType,
        14 => MessageContent::ErrRequestedNotFound,
        15 => MessageContent::RespClientList(bincode::deserialize_from(&mut serialized_data)?),
        16 => {
            let (from, message) = bincode::deserialize_from(&mut serialized_data)?;
            MessageContent::RespMessageFrom { from, message }
        }
        17 => MessageContent::ErrWrongClientId,
        _ => return Err("Unknown id in serialized data".into()),
    };
    Ok(message_content)
}

#[cfg(test)]
mod tests {

    // bring code to be tested into the scope
    use super::*;

    use crate::split_message_into_fragments;

    #[test]
    fn request_client_list() {
        let message_content = MessageContent::ReqClientList;
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::ReqClientList = reassembled {
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn request_server_type() {
        let message_content = MessageContent::ReqServerType;
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::ReqServerType = reassembled {
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }
    #[test]
    fn request_file_list() {
        let message_content = MessageContent::ReqFilesList;
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::ReqFilesList = reassembled {
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn error_about_unsupported_request_type() {
        let message_content = MessageContent::ErrUnsupportedRequestType;
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::ErrUnsupportedRequestType = reassembled {
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn error_about_request_not_found() {
        let message_content = MessageContent::ErrRequestedNotFound;
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::ErrRequestedNotFound = reassembled {
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn request_registration_to_chat() {
        let message_content = MessageContent::ReqRegistrationToChat;
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::ReqRegistrationToChat = reassembled {
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn error_about_wrong_client_id() {
        let message_content = MessageContent::ErrWrongClientId;
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::ErrWrongClientId = reassembled {
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn request_file() {
        let message_content = MessageContent::ReqFile(13);
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();

        if let MessageContent::ReqFile(13) = reassembled {
        } else {
            panic!("Reassembled message content variant or value was something else than the original!");
        }
        if let MessageContent::ReqFile(12) = reassembled {
            panic!("Reassembled message content variant or value was something else than the original!");
        }
    }

    #[test]
    fn request_media() {
        let message_content = MessageContent::ReqMedia(11);
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::ReqMedia(11) = reassembled {
        } else {
            panic!("Reassembled message content variant or value was something else than the original!");
        }
        if let MessageContent::ReqFile(12) = reassembled {
            panic!("Reassembled message content variant or value was something else than the original!");
        }
    }

    #[test]
    fn response_file_list() {
        let original_list = vec![1, 2, 4, 3, 5];
        let message_content = MessageContent::RespFilesList(original_list.clone());
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::RespFilesList(n) = reassembled {
            assert_eq!(n, original_list);
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn response_file() {
        let original_file: Vec<u8> = vec![116, 52, 64, 73, 22, 64, 73, 25];
        let message_content = MessageContent::RespFile(original_file.clone());
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::RespFile(file) = reassembled {
            assert_eq!(file, original_file);
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn response_media() {
        let original_media: Vec<u8> = vec![116, 52, 64, 73, 22, 64, 73, 25];
        let message_content = MessageContent::RespMedia(original_media.clone());
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::RespMedia(media) = reassembled {
            assert_eq!(media, original_media);
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn response_client_list() {
        let original_client_list: Vec<u8> = vec![116, 52, 64, 73, 22, 64, 73, 25];
        let message_content = MessageContent::RespClientList(original_client_list.clone());
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::RespClientList(client_list) = reassembled {
            assert_eq!(client_list, original_client_list);
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn response_server_type_chat() {
        let message_content = MessageContent::RespServerType(ServerType::Chat);
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::RespServerType(ServerType::Chat) = reassembled {
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn response_server_type_media() {
        let message_content = MessageContent::RespServerType(ServerType::Media);
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::RespServerType(ServerType::Media) = reassembled {
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }
    #[test]
    fn response_server_type_text() {
        let message_content = MessageContent::RespServerType(ServerType::Text);
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::RespServerType(ServerType::Text) = reassembled {
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn response_message_from() {
        let original_message: Vec<u8> = vec![116, 52, 64, 73, 22, 64, 73, 25];
        let message_content = MessageContent::RespMessageFrom {
            from: 13,
            message: original_message.clone(),
        };
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::RespMessageFrom { from, message } = reassembled {
            assert_eq!((from, message), (13, original_message));
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }

    #[test]
    fn request_message_send() {
        let original_message: Vec<u8> = vec![116, 52, 64, 73, 22, 64, 73, 25];
        let message_content = MessageContent::ReqMessageSend {
            to: 14,
            message: original_message.clone(),
        };
        let fragmented = split_message_into_fragments(message_content);
        let reassembled = reassemble_fragments_into_message(&fragmented).unwrap();
        if let MessageContent::ReqMessageSend { to, message } = reassembled {
            assert_eq!((to, message), (14, original_message));
        } else {
            panic!("Reassembled message content variant was something else than the original!");
        }
    }
}
