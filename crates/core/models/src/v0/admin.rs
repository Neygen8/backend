use iso8601_timestamp::Timestamp;

use crate::v0::{Report, User};

auto_derived! {
    #[derive(Default)]
    #[cfg_attr(feature = "validator", derive(validator::Validate))]
    pub struct AdminAuditItemCreate {
        /// The ID of the mod who performed the action
        #[cfg_attr(feature="serde", serde(rename="mod"))]
        pub mod_id: String,
        /// The action performed (previously 'permission')
        pub action: String,
        /// The relevant case ID, if applicable
        #[cfg_attr(feature="serde", serde(rename="case"))]
        pub case_id: String,
        /// The id of the target (any object) the action was taken against, if applicable
        #[cfg_attr(feature="serde", serde(rename="target"))]
        pub target_id: Option<String>,
        /// The context attached to the action, if applicable (eg. search phrases)
        pub context: Option<String>,
    }

    pub struct AdminAuditItem {
        /// The audit item ID
        pub id: String,
        /// The ID of the mod who performed the action
        #[cfg_attr(feature="serde", serde(rename="mod"))]
        pub mod_id: String,
        /// The action performed (previously 'permission')
        pub action: String,
        /// The relevant case ID, if applicable
        #[cfg_attr(feature="serde", serde(rename="case"))]
        pub case_id: String,
        /// The id of the target (any object) the action was taken against, if applicable
        #[cfg_attr(feature="serde", serde(rename="target"))]
        pub target_id: Option<String>,
        /// The context attached to the action, if applicable (eg. search phrases)
        pub context: Option<String>,
    }

    #[derive(Default)]
    #[cfg_attr(feature = "validator", derive(validator::Validate))]
    pub struct AdminCaseCommentCreate {
        /// The ID of the case this comment is attached to
        #[cfg_attr(feature="serde", serde(rename="case"))]
        pub case_id: String,
        /// The content
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 2000)))]
        pub content: String
    }

    #[derive(Default)]
    #[cfg_attr(feature = "validator", derive(validator::Validate))]
    pub struct AdminCaseCommentEdit {
        /// The new content
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 2000)))]
        pub content: String
    }

    pub struct AdminCaseComment {
        /// The comment ID
        pub id: String,
        /// The ID of the case this comment is attached to
        #[cfg_attr(feature="serde", serde(rename="case"))]
        pub case_id: String,
        /// The user who posted the comment
        #[cfg_attr(feature="serde", serde(rename="user"))]
        pub user_id: String,
        /// When the comment was edited, if applicable, in iso8601
        pub edited_at: Option<Timestamp>,
        /// The content
        pub content: String
    }

    #[derive(Default)]
    #[cfg_attr(feature = "validator", derive(validator::Validate))]
    pub struct AdminCaseCreate {
        /// The owner of the case. Defaults to the creator
        #[cfg_attr(feature="serde", serde(rename="owner"))]
        pub owner: Option<String>,
        /// The title of the case. If not provided, a default will be generated from the report(s) assigned
        pub title: Option<String>,
        /// The report IDs to initially attach to this case (and generate a default title from)
        #[cfg_attr(feature = "validator", validate(length(min = 1)))]
        pub initial_reports: Vec<String>
    }

    #[derive(Default)]
    pub struct AdminCaseEdit {
        /// The new owner of the case.
        #[cfg_attr(feature="serde", serde(rename="owner"))]
        pub owner_id: Option<String>,
        /// The new title of the case.
        pub title: Option<String>,
        /// Report IDs to add to the case.
        pub add_reports: Option<Vec<String>>,
        /// Report IDs to remove from the case.
        pub remove_reports: Option<Vec<String>>
    }

    pub struct AdminCase {
        /// The case ID
        pub id: String,
        /// The case Short ID
        pub short_id: String,
        /// The owner of the case
        #[cfg_attr(feature="serde", serde(rename="owner"))]
        pub owner_id: String,
        /// The title of the case
        pub title: String,
        /// The status of the case (open/closed)
        pub status: String,
        /// When the case was closed
        pub closed_at: Option<Timestamp>,
        /// The tags for the case
        pub tags: Vec<String>,
        /// The reports assigned to this case
        pub reports: Vec<Report>
    }


    pub struct AdminObjectNote {
        /// When the note was edited, in iso8601
        pub edited_at: Timestamp,
        /// The last user to edit the note
        #[cfg_attr(feature="serde", serde(rename="last_edited_by"))]
        pub last_edited_by_id: String,
        /// The content of the note
        pub content: String,
    }

    #[cfg_attr(feature = "validator", derive(validator::Validate))]
    pub struct AdminObjectNoteEdit {
        #[cfg_attr(feature = "validator", validate(length(max = 2000)))]
        pub content: String
    }

    pub struct AdminStrike {
        /// The strike ID
        pub id: String,
        /// The object receiving the strike (user/server)
        #[cfg_attr(feature="serde", serde(rename="target"))]
        pub target_id: String,
        /// The moderator who gave the strike
        #[cfg_attr(feature="serde", serde(rename="mod"))]
        pub mod_id: String,
        /// The case the strike was made under
        #[cfg_attr(feature="serde", serde(rename="case"))]
        #[cfg_attr(feature="serde", serde(skip_serializing_if="Option::is_none"))]
        pub case_id: Option<String>,
        /// Action associated with the strike (eg. suspension/ban)
        #[cfg_attr(feature="serde", serde(skip_serializing_if="Option::is_none"))]
        pub associated_action: Option<String>,
        /// Has the strike been removed
        #[cfg_attr(feature="serde", serde(skip_serializing_if="crate::if_false"))]
        pub overruled: bool,
        /// The user-facing reason for the strike
        pub reason: String,
        /// Internal context for the strike
        #[cfg_attr(feature="serde", serde(skip_serializing_if="Option::is_none"))]
        pub mod_context: Option<String>,
    }

    #[cfg_attr(feature = "validator", derive(validator::Validate))]
    pub struct AdminStrikeCreate {
        /// The object receiving the strike (user/server)
        #[cfg_attr(feature="serde", serde(rename="target"))]
        pub target_id: String,
        /// The case the strike was made under
        #[cfg_attr(feature="serde", serde(rename="case"))]
        pub case_id: Option<String>,
        /// Action associated with the strike (eg. suspension/ban)
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 25)))]
        pub associated_action: Option<String>,
        /// The user-facing reason for the strike
        #[cfg_attr(feature = "validator", validate(length(max = 2000)))]
        pub reason: String,
        /// Internal context for the strike
        #[cfg_attr(feature = "validator", validate(length(max = 2000)))]
        pub mod_context: Option<String>,
    }

    #[cfg_attr(feature = "validator", derive(validator::Validate))]
    pub struct AdminStrikeEdit {
        /// The case the strike was made under
        #[cfg_attr(feature="serde", serde(rename="case"))]
        pub case_id: Option<String>,
        /// Action associated with the strike (eg. suspension/ban)
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 25)))]
        pub associated_action: Option<String>,
        /// The user-facing reason for the strike
        #[cfg_attr(feature = "validator", validate(length(max = 2000)))]
        pub reason: Option<String>,
        /// Internal context for the strike
        #[cfg_attr(feature = "validator", validate(length(max = 2000)))]
        pub mod_context: Option<String>,
    }

    pub struct AdminToken {
        /// The token ID
        pub id: String,
        /// The user this token is attached to
        #[cfg_attr(feature = "serde", serde(rename="user"))]
        pub user_id: String,
        /// The token itself
        pub token: String,
        /// The expiry timestamp for this token, in iso6801
        pub expiry: Timestamp
    }

    #[cfg_attr(feature = "validator", derive(validator::Validate))]
    pub struct AdminTokenCreate {
        /// The expiry timestamp for this token, in iso6801. Max 30 days from current time.
        pub expiry: Timestamp,
    }

    pub struct AdminUser {
        /// The ID of the user
        pub id: String,
        /// The user's revolt ID.
        pub platform_user_id: String,
        /// The user's email
        pub email: String,
        /// Whether the user is active or not (ie. can they use the api)
        pub active: bool,
        /// The permissions of the user
        pub permissions: u64,
        /// The Revolt user attached to this user. Use this for data like names and avatars.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub revolt_user: Option<User>
    }

    #[cfg_attr(feature = "validator", derive(validator::Validate))]
    pub struct AdminUserCreate {
        /// The user's revolt ID.
        #[cfg_attr(feature = "validator", validate(length(min = 10, max = 20)))]
        pub platform_user_id: String,
        /// The user's email
        #[cfg_attr(feature = "validator", validate(email))]
        pub email: String,
        /// Whether the user is active or not (ie. can they use the api)
        pub active: bool,
        /// The permissions of the user
        pub permissions: u64,
    }

    #[cfg_attr(feature = "validator", derive(validator::Validate))]
    pub struct AdminUserEdit {
        /// The user's revolt ID.
        #[cfg_attr(feature = "validator", validate(length(min = 10, max = 20)))]
        pub platform_user_id: Option<String>,
        /// The user's email
        #[cfg_attr(feature = "validator", validate(email))]
        pub email: Option<String>,
        /// Whether the user is active or not (ie. can they use the api)
        pub active: Option<bool>,
        /// The permissions of the user
        pub permissions: Option<u64>,
    }

    #[repr(u64)]
    pub enum AdminUserPermissionFlags {
        ObjectNotes = 0,

        ManageAdminUsers = 1,
        CreateTokens = 2,

        ViewUsers = 3,
        ManageUsers = 4,
        ManageUsersSenstiveInfo = 5,

        ViewServers = 6,
        ManageServers = 7,
        ViewDMChannels = 8,
        ManageDMChannels = 9,

        ViewCases = 10,
        ManageOwnCases = 11,
        ManageOtherCases = 12,

        ViewReports = 13,

        Search = 14,
        ManageNotes = 15,
        Discover = 16,
    }
}
