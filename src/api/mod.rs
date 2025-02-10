use crate::services::database::*;
use leptos::prelude::*;

#[server]
pub async fn get_cars() -> Result<Vec<Car>, ServerFnError> {
    Ok(vec![])
}
