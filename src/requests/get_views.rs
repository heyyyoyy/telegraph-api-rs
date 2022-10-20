use std::sync::Arc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::types::{TelegraphResult, PageViews};
use crate::requests::Request;
use crate::error::TelegraphError;


/// Builder of `getViews`
#[derive(Default, Serialize)]
pub struct GetViews {
    #[serde(skip)]
    client: Arc<Client>,
    #[serde(skip)]
    method_name: Arc<String>,

    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hour: Option<i32>
}

impl Request for GetViews {
    type MethodBuilder = GetViews;
    type Response = PageViews;

    fn new(client: Arc<Client>, method_name: Arc<String>) -> Self::MethodBuilder {
        Self::MethodBuilder { 
            client, 
            method_name,
            ..Self::MethodBuilder::default()
        }
    }
    fn send(&self) -> Result<Self::Response, TelegraphError> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Self::MethodBuilder::catch_api_error(json)
    }
}


impl GetViews {
    /// Required. Path to the Telegraph page 
    /// (in the format Title-12-31, where 12 is the month 
    /// and 31 the day the article was first published).
    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = path.into();
        self
    }

    /// Required if month is passed. 
    /// If passed, the number of page 
    /// views for the requested year will be returned.
    pub fn year(&mut self, year: i32) -> &mut Self {
        self.year = year.into();
        self
    }

    /// Required if day is passed. 
    /// If passed, the number of page views 
    /// for the requested month will be returned.
    pub fn month(&mut self, month: i32) -> &mut Self {
        self.month = month.into();
        self
    }

    /// Required if hour is passed. 
    /// If passed, the number of page views 
    /// for the requested day will be returned.
    pub fn day(&mut self, day: i32) -> &mut Self {
        self.day = day.into();
        self
    }

    /// If passed, the number of page views for the requested hour will be returned.
    pub fn hour(&mut self, hour: i32) -> &mut Self {
        self.hour = hour.into();
        self
    }
}
