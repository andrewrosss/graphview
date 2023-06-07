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
// json!({"error": Some("It looks like something went wrong parsing the graph description")})
impl Root {
    pub async fn get() -> impl IntoResponse {
        let mut hbs = Handlebars::new();
        hbs.register_template_string("index", templates::INDEX)
            .unwrap();

        // Html(hbs.render("index", &()).unwrap())
        Html(hbs.render("index", &()).unwrap())
    }

    pub async fn post(form: Form<CreateGraphRequest>) -> impl IntoResponse {
        let mut hbs = Handlebars::new();
        hbs.set_prevent_indent(true);
        hbs.register_template_string("index", templates::INDEX)
            .unwrap();

        match dot_to_svg(&form.dot) {
            Ok(graph_svg) => {
                let graph_svg = graph_svg.replace("<svg ", "<svg id=\"graph\" ");
                Html(
                    hbs.render("index", &json!({"dot": form.dot, "graph_svg": graph_svg}))
                        .unwrap(),
                )
            }
            Err(DotError { code, message }) => Html(
                hbs.render(
                    "index",
                    &json!({"dot": form.dot, "error": {"code": code, "message": message}}),
                )
                .unwrap(),
            ),
        }
    }
}

struct DotError {
    code: Option<i32>,
    message: Option<String>,
}

fn dot_to_svg(dot: &str) -> anyhow::Result<String, DotError> {
    let mut child = process::Command::new("dot")
        .arg("-Tsvg")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
        .expect("Failed to execute dot");

    {
        let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");
        child_stdin
            .write_all(dot.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");

    if !output.status.success() {
        return Err(DotError {
            code: output.status.code(),
            message: String::from_utf8_lossy(&output.stderr)
                .trim()
                .to_string()
                .into(),
        });
        // return Err(io::Error::new(
        //     io::ErrorKind::Other,
        //     format!(
        //         "Exit code: {}\nMessage: {}\nPrevious Input:\n\n{}",
        //         output.status.code().unwrap_or(-1),
        //         String::from_utf8_lossy(&output.stderr).trim(),
        //         dot
        //     ),
        // ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
