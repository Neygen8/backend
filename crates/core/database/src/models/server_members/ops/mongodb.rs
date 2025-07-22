use std::collections::HashMap;

use futures::StreamExt;
use mongodb::options::ReadConcern;
use revolt_result::Result;

use crate::{AbstractUsers, FieldsMember, Member, MemberCompositeKey, PartialMember};
use crate::{IntoDocumentPath, MongoDb};

use super::{AbstractServerMembers, ChunkedServerMembersGenerator};

static COL: &str = "server_members";

#[async_trait]
impl AbstractServerMembers for MongoDb {
    /// Insert a new server member into the database
    async fn insert_member(&self, member: &Member) -> Result<()> {
        query!(self, insert_one, COL, &member).map(|_| ())
    }

    /// Fetch a server member by their id
    async fn fetch_member(&self, server_id: &str, user_id: &str) -> Result<Member> {
        query!(
            self,
            find_one,
            COL,
            doc! {
                "_id.server": server_id,
                "_id.user": user_id
            }
        )?
        .ok_or_else(|| create_error!(NotFound))
    }

    /// Fetch all members in a server
    async fn fetch_all_members<'a>(&self, server_id: &str) -> Result<Vec<Member>> {
        Ok(self
            .col::<Member>(COL)
            .find(doc! {
                "_id.server": server_id
            })
            .await
            .map_err(|_| create_database_error!("find", COL))?
            .filter_map(|s| async {
                if cfg!(debug_assertions) {
                    Some(s.unwrap())
                } else {
                    s.ok()
                }
            })
            .collect()
            .await)
    }

    /// Fetch all members in a server as a generator.
    /// Uses config key pushd.mass_mention_chunk_size as the batch size.
    async fn fetch_all_members_chunked(
        &self,
        server_id: &str,
    ) -> Result<ChunkedServerMembersGenerator> {
        let config = revolt_config::config().await;

        let mut session = self
            .start_session()
            .await
            .map_err(|_| create_database_error!("start_session", COL))?;

        session
            .start_transaction()
            .read_concern(ReadConcern::snapshot())
            .await
            .map_err(|_| create_database_error!("start_transaction", COL))?;

        let cursor = self
            .col::<Member>(COL)
            .find(doc! {
                "_id.server": server_id
            })
            .session(&mut session)
            .batch_size(config.pushd.mass_mention_chunk_size as u32)
            .await
            .map_err(|_| create_database_error!("find", COL))?;

        Ok(ChunkedServerMembersGenerator::new_mongo(session, cursor))
    }

    async fn fetch_all_members_with_roles(
        &self,
        server_id: &str,
        roles: &[String],
    ) -> Result<Vec<Member>> {
        Ok(self
            .col::<Member>(COL)
            .find(doc! {
                "_id.server": server_id,
                "roles": {"$in": roles}
            })
            .await
            .map_err(|_| create_database_error!("find", COL))?
            .filter_map(|s| async {
                if cfg!(debug_assertions) {
                    Some(s.unwrap())
                } else {
                    s.ok()
                }
            })
            .collect()
            .await)
    }

    async fn fetch_all_members_with_roles_chunked(
        &self,
        server_id: &str,
        roles: &[String],
    ) -> Result<ChunkedServerMembersGenerator> {
        let config = revolt_config::config().await;

        let mut session = self
            .start_session()
            .await
            .map_err(|_| create_database_error!("start_session", COL))?;

        session
            .start_transaction()
            .read_concern(ReadConcern::snapshot())
            .await
            .map_err(|_| create_database_error!("start_transaction", COL))?;

        let cursor = self
            .col::<Member>(COL)
            .find(doc! {
                "_id.server": server_id,
                "roles": {"$in": roles}
            })
            .session(&mut session)
            .batch_size(config.pushd.mass_mention_chunk_size as u32)
            .await
            .map_err(|_| create_database_error!("find", COL))?;

        return Ok(ChunkedServerMembersGenerator::new_mongo(session, cursor));
    }

    /// Fetch all memberships for a user
    async fn fetch_all_memberships<'a>(&self, user_id: &str) -> Result<Vec<Member>> {
        Ok(self
            .col::<Member>(COL)
            .find(doc! {
                "_id.user": user_id
            })
            .await
            .map_err(|_| create_database_error!("find", COL))?
            .filter_map(|s| async {
                if cfg!(debug_assertions) {
                    Some(s.unwrap())
                } else {
                    s.ok()
                }
            })
            .collect()
            .await)
    }

    /// Fetch multiple members by their ids
    async fn fetch_members<'a>(&self, server_id: &str, ids: &'a [String]) -> Result<Vec<Member>> {
        Ok(self
            .col::<Member>(COL)
            .find(doc! {
                "_id.server": server_id,
                "_id.user": {
                    "$in": ids
                }
            })
            .await
            .map_err(|_| create_database_error!("find", COL))?
            .filter_map(|s| async {
                if cfg!(debug_assertions) {
                    Some(s.unwrap())
                } else {
                    s.ok()
                }
            })
            .collect()
            .await)
    }

    /// Fetch member count of a server
    async fn fetch_member_count(&self, server_id: &str) -> Result<usize> {
        self.col::<Member>(COL)
            .count_documents(doc! {
                "_id.server": server_id
            })
            .await
            .map(|c| c as usize)
            .map_err(|_| create_database_error!("count_documents", COL))
    }

    /// Fetch server count of a user
    async fn fetch_server_count(&self, user_id: &str) -> Result<usize> {
        self.col::<Member>(COL)
            .count_documents(doc! {
                "_id.user": user_id
            })
            .await
            .map(|c| c as usize)
            .map_err(|_| create_database_error!("count_documents", COL))
    }

    /// Update information for a server member
    async fn update_member(
        &self,
        id: &MemberCompositeKey,
        partial: &PartialMember,
        remove: Vec<FieldsMember>,
    ) -> Result<()> {
        query!(
            self,
            update_one,
            COL,
            doc! {
                "_id.server": &id.server,
                "_id.user": &id.user
            },
            partial,
            remove.iter().map(|x| x as &dyn IntoDocumentPath).collect(),
            None
        )
        .map(|_| ())
    }

    /// Delete a server member by their id
    async fn delete_member(&self, id: &MemberCompositeKey) -> Result<()> {
        query!(
            self,
            delete_one,
            COL,
            doc! {
                "_id.server": &id.server,
                "_id.user": &id.user
            }
        )
        .map(|_| ())
    }

    async fn fetch_server_participants(
        &self,
        server_id: &str,
    ) -> Result<Vec<(crate::User, Option<Member>)>> {
        let server: crate::Server = query!(self, find_one, "servers", doc! {"_id": server_id})?
            .ok_or_else(|| create_database_error!("find", "servers"))?;
        let ids: Vec<String> = self
            .db()
            .collection::<crate::Message>("messages")
            .distinct("author", doc! {"channel": {"$in": server.channels}})
            .await
            .map_err(|_| create_database_error!("distinct", "messages"))?
            .iter()
            .map(|b| bson::from_bson::<String>(b.clone()).unwrap()) // json encoded string for some logic-defying reason
            .collect();

        println!("{:?}", &ids);

        let users = self.fetch_users(&ids).await?;
        let members = self
            .fetch_members(server_id, &ids)
            .await?
            .iter()
            .map(|f| (f.id.user.clone(), f.clone()))
            .collect::<HashMap<String, Member>>();

        Ok(users
            .iter()
            .map(|u| (u.clone(), members.get(&u.id).cloned()))
            .collect())
    }

    async fn fetch_server_members(
        &self,
        server_id: &str,
        page_size: u8,
        after: Option<usize>,
    ) -> Result<Vec<(crate::User, Member)>> {
        let mut members_stream = if let Some(after) = after {
            self.col::<crate::Member>(COL)
                .find(doc! {"_id.server": server_id, "joined_at": {"$gt": after as i64}})
                .sort(doc! {"joined_at": -1})
                .limit(page_size as i64)
                .await
                .map_err(|_| create_database_error!("find", COL))?
        } else {
            self.col::<crate::Member>(COL)
                .find(doc! {"_id.server": server_id})
                .sort(doc! {"joined_at": -1})
                .limit(page_size as i64)
                .await
                .map_err(|_| create_database_error!("find", COL))?
        };

        let mut members = vec![];
        if members_stream.advance().await.is_ok_and(|f| f) {
            loop {
                if let Ok(x) = members_stream.deserialize_current() {
                    members.push(x);
                    if let Ok(r) = members_stream.advance().await {
                        if !r {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        let user_ids: Vec<String> = members.iter().map(|m| m.id.user.clone()).collect();
        let mut set = HashMap::new();
        self.fetch_users(&user_ids).await?.iter().for_each(|f| {
            set.insert(f.id.clone(), f.clone());
        });

        return Ok(members
            .iter()
            .map(|f| (set.remove(&f.id.user).unwrap(), f.clone()))
            .collect());
    }
}

impl IntoDocumentPath for FieldsMember {
    fn as_path(&self) -> Option<&'static str> {
        Some(match self {
            FieldsMember::Avatar => "avatar",
            FieldsMember::Nickname => "nickname",
            FieldsMember::Roles => "roles",
            FieldsMember::Timeout => "timeout",
        })
    }
}
