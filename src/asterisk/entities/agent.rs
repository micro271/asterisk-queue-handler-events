use macros::ParserEvent;

/// Raised when an queue member is notified of a caller in the queue.
#[derive(Debug, ParserEvent)]
pub struct AgenteCalled {
    #[parser(key = "Queue")]
    queue: String,

    #[parser(key = "MemberName")]
    member_name: String,

    #[parser(key = "Interface")]
    interface: String,

    #[parser(key = "Uniqueid")]
    caller_unique_id: String,

    #[parser(key = "CallerIDNum")]
    caller_id_num: String,

    #[parser(key = "CallerIDName")]
    caller_id_name: String,

    #[parser(key = "DestUniqueid")]
    dest_unique_id: String,

    #[parser(key = "DestCallerIDNum")]
    dest_caller_id_num: String,

    #[parser(key = "DestCallerIDName")]
    dest_caller_id_name: String,

    #[parser(key = "DestConnectedLineNum")]
    dest_connected_line_num: String,

    #[parser(key = "DestConnectedLineName")]
    dest_connected_line_name: String,
}

/// Raised when a queue member answers and is bridged to a caller in the queue.
#[derive(Debug, ParserEvent)]
pub struct AgentConnect {    
    #[parser(key = "Queue")]
    queue: String,
    
    #[parser(key = "Uniqueid")]
    caller_unique_id: String,
    
    #[parser(key = "CallerIDNum")]
    caller_id_num: String,
    
    #[parser(key = "CallerIDName")]
    caller_id_name: String,
    
    #[parser(key = "DestUniqueid")]
    dest_unique_id: String,
    
    #[parser(key = "DestCallerIDNum")]
    dest_caller_id_num: String,
    
    #[parser(key = "DestCallerIDName")]
    dest_caller_id_name: String,
    
    #[parser(key = "DestConnectedLineNum")]
    dest_connected_line_num: String,
    
    #[parser(key = "DestConnectedLineName")]
    dest_connected_line_name: String,
    
    #[parser(key = "Interface")]
    interface: String,
    
    #[parser(key = "MemberName")]
    member_name: String,
    
    #[parser(key = "RingTime", use_parse)]
    ring_time: u64,
    
    #[parser(key = "HoldTime", use_parse)]
    hold_time: u64,
}

// Raised when a queue member has finished servicing a caller in the queue.
#[derive(Debug, ParserEvent)]
pub struct AgentComplete {
    #[parser(key = "Queue")]
    queue: String,

    #[parser(key = "MemberName")]
    member_name: String,

    #[parser(key = "Interface")]
    interface: String,

    #[parser(key = "HoldTime", use_parse)]
    hold_time: u64,

    #[parser(key = "TalkTime", use_parse)]
    talk_time: u64,

    #[parser(key = "Uniqueid")]
    caller_unique_id: String,

    #[parser(key = "CallerIDNum")]
    caller_id_num: String,

    #[parser(key = "CallerIDName")]
    caller_id_name: String,

    #[parser(key = "DestCallerIDNum")]
    dest_caller_id_num: String,

    #[parser(key = "DestCallerIDName")]
    dest_caller_id_name: String,

    #[parser(key = "DestConnectedLineNum")]
    dest_connected_line_num: String,

    #[parser(key = "DestConnectedLineName")]
    dest_connected_line_name: String,

    #[parser(key = "DestUniqueid")]
    dest_unique_id: String,

    #[parser(key = "Reason")]
    reason: String,
}

//Raised when a queue member is notified of a caller in the queue and fails to answer.
#[derive(Debug, ParserEvent)]
pub struct AgentRingNoAnswer {
    #[parser(key = "Queue")]
    queue: String,

    #[parser(key = "MemberName")]
    member_name: String,

    #[parser(key = "Interface")]
    interface: String,

    #[parser(key = "RingTime", use_parse)]
    ring_time: u64,

    #[parser(key = "Uniqueid")]
    caller_unique_id: String,

    #[parser(key = "CallerIDNum")]
    caller_id_num: String,

    #[parser(key = "CallerIDName")]
    caller_id_name: String,

    #[parser(key = "DestCallerIDNum")]
    dest_caller_id_num: String,

    #[parser(key = "DestCallerIDName")]
    dest_caller_id_name: String,

    #[parser(key = "DestConnectedLineNum")]
    dest_connected_line_num: String,

    #[parser(key = "DestConnectedLineName")]
    dest_connected_line_name: String,

    #[parser(key = "DestUniqueid")]
    dest_unique_id: String,
}

// Raised when a queue member hangs up on a caller in the queue.
#[derive(Debug, ParserEvent)]
pub struct AgentDump {

    #[parser(key = "Queue")]
    queue: String,

    #[parser(key = "MemberName")]
    member_name: String,

    #[parser(key = "Interface")]
    interface: String,

    #[parser(key = "Uniqueid")]
    caller_unique_id: String,

    #[parser(key = "CallerIDNum")]
    caller_id_num: String,

    #[parser(key = "CallerIDName")]
    caller_id_name: String,

    #[parser(key = "DestUniqueid")]
    dest_caller_id_num: String,

    #[parser(key = "DestCallerIDNum")]
    dest_caller_id_name: String,

    #[parser(key = "DestCallerIDName")]
    dest_unique_id: String,
}


// Raised when an Agent has logged in.
#[derive(Debug)]
pub struct AgentLogin {
    // ...
}

// Raised when an Agent has logged off.
#[derive(Debug)]
pub struct AgentLogoff;
