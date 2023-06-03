use axum::extract::Form;
use axum::extract::Path;
use axum::response::Html;
use axum::response::IntoResponse;
use handlebars::Handlebars;
use serde;

#[derive(serde::Deserialize)]
pub struct CreateGraphRequest {
    dot: String,
}

pub struct Root {}

impl Root {
    pub async fn get() -> impl IntoResponse {
        let mut hbs = Handlebars::new();
        hbs.register_template_file("layout", "src/templates/layout.hbs")
            .unwrap();
        hbs.register_template_file("main", "src/templates/index.hbs")
            .unwrap();

        Html(hbs.render("layout", &()).unwrap())
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
