use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Messaging {
    pub sender: IdObj,
    pub recipient: IdObj,
    pub timestamp: i64,
    pub message: MessageData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessageData {
    StoryCommentMessage(StoryCommentMessage),
    StoryMentionMessage(StoryMentionMessage),
    QuickReplyMessage(QuickReplyMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoryCommentMessage {
    pub reply_to: ReplyTo,
    pub mid: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoryMentionMessage {
    pub attachments: Vec<Attachment>,
    pub mid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuickReplyMessage {
    pub quick_reply: QuickReply,
    pub mid: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdObj {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplyTo {
    pub mid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub type_: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuickReply {
    pub payload: String,
}

fn main() {
    let json_data = r#"
    {
        "sender": {
            "id": "6393353920780174"
        },
        "recipient": {
            "id": "17841408678837640"
        },
        "timestamp": 1702954369304,
        "message": {
            "mid": "xxxxxxx",
            "text": "text1",
            "quick_reply": {
                "payload": "DEVELOPER_TEST_HERE_1"
            }
        }
    }
    "#;

    let messaging: Messaging = serde_json::from_str(json_data).unwrap();
    println!("{:?}", messaging);
}
