/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;
use crate::models::common::*;

/// Struct representing a row in table `user`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Identifiable)]
#[diesel(table_name=user, primary_key(id))]
pub struct User {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `verify_id`
    pub verify_id: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `count`
    pub count: i32,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `user` for [`User`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=user)]
pub struct CreateUser {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `verify_id`
    pub verify_id: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `count`
    pub count: i32,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `user` for [`User`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=user)]
pub struct UpdateUser {
    /// Field representing column `name`
    pub name: Option<String>,
    /// Field representing column `address`
    pub address: Option<String>,
    /// Field representing column `verify_id`
    pub verify_id: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
    /// Field representing column `tag`
    pub tag: Option<String>,
    /// Field representing column `count`
    pub count: Option<i32>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl User {
    /// Insert a new row into `user` with a given [`CreateUser`]
    pub fn create(db: &mut ConnectionType, item: &CreateUser) -> diesel::QueryResult<Self> {
        use crate::schema::user::dsl::*;

        diesel::insert_into(user).values(item).get_result::<Self>(db)
    }

    /// Get a row from `user`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::user::dsl::*;

        user.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: UserFilter) -> diesel::QueryResult<PaginationResult<Self>> {
        let page = page.max(0);
        let page_size = page_size.max(1);
        let total_items = Self::filter(filter.clone()).count().get_result(db)?;
        let items = Self::filter(filter).limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// A utility function to help build custom search queries
    /// 
    /// Example:
    /// 
    /// ```
    /// // create a filter for completed todos
    /// let query = Todo::filter(TodoFilter {
    ///     completed: Some(true),
    ///     ..Default::default()
    /// });
    /// 
    /// // delete completed todos
    /// diesel::delete(query).execute(db)?;
    /// ```
    pub fn filter<'a>(
        filter: UserFilter,
    ) -> crate::schema::user::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::user::table.into_boxed();
        
        if let Some(filter_id) = filter.id {
            query = query.filter(crate::schema::user::id.eq(filter_id));
        }
        if let Some(filter_name) = filter.name {
            query = query.filter(crate::schema::user::name.eq(filter_name));
        }
        if let Some(filter_address) = filter.address {
            query = query.filter(crate::schema::user::address.eq(filter_address));
        }
        if let Some(filter_verify_id) = filter.verify_id {
            query = query.filter(crate::schema::user::verify_id.eq(filter_verify_id));
        }
        if let Some(filter_status) = filter.status {
            query = query.filter(crate::schema::user::status.eq(filter_status));
        }
        if let Some(filter_tag) = filter.tag {
            query = query.filter(crate::schema::user::tag.eq(filter_tag));
        }
        if let Some(filter_count) = filter.count {
            query = query.filter(crate::schema::user::count.eq(filter_count));
        }
        if let Some(filter_created_at) = filter.created_at {
            query = query.filter(crate::schema::user::created_at.eq(filter_created_at));
        }
        
        query
    }

    /// Update a row in `user`, identified by the primary key with [`UpdateUser`]
    pub fn update(db: &mut ConnectionType, param_id: String, item: &UpdateUser) -> diesel::QueryResult<Self> {
        use crate::schema::user::dsl::*;

        diesel::update(user.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `user`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::user::dsl::*;

        diesel::delete(user.filter(id.eq(param_id))).execute(db)
    }
}
#[derive(Debug, Default, Clone)]
pub struct UserFilter {
    pub id: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub verify_id: Option<String>,
    pub status: Option<String>,
    pub tag: Option<String>,
    pub count: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
