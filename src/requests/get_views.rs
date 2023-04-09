use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::types::{TelegraphResult, PageViews};
use crate::requests::{NoPath, Path};
use crate::error::TelegraphError;


/// Builder of `getViews`
#[derive(Default, Serialize)]
pub struct GetViews<P> {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    path: P,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hour: Option<i32>
}


impl GetViews<NoPath> {
    pub(crate) fn new(client: Rc<Client>, method_name: Rc<String>) -> GetViews<NoPath> {
        Self {  client, method_name, ..Self::default() }
    }
}

impl<P> GetViews<P> {
    /// Required. Path to the Telegraph page 
    /// (in the format Title-12-31, where 12 is the month 
    /// and 31 the day the article was first published).
    pub fn path(self, path: &str) -> GetViews<Path> {
        GetViews { 
            client: self.client, 
            method_name: self.method_name, 
            path: Path(path.into()), 
            year: self.year, 
            month: self.month, 
            day: self.day, 
            hour: self.hour 
        }
    }

    /// Required if month is passed. 
    /// If passed, the number of page 
    /// views for the requested year will be returned.
    pub fn year(mut self, year: i32) -> Self {
        self.year = year.into();
        self
    }

    /// Required if day is passed. 
    /// If passed, the number of page views 
    /// for the requested month will be returned.
    pub fn month(mut self, month: i32) -> Self {
        self.month = month.into();
        self
    }

    /// Required if hour is passed. 
    /// If passed, the number of page views 
    /// for the requested day will be returned.
    pub fn day(mut self, day: i32) -> Self {
        self.day = day.into();
        self
    }

    /// If passed, the number of page views for the requested hour will be returned.
    pub fn hour(mut self, hour: i32) -> Self {
        self.hour = hour.into();
        self
    }
}

impl GetViews<Path> {
    /// Sending request to API
    pub fn send(self) -> Result<PageViews, TelegraphError> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<PageViews> = req.json()?;
        if !json.ok {
            Err(json.error.unwrap())
        } else {
            Ok(json.result.unwrap())
        }
    }
}
