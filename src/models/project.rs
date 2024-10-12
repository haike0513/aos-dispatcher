/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;
use crate::models::common::*;

/// Struct representing a row in table `project`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Identifiable)]
#[diesel(table_name=project, primary_key(id))]
pub struct Project {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `token`
    pub token: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `project` for [`Project`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=project)]
pub struct CreateProject {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `token`
    pub token: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `project` for [`Project`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=project)]
pub struct UpdateProject {
    /// Field representing column `name`
    pub name: Option<String>,
    /// Field representing column `address`
    pub address: Option<String>,
    /// Field representing column `token`
    pub token: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Project {
    /// Insert a new row into `project` with a given [`CreateProject`]
    pub fn create(db: &mut ConnectionType, item: &CreateProject) -> diesel::QueryResult<Self> {
        use crate::schema::project::dsl::*;

        diesel::insert_into(project).values(item).get_result::<Self>(db)
    }

    /// Get a row from `project`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::project::dsl::*;

        project.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: ProjectFilter) -> diesel::QueryResult<PaginationResult<Self>> {
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
        filter: ProjectFilter,
    ) -> crate::schema::project::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::project::table.into_boxed();
        
        if let Some(filter_id) = filter.id {
            query = query.filter(crate::schema::project::id.eq(filter_id));
        }
        if let Some(filter_name) = filter.name {
            query = query.filter(crate::schema::project::name.eq(filter_name));
        }
        if let Some(filter_address) = filter.address {
            query = query.filter(crate::schema::project::address.eq(filter_address));
        }
        if let Some(filter_token) = filter.token {
            query = query.filter(crate::schema::project::token.eq(filter_token));
        }
        if let Some(filter_status) = filter.status {
            query = query.filter(crate::schema::project::status.eq(filter_status));
        }
        if let Some(filter_created_at) = filter.created_at {
            query = query.filter(crate::schema::project::created_at.eq(filter_created_at));
        }
        
        query
    }

    /// Update a row in `project`, identified by the primary key with [`UpdateProject`]
    pub fn update(db: &mut ConnectionType, param_id: String, item: &UpdateProject) -> diesel::QueryResult<Self> {
        use crate::schema::project::dsl::*;

        diesel::update(project.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `project`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::project::dsl::*;

        diesel::delete(project.filter(id.eq(param_id))).execute(db)
    }
}
#[derive(Debug, Default, Clone)]
pub struct ProjectFilter {
    pub id: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub token: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
