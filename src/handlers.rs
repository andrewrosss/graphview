use std::io::Write;
use std::process;

use axum::extract::Form;
use axum::extract::Path;
use axum::response::Html;
use axum::response::IntoResponse;
use handlebars::Handlebars;
use serde;
use serde_json::json;

use crate::templates;

#[derive(serde::Deserialize)]
pub struct CreateGraphRequest {
    dot: String,
}

pub struct Root {}

impl Root {
    pub async fn get() -> impl IntoResponse {
        let mut hbs = Handlebars::new();
        hbs.register_template_string("layout", templates::LAYOUT)
            .unwrap();
        hbs.register_template_string("main", templates::INDEX)
            .unwrap();

        Html(hbs.render("layout", &()).unwrap())
    }

    pub async fn post(form: Form<CreateGraphRequest>) -> impl IntoResponse {
        let mut hbs = Handlebars::new();
        hbs.set_prevent_indent(true);
        hbs.register_template_string("layout", templates::LAYOUT)
            .unwrap();
        hbs.register_template_string("main", templates::INDEX)
            .unwrap();

        let graph_svg = dot_to_svg(&form.dot);
        let graph_svg = graph_svg.replace("<svg ", "<svg class=\"graph\" ");

        Html(
            hbs.render("layout", &json!({"dot": form.dot, "graph_svg": graph_svg}))
                .unwrap(),
        )
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

fn dot_to_svg(dot: &str) -> String {
    let mut child = process::Command::new("dot")
        .arg("-Tsvg")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Failed to execute dot");

    {
        let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");
        child_stdin
            .write_all(dot.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    String::from_utf8(output.stdout).expect("Output was not valid UTF-8")
}
