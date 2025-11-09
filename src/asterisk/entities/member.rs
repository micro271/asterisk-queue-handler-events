use macros::ParserEvent;

///
/// Queue user information
/// This information is received when we are connected to the queue event
///
/// Queue: queue name
///
#[derive(Debug, ParserEvent)]
pub struct QueueMember {
    #[parser(key = "Queue")]
    queue: String,

    #[parser(key = "Interface", key = "StateInterface")]
    interface: String,

    #[parser(key = "MemberName")]
    member_name: String,

    #[parser(key = "Status")]
    status: Status,

    #[parser(key = "LoginTime")]
    log_in_time: String,

    #[parser(key = "LastCall", use_parse)]
    last_call: u64,

    #[parser(key = "LastPause", use_parse)]
    last_pause: u64,

    #[parser(key = "CallsTaken", use_parse)]
    calls_taken: u16,

    #[parser(key = "InCall", use_parse)]
    in_call: bool,

    #[parser(key = "Ringinuse", use_parse)]
    ring_in_use: bool,

    #[parser(key = "PausedReason")]
    pause_reason: String,

    #[parser(key = "Paused", use_parse)]
    paused: bool,
}

// Member status
#[derive(Debug, Default)]
enum Status {
    #[default]
    Unknown,
    NotInUse,
    InUse,
    Busy,
    Invalid,
    Unavailable,
    Ringing,
    RingingAndInUse,
    OnHold,
}

impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value {
            "1" => Self::NotInUse,
            "2" => Self::InUse,
            "3" => Self::Busy,
            "4" => Self::Invalid,
            "5" => Self::Unavailable,
            "6" => Self::Ringing,
            "7" => Self::RingingAndInUse,
            "8" => Self::OnHold,
            _ => Self::Unknown,
        }
    }
}

/// event: QueueMemberEingNoAnswer
///
/// Queue: queue name
/// Interface: agent or endpoint
/// MemberName: member name
/// Position:
/// CallerIDNum:
/// CallerIDName:
/// Uniqueid:
#[derive(Debug, ParserEvent)]
pub struct MemberRingninuse {
    #[parser(key = "Queue")]
    queue: String,

    #[parser(key = "Interface")]
    interface: String,

    #[parser(key = "MemberName")]
    member_name: String,

    #[parser(key = "Position", use_parse)]
    position: u16,

    #[parser(key = "CallerId")]
    caller_id: String,

    #[parser(key = "CallerName")]
    caller_name: String,
}

/// Event: QueueMemberBusy
#[derive(Debug)]
pub struct MemberBusy {
    queue: String,
    interface: String,
    member_name: String,
    position: u16,
    caller_id: String,
    caller_name: String,
}
