use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::types::{TelegraphResult, PageViews};


#[derive(Default, Serialize)]
struct GetViewsInner {
    path: String,
    year: i32,
    month: i32,
    day: i32,
    hour: i32
}


pub struct GetViews<'client> {
    client: &'client Client,
    method_name: &'static str,
    inner: GetViewsInner
}

impl<'client> GetViews<'client> {
    pub fn new(client: &'client Client, method_name: &'static str) -> Self {
        GetViews { client, method_name, inner: GetViewsInner::default() }
    }

    pub fn path(&mut self, path: &str) -> &mut Self {
        self.inner.path = path.into();
        self
    }

    pub fn year(&mut self, year: i32) -> &mut Self {
        self.inner.year = year;
        self
    }

    pub fn month(&mut self, month: i32) -> &mut Self {
        self.inner.month = month;
        self
    }

    pub fn day(&mut self, day: i32) -> &mut Self {
        self.inner.day = day;
        self
    }

    pub fn hour(&mut self, hour: i32) -> &mut Self {
        self.inner.hour = hour;
        self
    }

    // TODO: use trait for send request
    pub fn send(&self) -> Result<PageViews, Error> {
        let req = self.client.post(self.method_name).form(&self.inner).send()?;
        let json: TelegraphResult<PageViews> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}
