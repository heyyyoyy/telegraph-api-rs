use std::sync::Arc;

use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::{types::{TelegraphResult, PageViews}, Request};


#[derive(Serialize)]
pub struct GetViews {
    #[serde(skip)]
    client: Arc<Client>,
    #[serde(skip)]
    method_name: Arc<String>,

    path: String,
    year: i32,
    month: i32,
    day: i32,
    hour: i32
}

impl Request for GetViews {
    type MethodBuilder = GetViews;
    type Response = PageViews;

    fn new(client: Arc<Client>, method_name: Arc<String>) -> Self::MethodBuilder {
        Self::MethodBuilder { 
            client, 
            method_name,
            path: "".to_string(),
            year: 2000,
            month: 1,
            day: 1,
            hour: 0 
        }
    }
    fn send(&self) -> Result<Self::Response, Error> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}


impl GetViews {
    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = path.into();
        self
    }

    pub fn year(&mut self, year: i32) -> &mut Self {
        self.year = year;
        self
    }

    pub fn month(&mut self, month: i32) -> &mut Self {
        self.month = month;
        self
    }

    pub fn day(&mut self, day: i32) -> &mut Self {
        self.day = day;
        self
    }

    pub fn hour(&mut self, hour: i32) -> &mut Self {
        self.hour = hour;
        self
    }
}
