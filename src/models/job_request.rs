/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;
use crate::models::common::*;

/// Struct representing a row in table `job_request`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Identifiable)]
#[diesel(table_name=job_request, primary_key(id))]
pub struct JobRequest {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `user`
    pub user: String,
    /// Field representing column `job`
    pub job: serde_json::Value,
    /// Field representing column `clock`
    pub clock: serde_json::Value,
    /// Field representing column `job_type`
    pub job_type: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `job_request` for [`JobRequest`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=job_request)]
pub struct CreateJobRequest {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `user`
    pub user: String,
    /// Field representing column `job`
    pub job: serde_json::Value,
    /// Field representing column `clock`
    pub clock: serde_json::Value,
    /// Field representing column `job_type`
    pub job_type: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `job_request` for [`JobRequest`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=job_request)]
pub struct UpdateJobRequest {
    /// Field representing column `user`
    pub user: Option<String>,
    /// Field representing column `job`
    pub job: Option<serde_json::Value>,
    /// Field representing column `clock`
    pub clock: Option<serde_json::Value>,
    /// Field representing column `job_type`
    pub job_type: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
    /// Field representing column `tag`
    pub tag: Option<String>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl JobRequest {
    /// Insert a new row into `job_request` with a given [`CreateJobRequest`]
    pub fn create(db: &mut ConnectionType, item: &CreateJobRequest) -> diesel::QueryResult<Self> {
        use crate::schema::job_request::dsl::*;

        diesel::insert_into(job_request).values(item).get_result::<Self>(db)
    }

    /// Get a row from `job_request`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::job_request::dsl::*;

        job_request.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: JobRequestFilter) -> diesel::QueryResult<PaginationResult<Self>> {
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
        filter: JobRequestFilter,
    ) -> crate::schema::job_request::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::job_request::table.into_boxed();
        
        if let Some(filter_id) = filter.id {
            query = query.filter(crate::schema::job_request::id.eq(filter_id));
        }
        if let Some(filter_user) = filter.user {
            query = query.filter(crate::schema::job_request::user.eq(filter_user));
        }
        if let Some(filter_job) = filter.job {
            query = query.filter(crate::schema::job_request::job.eq(filter_job));
        }
        if let Some(filter_clock) = filter.clock {
            query = query.filter(crate::schema::job_request::clock.eq(filter_clock));
        }
        if let Some(filter_job_type) = filter.job_type {
            query = query.filter(crate::schema::job_request::job_type.eq(filter_job_type));
        }
        if let Some(filter_status) = filter.status {
            query = query.filter(crate::schema::job_request::status.eq(filter_status));
        }
        if let Some(filter_tag) = filter.tag {
            query = query.filter(crate::schema::job_request::tag.eq(filter_tag));
        }
        if let Some(filter_created_at) = filter.created_at {
            query = query.filter(crate::schema::job_request::created_at.eq(filter_created_at));
        }
        
        query
    }

    /// Update a row in `job_request`, identified by the primary key with [`UpdateJobRequest`]
    pub fn update(db: &mut ConnectionType, param_id: String, item: &UpdateJobRequest) -> diesel::QueryResult<Self> {
        use crate::schema::job_request::dsl::*;

        diesel::update(job_request.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `job_request`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::job_request::dsl::*;

        diesel::delete(job_request.filter(id.eq(param_id))).execute(db)
    }
}
#[derive(Debug, Default, Clone)]
pub struct JobRequestFilter {
    pub id: Option<String>,
    pub user: Option<String>,
    pub job: Option<serde_json::Value>,
    pub clock: Option<serde_json::Value>,
    pub job_type: Option<String>,
    pub status: Option<String>,
    pub tag: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
