use crate::util::basic::transform_optional_string;

auto_derived! {
    pub struct AdminAuditItem {
        /// The audit item ID
        #[serde(rename = "_id")]
        pub id: String,
        /// The moderator who performed the action
        pub mod_id: String,
        /// The action performed (previously 'permission')
        pub action: String,
        /// The relevant case ID, if applicable
        pub case_id: Option<String>,
        /// The object the action was taken against, if applicable
        pub target_id: Option<String>,
        /// The context of the action, if applicable (eg. search phrases)
        pub context: Option<String>,
    }
}

impl AdminAuditItem {
    pub fn new(
        mod_id: &str,
        action: &str,
        case_id: Option<&str>,
        target_id: Option<&str>,
        context: Option<&str>,
    ) -> AdminAuditItem {
        let id = ulid::Ulid::new().to_string();
        AdminAuditItem {
            id,
            mod_id: mod_id.to_string(),
            action: action.to_string(),
            case_id: transform_optional_string(case_id),
            target_id: transform_optional_string(target_id),
            context: transform_optional_string(context),
        }
    }
}
