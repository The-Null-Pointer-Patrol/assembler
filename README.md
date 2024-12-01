This repository contains the assembler library for the group project.

# Overview
The assembler has two public functions:
1. `pub fn split_message_into_fragments(message_content: MessageContent) -> Vec<Fragment>`
2. `pub fn reassemble_fragments_into_message(fragments: &[Fragment],) -> Result<MessageContent, Box<dyn std::error::Error>>`

One creates takes `MessageContent` as an argument, and returns a vector of `Fragment`s. The other takes a slice `&[Fragment]` and returns, in most cases, a `MessageContent`.

# Usage
Usage is simple as the overview sounds like
```rust
let message_content = MessageContent::ReqMessageSend {
    to: 12,
    message: "This is a test message to a drone with ID 12"
        .to_string()
        .into_bytes(),
};

let fragmentized_message_content = assembler::split_message_into_fragments(message_content);
let reassembled_message_content = assembler::reassemble_fragments_into_message(&fragments).unwrap();
```
