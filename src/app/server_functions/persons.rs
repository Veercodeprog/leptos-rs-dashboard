use crate::app::models::person::{AddPersonRequest, Person};
use leptos::server_fn::error::ServerFnError;
use leptos::*;
use serde::*;

// ✅ Step 1: Declare these functions so they exist on both client & server
#[server(GetPersons, "/api")]
pub async fn get_persons() -> Result<Vec<Person>, ServerFnError> {
    retrieve_all_persons()
        .await
        .map_err(|e| ServerFnError::ServerError(e))
}

#[server(AddPerson, "/api")]
pub async fn add_person(add_person_request: AddPersonRequest) -> Result<Person, ServerFnError> {
    add_new_person(
        add_person_request.name,
        add_person_request.title,
        add_person_request.level,
        add_person_request.compensation,
    )
    .await
    .map_err(|e| ServerFnError::ServerError(e))
}

// ✅ Step 2: Define actual logic only for SSR (server-side)
cfg_if::cfg_if! {
    if #[cfg(feature="ssr")] {
        use crate::app::db::database;
        use chrono::Local;
        use uuid::Uuid;

        pub async fn retrieve_all_persons() -> Result<Vec<Person>, String> {
            let get_all_persons_result = database::get_all_persons().await;
            Ok(get_all_persons_result.unwrap_or_default())
        }

        pub async fn add_new_person<T>(
            name: T,
            title: T,
            level: T,
            compensation: i32,
        ) -> Result<Person, String>
        where
            T: Into<String>,
        {
            let mut buffer = Uuid::encode_buffer();
            let uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);
            let current_formatted = Local::now().to_string();

            let new_person = Person::new(
                uuid.to_string(),
                name.into(),
                title.into(),
                level.into(),
                compensation,
                current_formatted,
            );

            database::add_person(new_person)
                .await
                .ok_or_else(|| "Failed to add person".to_string())
        }
    } else {
        // ✅ Step 3: Provide no-op stubs for client build
        pub async fn retrieve_all_persons() -> Result<Vec<Person>, String> {
            Ok(Vec::new())
        }

        pub async fn add_new_person<T>(
            _name: T,
            _title: T,
            _level: T,
            _compensation: i32,
        ) -> Result<Person, String>
        where
            T: Into<String>,
        {
            Err("Not available on client".to_string())
        }
    }
}
