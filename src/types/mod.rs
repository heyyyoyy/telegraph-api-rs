mod account;
mod page;


pub use self::account::Account;
pub use self::page::Page;


use serde::Deserialize;


#[derive(Deserialize)]
struct TelegraphResult<T> {
    ok: bool,
    result: Option<T>,
}
