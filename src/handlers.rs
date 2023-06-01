use axum::extract::Form;
use axum::extract::Path;
use serde;

#[derive(serde::Deserialize)]
pub struct CreateGraphRequest {
    dot: String,
}

pub struct Root {}

impl Root {
    pub async fn get() -> String {
        // Show dot editor
        "Hello, World!".to_string()
    }

    pub async fn post(form: Form<CreateGraphRequest>) -> String {
        // save dot to the database
        // redirect to the graph detail page
        format!("dot: {}", form.dot)
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateGraphRequest {
    dot: String,
}

pub struct GraphDetail {}

impl GraphDetail {
    pub async fn get(Path(id): Path<String>) -> String {
        // get dot from the database
        // render the graph detail page
        format!("id: {}", id)
    }

    pub async fn patch(Path(id): Path<String>, form: Form<UpdateGraphRequest>) -> String {
        // update dot in the database
        // redirect to the graph detail page
        format!("id: {}, dot: {}", id, form.dot)
    }

    pub async fn delete(Path(id): Path<String>) -> String {
        // delete dot from the database
        // redirect to the root page
        format!("id: {}", id)
    }
}
