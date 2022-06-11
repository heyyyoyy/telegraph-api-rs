mod account;

pub use self::account::Account;

use serde::Deserialize;


#[derive(Deserialize)]
struct TelegraphResult<T> {
    ok: bool,
    result: Option<T>,
}
