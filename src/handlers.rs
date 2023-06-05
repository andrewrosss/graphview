use std::io::Write;
use std::process;

use axum::extract::Form;
use axum::response::Html;
use axum::response::IntoResponse;
use handlebars::Handlebars;
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
        hbs.register_template_string("index", templates::INDEX)
            .unwrap();

        Html(hbs.render("index", &()).unwrap())
    }

    pub async fn post(form: Form<CreateGraphRequest>) -> impl IntoResponse {
        let mut hbs = Handlebars::new();
        hbs.set_prevent_indent(true);
        hbs.register_template_string("index", templates::INDEX)
            .unwrap();

        let graph_svg = dot_to_svg(&form.dot);
        let graph_svg = graph_svg.replace("<svg ", "<svg id=\"graph\" ");

        Html(
            hbs.render("index", &json!({"dot": form.dot, "graph_svg": graph_svg}))
                .unwrap(),
        )
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
