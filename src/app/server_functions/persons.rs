use crate::app::models::person::{AddPersonRequest, Person};
use leptos::leptos_dom::logging::console_log;
use leptos::server_fn::error::ServerFnError;
use leptos::*;
use serde::*;

#[server(GetPersons, "/api")]
pub async fn get_persons() -> Result<Vec<Person>, ServerFnError> {
    let persons = retrieve_all_persons().await;
    Ok(persons)
}

#[server(AddPerson, "/api")]
pub async fn add_person(add_person_request: AddPersonRequest) -> Result<Person, ServerFnError> {
    // Add logging at the server function level
    console_log(&format!(
        "Server function add_person called with: {:?}",
        add_person_request
    ));

    add_new_person(
        add_person_request.name,
        add_person_request.title,
        add_person_request.level,
        add_person_request.compensation,
    )
    .await
    .map_err(|e| {
        console_log(&format!("Error from add_new_person: {}", e));
        ServerFnError::ServerError(e)
    })
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::app::db::database;
        use chrono::Local;
        use uuid::Uuid;

        pub async fn retrieve_all_persons() -> Vec<Person> {
            let get_all_persons_result = database::get_all_persons().await;
            match get_all_persons_result {
                Some(found_persons) => found_persons,
                None => Vec::new()
            }
        }

        // Return Result instead of Option
        pub async fn add_new_person<T>(
            name: T,
            title: T,
            level: T,
            compensation: i32
        ) -> Result<Person, String>
        where
            T: Into<String>
        {
            console_log("add_new_person called");

            let mut buffer = Uuid::encode_buffer();
            let uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            let current_now = Local::now();
            let current_formatted = current_now.to_string();

            let new_person = Person::new(
                String::from(uuid),
                name.into(),
                title.into(),
                level.into(),
                compensation,
                current_formatted
            );

            console_log(&format!("Created person object: {:?}", new_person));

            // Convert Option to Result
            let result = database::add_person(new_person.clone()).await;

            console_log(&format!("Database result: {:?}", result));

            result.ok_or_else(|| {
                let error = String::from("Database returned None - failed to add person");
                console_log(&format!("Error: {}", error));
                error
            })
        }
    } else {
        // Client-side stub
        pub async fn retrieve_all_persons() -> Vec<Person> {
            Vec::new()
        }

        pub async fn add_new_person<T>(
            _name: T,
            _title: T,
            _level: T,
            _compensation: i32
        ) -> Result<Person, String>
        where
            T: Into<String>
        {
            Err(String::from("Not available on client"))
        }
    }
}
