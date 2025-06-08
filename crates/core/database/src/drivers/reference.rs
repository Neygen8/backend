use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use futures::lock::Mutex;

use crate::{
    AdminAuditItem, AdminCase, AdminCaseComment, AdminObjectNote, AdminStrike, AdminToken,
    AdminUser, Bot, Channel, ChannelCompositeKey, ChannelUnread, Emoji, File, FileHash, Invite,
    Member, MemberCompositeKey, Message, PolicyChange, RatelimitEvent, Report, Server, ServerBan,
    Snapshot, User, UserSettings, Webhook,
};

database_derived!(
    /// Reference implementation
    #[derive(Default)]
    pub struct ReferenceDb {
        pub admin_audits: Arc<Mutex<BTreeMap<String, AdminAuditItem>>>,
        pub admin_cases: Arc<Mutex<HashMap<String, AdminCase>>>,
        pub admin_case_comments: Arc<Mutex<HashMap<String, AdminCaseComment>>>,
        pub admin_object_notes: Arc<Mutex<HashMap<String, AdminObjectNote>>>,
        pub admin_strikes: Arc<Mutex<HashMap<String, AdminStrike>>>,
        pub admin_tokens: Arc<Mutex<HashMap<String, AdminToken>>>,
        pub admin_users: Arc<Mutex<HashMap<String, AdminUser>>>,
        pub bots: Arc<Mutex<HashMap<String, Bot>>>,
        pub channels: Arc<Mutex<HashMap<String, Channel>>>,
        pub channel_invites: Arc<Mutex<HashMap<String, Invite>>>,
        pub channel_unreads: Arc<Mutex<HashMap<ChannelCompositeKey, ChannelUnread>>>,
        pub channel_webhooks: Arc<Mutex<HashMap<String, Webhook>>>,
        pub emojis: Arc<Mutex<HashMap<String, Emoji>>>,
        pub file_hashes: Arc<Mutex<HashMap<String, FileHash>>>,
        pub files: Arc<Mutex<HashMap<String, File>>>,
        pub messages: Arc<Mutex<HashMap<String, Message>>>,
        pub policy_changes: Arc<Mutex<HashMap<String, PolicyChange>>>,
        pub ratelimit_events: Arc<Mutex<HashMap<String, RatelimitEvent>>>,
        pub user_settings: Arc<Mutex<HashMap<String, UserSettings>>>,
        pub users: Arc<Mutex<HashMap<String, User>>>,
        pub server_bans: Arc<Mutex<HashMap<MemberCompositeKey, ServerBan>>>,
        pub server_members: Arc<Mutex<HashMap<MemberCompositeKey, Member>>>,
        pub servers: Arc<Mutex<HashMap<String, Server>>>,
        pub safety_reports: Arc<Mutex<HashMap<String, Report>>>,
        pub safety_snapshots: Arc<Mutex<HashMap<String, Snapshot>>>,
    }
);
