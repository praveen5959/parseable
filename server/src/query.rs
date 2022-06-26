/*
 * Parseable Server (C) 2022 Parseable, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use chrono::{DateTime, Utc};
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::datasource::file_format::parquet::ParquetFormat;
use datafusion::datasource::listing::ListingOptions;
use datafusion::prelude::*;
use serde_json::Value;
use std::sync::Arc;

use crate::option::CONFIG;
use crate::storage;
use crate::storage::ObjectStorage;
use crate::utils::TimePeriod;
use crate::Error;

fn get_value<'a>(value: &'a Value, key: &'static str) -> Result<&'a str, Error> {
    value
        .get(key)
        .ok_or(Error::JsonQuery(key))?
        .as_str()
        .ok_or(Error::JsonQuery(key))
}

// Query holds all values relevant to a query for a single log stream
pub struct Query {
    pub query: String,
    pub stream_name: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Query {
    // parse_query parses the SQL query and returns the log stream name on which
    // this query is supposed to be executed
    pub fn parse(json: Value) -> Result<Query, Error> {
        // retrieve query, start and end time information from payload.
        // Convert query to lowercase.
        let query = get_value(&json, "query")?.to_lowercase();
        let start_time = get_value(&json, "startTime")?;
        let end_time = get_value(&json, "endTime")?;

        let tokens = query.split(' ').collect::<Vec<&str>>();
        // validate query
        if tokens.is_empty() {
            return Err(Error::Empty);
        } else if tokens.contains(&"join") {
            return Err(Error::Join(query));
        }
        // log stream name is located after the `from` keyword
        let stream_name_index = tokens.iter().position(|&x| x == "from").unwrap() + 1;
        // we currently don't support queries like "select name, address from stream1 and stream2"
        // so if there is an `and` after the first log stream name, we return an error.
        if tokens.len() > stream_name_index + 1 && tokens[stream_name_index + 1] == "and" {
            return Err(Error::MultipleStreams(query));
        }
        let stream_name = tokens[stream_name_index].to_string();

        // Parse time into DateTime
        let start = DateTime::parse_from_rfc3339(start_time)?.into();
        let end = DateTime::parse_from_rfc3339(end_time)?.into();

        Ok(Query {
            stream_name,
            start,
            end,
            query,
        })
    }

    /// Return prefixes, each per day/hour/minutes as necessary
    pub fn get_prefixes(&self) -> Vec<String> {
        TimePeriod::new(self.start, self.end, storage::BLOCK_DURATION)
            .generate_prefixes(&self.stream_name)
    }

    /// Execute query on object storage(and if necessary on cache as well) with given stream information
    pub async fn execute(&self, storage: &impl ObjectStorage) -> Result<Vec<RecordBatch>, Error> {
        let ctx = SessionContext::new();
        storage.query(&ctx, self).await?;

        // query cache only if end_time coulld have been after last sync.
        let duration_since = Utc::now() - self.end;
        if duration_since.num_seconds() < CONFIG.parseable.sync_duration as i64 {
            self.execute_on_cache(&ctx).await?;
        }

        // execute the query and collect results
        let df = ctx.sql(self.query.as_str()).await?;
        let results = df.collect().await.map_err(Error::DataFusion)?;

        Ok(results)
    }

    async fn execute_on_cache(&self, ctx: &SessionContext) -> Result<(), Error> {
        let file_format = ParquetFormat::default().with_enable_pruning(true);

        let listing_options = ListingOptions {
            file_extension: ".parquet".to_owned(),
            format: Arc::new(file_format),
            table_partition_cols: vec![],
            collect_stat: true,
            target_partitions: 1,
        };

        ctx.register_listing_table(
            &self.stream_name,
            CONFIG.parseable.get_cache_path(&self.stream_name).as_str(),
            listing_options,
            None,
        )
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde_json::Value;

    use super::Query;

    #[test]
    fn query_parse_prefix() {
        let query = Value::from_str(
            r#"{
    "query": "SELECT * FROM stream_name",
    "startTime": "2022-10-15T10:00:00+00:00",
    "endTime": "2022-10-15T10:01:00+00:00"
}"#,
        )
        .unwrap();

        let query = Query::parse(query).unwrap();

        assert_eq!(&query.stream_name, "stream_name");
        assert_eq!(
            query.get_prefixes(),
            vec!["stream_name/date=2022-10-15/hour=10/minute=00/".to_string()]
        );
    }
}
