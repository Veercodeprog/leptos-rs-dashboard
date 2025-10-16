use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: i64,
    pub documentId: String,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
    pub publishedAt: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub pageSize: u32,
    pub pageCount: u32,
    pub total: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Meta {
    pub pagination: Pagination,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StrapiList<T> {
    pub data: Vec<T>,
    pub meta: Meta,
}

pub async fn fetch_categories_page(
    page: u32,
    page_size: u32,
) -> Result<StrapiList<Category>, String> {
    let url =
        format!("http://localhost:1337/api/articles?filters[category][slug][$eq]=tech&populate=*");
    let resp = Request::get(&url)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("network error: {e}"))?;
    if !resp.ok() {
        return Err(format!("http {} {}", resp.status(), resp.status_text()));
    }
    resp.json::<StrapiList<Category>>()
        .await
        .map_err(|e| format!("parse error: {e}"))
}

pub async fn fetch_all_categories() -> Result<Vec<Category>, String> {
    let first = fetch_categories_page(1, 100).await?;
    let mut all = first.data;
    let page_count = first.meta.pagination.pageCount;

    for page in 2..=page_count {
        let next = fetch_categories_page(page, 100).await?;
        all.extend(next.data);
    }
    Ok(all)
}

use leptos::prelude::*;
use leptos::*;
// Basic component to display all categories
#[component]
pub fn CategoriesList() -> impl IntoView {
    let categories = LocalResource::new(|| async { fetch_all_categories().await });

    view! {
        <section>
            <h2>"Categories"</h2>
            <Suspense fallback=|| {
                view! { <p>"Loading…"</p> }
            }>
                {move || match categories.get() {
                    Some(Ok(items)) => {
                        view! {
                            <ul>
                                {items
                                    .into_iter()
                                    .map(|c| {
                                        let name = c
                                            .name
                                            .clone()
                                            .unwrap_or_else(|| "(untitled)".to_string());
                                        let slug = c.slug.clone().unwrap_or_default();
                                        view! {
                                            <li>
                                                <strong>{name}</strong>
                                                {if !slug.is_empty() {
                                                    view! { <span>{" — "}{slug}</span> }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => view! { <p class="text-red-600">{"Error: "}{e}</p> }.into_any(),
                    None => view! { <p>"…"</p> }.into_any(),
                }}

            </Suspense>
        </section>
    }
}
