use axum::extract::Form;
use axum::extract::Path;
use serde;

#[derive(serde::Deserialize)]
pub struct CreateGraphRequest {
    dot: String,
}

pub struct Root {}

impl Root {
    pub async fn get() {
        println!("root page");

        // Show dot editor
        todo!()
    }

    pub async fn post(form: Form<CreateGraphRequest>) {
        // save dot to the database
        println!("dot: {}", form.dot);

        // redirect to the graph detail page
        todo!()
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateGraphRequest {
    dot: String,
}

pub struct GraphDetail {}

impl GraphDetail {
    pub async fn get(Path(id): Path<String>) {
        // get dot from the database
        println!("id: {}", id);

        // render the graph detail page
        todo!()
    }

    pub async fn patch(Path(id): Path<String>, form: Form<UpdateGraphRequest>) {
        // update dot in the database
        println!("id: {}, dot: {}", id, form.dot);

        // redirect to the graph detail page
        todo!()
    }

    pub async fn delete(Path(id): Path<String>) {
        // delete dot from the database
        println!("id: {}", id);

        // redirect to the root page
        todo!()
    }
}
