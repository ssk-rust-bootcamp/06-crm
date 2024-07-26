use chrono::{DateTime, TimeZone, Utc};
use itertools::Itertools;
use prost_types::Timestamp;
use tonic::{Response, Status};

use crate::{
    pb::{QueryRequest, RowQueryRequest, User},
    ResponseStream, ServiceResult, UserStatsService,
};

impl UserStatsService {
    pub async fn query(&self, query: QueryRequest) -> ServiceResult<ResponseStream> {
        // generate sql query from query request
        let mut sql = "SELECT email,name from user_stats where ".to_string();
        let time_condition = query
            .timestamps
            .into_iter()
            .map(|(k, v)| timestamp_query(&k, v.lower, v.upper))
            .join(" AND ");

        sql.push_str(&time_condition);
        let id_condition = query
            .ids
            .into_iter()
            .map(|(k, v)| ids_query(&k, v.ids))
            .join(" AND ");

        sql.push_str(" AND ");
        sql.push_str(&id_condition);
        println!("Generated SQL query: {}", sql);

        self.raw_query(RowQueryRequest { query: sql }).await
    }

    pub async fn raw_query(&self, req: RowQueryRequest) -> ServiceResult<ResponseStream> {
        // TODO: query must only return email and name, so we should use sqlparser to parse the query
        let Ok(ret) = sqlx::query_as::<_, User>(&req.query)
            .fetch_all(&self.inner.pool)
            .await
        else {
            return Err(Status::internal(format!(
                "Failed to fetch data with query: {}",
                req.query
            )));
        };

        Ok(Response::new(Box::pin(futures::stream::iter(
            ret.into_iter().map(Ok),
        ))))
    }
}

fn ids_query(name: &str, ids: Vec<u32>) -> String {
    if ids.is_empty() {
        return "TRUE".to_string();
    }
    format!("array{:?} <@ {}", ids, name)
}

fn timestamp_query(name: &str, lower: Option<Timestamp>, upper: Option<Timestamp>) -> String {
    if lower.is_none() && upper.is_none() {
        return "TRUE".to_string();
    }
    if lower.is_none() {
        let upper = to_use_utc(upper.unwrap());
        return format!("{} <= {}", name, upper.to_rfc3339());
    }
    if upper.is_none() {
        let lower = ts_to_utc(lower.unwrap());
        return format!("{} >= '{}'", name, lower.to_rfc3339());
    }

    format!(
        "{} BETWEEN '{}' AND '{}'",
        name,
        ts_to_utc(upper.unwrap()).to_rfc3339(),
        ts_to_utc(lower.unwrap()).to_rfc3339()
    )
}
fn ts_to_utc(ts: Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(ts.seconds, ts.nanos as _).unwrap()
}
fn to_use_utc(ts: Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(ts.seconds, ts.nanos as _).unwrap()
}
