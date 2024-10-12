/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;
use crate::models::common::*;

/// Struct representing a row in table `operator`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Identifiable)]
#[diesel(table_name=operator, primary_key(id))]
pub struct Operator {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `start`
    pub start: String,
    /// Field representing column `end`
    pub end: String,
    /// Field representing column `operator_type`
    pub operator_type: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `operator` for [`Operator`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=operator)]
pub struct CreateOperator {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `start`
    pub start: String,
    /// Field representing column `end`
    pub end: String,
    /// Field representing column `operator_type`
    pub operator_type: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `operator` for [`Operator`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=operator)]
pub struct UpdateOperator {
    /// Field representing column `name`
    pub name: Option<String>,
    /// Field representing column `address`
    pub address: Option<String>,
    /// Field representing column `start`
    pub start: Option<String>,
    /// Field representing column `end`
    pub end: Option<String>,
    /// Field representing column `operator_type`
    pub operator_type: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Operator {
    /// Insert a new row into `operator` with a given [`CreateOperator`]
    pub fn create(db: &mut ConnectionType, item: &CreateOperator) -> diesel::QueryResult<Self> {
        use crate::schema::operator::dsl::*;

        diesel::insert_into(operator).values(item).get_result::<Self>(db)
    }

    /// Get a row from `operator`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::operator::dsl::*;

        operator.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: OperatorFilter) -> diesel::QueryResult<PaginationResult<Self>> {
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
        filter: OperatorFilter,
    ) -> crate::schema::operator::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::operator::table.into_boxed();
        
        if let Some(filter_id) = filter.id {
            query = query.filter(crate::schema::operator::id.eq(filter_id));
        }
        if let Some(filter_name) = filter.name {
            query = query.filter(crate::schema::operator::name.eq(filter_name));
        }
        if let Some(filter_address) = filter.address {
            query = query.filter(crate::schema::operator::address.eq(filter_address));
        }
        if let Some(filter_start) = filter.start {
            query = query.filter(crate::schema::operator::start.eq(filter_start));
        }
        if let Some(filter_end) = filter.end {
            query = query.filter(crate::schema::operator::end.eq(filter_end));
        }
        if let Some(filter_operator_type) = filter.operator_type {
            query = query.filter(crate::schema::operator::operator_type.eq(filter_operator_type));
        }
        if let Some(filter_status) = filter.status {
            query = query.filter(crate::schema::operator::status.eq(filter_status));
        }
        if let Some(filter_created_at) = filter.created_at {
            query = query.filter(crate::schema::operator::created_at.eq(filter_created_at));
        }
        
        query
    }

    /// Update a row in `operator`, identified by the primary key with [`UpdateOperator`]
    pub fn update(db: &mut ConnectionType, param_id: String, item: &UpdateOperator) -> diesel::QueryResult<Self> {
        use crate::schema::operator::dsl::*;

        diesel::update(operator.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `operator`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::operator::dsl::*;

        diesel::delete(operator.filter(id.eq(param_id))).execute(db)
    }
}
#[derive(Debug, Default, Clone)]
pub struct OperatorFilter {
    pub id: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub operator_type: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
