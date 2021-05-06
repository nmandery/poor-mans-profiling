use chrono::{DateTime, Utc};
use serde::Serialize;

pub struct Span<T>
where
    T: Serialize,
{
    t_start: DateTime<Utc>,
    message: Option<String>,
    data: T,
}

#[derive(Serialize)]
pub struct FinishedSpan<'a, T>
where
    T: Serialize,
{
    t_start: DateTime<Utc>,
    t_end: DateTime<Utc>,
    message: &'a Option<String>,
    data: &'a T,
}

impl<T> Span<T>
where
    T: Serialize,
{
    pub fn new_with_message(data: T, message: &str) -> Self {
        Self {
            t_start: Utc::now(),
            message: Some(message.to_string()),
            data,
        }
    }

    pub fn new(data: T) -> Self {
        Self {
            t_start: Utc::now(),
            message: None,
            data,
        }
    }

    fn finish(&self) -> FinishedSpan<T> {
        FinishedSpan {
            t_start: self.t_start,
            t_end: Utc::now(),
            message: &self.message,
            data: &self.data,
        }
    }
}

impl<T> Drop for Span<T>
where
    T: Serialize,
{
    fn drop(&mut self) {
        let _finished_span = self.finish();
    }
}

impl<'a, T> Drop for FinishedSpan<'a, T>
where
    T: Serialize,
{
    fn drop(&mut self) {
        println!("{}", serde_json::to_string(self).unwrap())
    }
}
