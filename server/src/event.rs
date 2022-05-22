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
 *
 */

use arrow::json;
use arrow::json::reader::infer_json_schema;
use arrow::record_batch::RecordBatch;
use bytes::Bytes;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs;
use std::io::{BufReader, Cursor, Seek, SeekFrom, Write};
use std::sync::Arc;

use crate::mem_store;
use crate::response;
use crate::storage;
use crate::Error;

// Event holds all values relevant to a single event for a single logstream
pub struct Event {
    pub body: String,
    pub stream_name: String,
    pub path: String,
    pub schema: Bytes,
}

// Events holds the schema related to a each event for a single logstream
pub struct Schema {
    pub arrow_schema: arrow::datatypes::Schema,
    pub string_schema: String,
}

impl Event {
    pub fn process(&self) -> Result<response::EventResponse, Error> {
        // If the .schema file is still empty, this is the first event in this logstream.
        if self.schema.is_empty() {
            self.initial_event()
        } else {
            self.next_event()
        }
    }

    // This is called when the first event of a LogStream is received. The first event is
    // special because we parse this event to generate the schema for the logstream. This
    // schema is then enforced on rest of the events sent to this logstream.
    fn initial_event(&self) -> Result<response::EventResponse, Error> {
        let mut c = Cursor::new(Vec::new());
        let reader = self.body.as_bytes();

        c.write_all(reader)?;
        c.seek(SeekFrom::Start(0))?;
        let buf_reader = BufReader::new(reader);

        let mut event = json::Reader::new(
            buf_reader,
            Arc::new(self.infer_schema().arrow_schema),
            1024,
            None,
        );
        let b1 = event.next()?.ok_or(Error::MissingRecord)?;

        // Put the event into in memory store
        mem_store::MEM_STREAMS::put(
            self.stream_name.to_string(),
            mem_store::LogStream {
                schema: Some(self.infer_schema().string_schema),
                rb: Some(b1.clone()),
            },
        );

        // Store record batch to Parquet file on local cache
        self.convert_arrow_parquet(b1);

        // Put the inferred schema to object store
        storage::put_schema(&self.stream_name, self.infer_schema().string_schema).map_err(|e| {
            Error::Event(response::EventError {
                msg: format!(
                    "Failed to upload schema for LogStream {} due to err: {}",
                    self.stream_name, e
                ),
            })
        })?;

        Ok(response::EventResponse {
            msg: format!(
                "Intial Event recieved for LogStream {}, schema uploaded successfully",
                self.stream_name
            ),
        })
    }

    // next_event process all events after the 1st event. Concatenates record batches
    // and puts them in memory store for each event.
    fn next_event(&self) -> Result<response::EventResponse, Error> {
        let mut c = Cursor::new(Vec::new());
        let reader = self.body.as_bytes();
        c.write_all(reader).unwrap();
        c.seek(SeekFrom::Start(0)).unwrap();

        let mut event = json::Reader::new(
            self.body.as_bytes(),
            Arc::new(self.infer_schema().arrow_schema),
            1024,
            None,
        );
        let next_event_rb = event.next().unwrap().unwrap();

        let rb = mem_store::MEM_STREAMS::get_rb(self.stream_name.clone())?;

        let vec = vec![next_event_rb.clone(), rb];
        let new_batch = RecordBatch::concat(&next_event_rb.schema(), &vec);

        let rb = new_batch.map_err(|e| {
            Error::Event(response::EventError {
                msg: format!("Error recieved for LogStream {}, {}", &self.stream_name, e),
            })
        })?;

        mem_store::MEM_STREAMS::put(
            self.stream_name.clone(),
            mem_store::LogStream {
                schema: Some(mem_store::MEM_STREAMS::get_schema(self.stream_name.clone())),
                rb: Some(rb.clone()),
            },
        );

        self.convert_arrow_parquet(rb);

        Ok(response::EventResponse {
            msg: format!("Event recieved for LogStream {}", &self.stream_name),
        })
    }

    // inferSchema is a constructor to Schema
    // returns raw arrow schema type and arrow schema to string type.
    fn infer_schema(&self) -> Schema {
        let reader = self.body.as_bytes();
        let mut buf_reader = BufReader::new(reader);
        let inferred_schema = infer_json_schema(&mut buf_reader, None).unwrap();
        let str_inferred_schema = serde_json::to_string(&inferred_schema).unwrap();

        Schema {
            arrow_schema: inferred_schema,
            string_schema: str_inferred_schema,
        }
    }

    // convert arrow record batch to parquet
    // and write it to local cache path as a data.parquet file.
    fn convert_arrow_parquet(&self, rb: RecordBatch) {
        let dir_name = format!("{}{}{}", &self.path, "/", &self.stream_name);
        let file_name = format!("{}{}{}", dir_name, "/", "data.parquet");
        fs::create_dir_all(dir_name).unwrap();
        let parquet_file = fs::File::create(file_name);
        let props = WriterProperties::builder().build();
        let mut writer = ArrowWriter::try_new(
            parquet_file.unwrap(),
            Arc::new(self.infer_schema().arrow_schema),
            Some(props),
        )
        .unwrap();
        writer.write(&rb).expect("Writing batch");
        writer.close().unwrap();
    }
}