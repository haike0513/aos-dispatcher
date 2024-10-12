/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::models::job_request::JobRequest;
use crate::schema::*;
use crate::models::common::*;

/// Struct representing a row in table `job_result`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Associations, diesel::Identifiable)]
#[diesel(table_name=job_result, primary_key(id), belongs_to(JobRequest, foreign_key=job_id))]
pub struct JobResult {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `job_id`
    pub job_id: String,
    /// Field representing column `operator`
    pub operator: String,
    /// Field representing column `result`
    pub result: serde_json::Value,
    /// Field representing column `vrf`
    pub vrf: serde_json::Value,
    /// Field representing column `verify_id`
    pub verify_id: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `clock`
    pub clock: serde_json::Value,
    /// Field representing column `signature`
    pub signature: String,
    /// Field representing column `job_type`
    pub job_type: String,
    /// Field representing column `reason`
    pub reason: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `job_result` for [`JobResult`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=job_result)]
pub struct CreateJobResult {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `job_id`
    pub job_id: String,
    /// Field representing column `operator`
    pub operator: String,
    /// Field representing column `result`
    pub result: serde_json::Value,
    /// Field representing column `vrf`
    pub vrf: serde_json::Value,
    /// Field representing column `verify_id`
    pub verify_id: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `clock`
    pub clock: serde_json::Value,
    /// Field representing column `signature`
    pub signature: String,
    /// Field representing column `job_type`
    pub job_type: String,
    /// Field representing column `reason`
    pub reason: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `job_result` for [`JobResult`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=job_result)]
pub struct UpdateJobResult {
    /// Field representing column `job_id`
    pub job_id: Option<String>,
    /// Field representing column `operator`
    pub operator: Option<String>,
    /// Field representing column `result`
    pub result: Option<serde_json::Value>,
    /// Field representing column `vrf`
    pub vrf: Option<serde_json::Value>,
    /// Field representing column `verify_id`
    pub verify_id: Option<String>,
    /// Field representing column `tag`
    pub tag: Option<String>,
    /// Field representing column `clock`
    pub clock: Option<serde_json::Value>,
    /// Field representing column `signature`
    pub signature: Option<String>,
    /// Field representing column `job_type`
    pub job_type: Option<String>,
    /// Field representing column `reason`
    pub reason: Option<String>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl JobResult {
    /// Insert a new row into `job_result` with a given [`CreateJobResult`]
    pub fn create(db: &mut ConnectionType, item: &CreateJobResult) -> diesel::QueryResult<Self> {
        use crate::schema::job_result::dsl::*;

        diesel::insert_into(job_result).values(item).get_result::<Self>(db)
    }

    /// Get a row from `job_result`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::job_result::dsl::*;

        job_result.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: JobResultFilter) -> diesel::QueryResult<PaginationResult<Self>> {
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
        filter: JobResultFilter,
    ) -> crate::schema::job_result::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::job_result::table.into_boxed();
        
        if let Some(filter_id) = filter.id {
            query = query.filter(crate::schema::job_result::id.eq(filter_id));
        }
        if let Some(filter_job_id) = filter.job_id {
            query = query.filter(crate::schema::job_result::job_id.eq(filter_job_id));
        }
        if let Some(filter_operator) = filter.operator {
            query = query.filter(crate::schema::job_result::operator.eq(filter_operator));
        }
        if let Some(filter_result) = filter.result {
            query = query.filter(crate::schema::job_result::result.eq(filter_result));
        }
        if let Some(filter_vrf) = filter.vrf {
            query = query.filter(crate::schema::job_result::vrf.eq(filter_vrf));
        }
        if let Some(filter_verify_id) = filter.verify_id {
            query = query.filter(crate::schema::job_result::verify_id.eq(filter_verify_id));
        }
        if let Some(filter_tag) = filter.tag {
            query = query.filter(crate::schema::job_result::tag.eq(filter_tag));
        }
        if let Some(filter_clock) = filter.clock {
            query = query.filter(crate::schema::job_result::clock.eq(filter_clock));
        }
        if let Some(filter_signature) = filter.signature {
            query = query.filter(crate::schema::job_result::signature.eq(filter_signature));
        }
        if let Some(filter_job_type) = filter.job_type {
            query = query.filter(crate::schema::job_result::job_type.eq(filter_job_type));
        }
        if let Some(filter_reason) = filter.reason {
            query = query.filter(crate::schema::job_result::reason.eq(filter_reason));
        }
        if let Some(filter_created_at) = filter.created_at {
            query = query.filter(crate::schema::job_result::created_at.eq(filter_created_at));
        }
        
        query
    }

    /// Update a row in `job_result`, identified by the primary key with [`UpdateJobResult`]
    pub fn update(db: &mut ConnectionType, param_id: String, item: &UpdateJobResult) -> diesel::QueryResult<Self> {
        use crate::schema::job_result::dsl::*;

        diesel::update(job_result.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `job_result`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::job_result::dsl::*;

        diesel::delete(job_result.filter(id.eq(param_id))).execute(db)
    }
}
#[derive(Debug, Default, Clone)]
pub struct JobResultFilter {
    pub id: Option<String>,
    pub job_id: Option<String>,
    pub operator: Option<String>,
    pub result: Option<serde_json::Value>,
    pub vrf: Option<serde_json::Value>,
    pub verify_id: Option<String>,
    pub tag: Option<String>,
    pub clock: Option<serde_json::Value>,
    pub signature: Option<String>,
    pub job_type: Option<String>,
    pub reason: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
