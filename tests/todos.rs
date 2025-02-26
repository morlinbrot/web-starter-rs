use web_starter_rs::prelude::*;

mod utils;
use utils::spawn_test_app;

// #[ignore]
#[tokio::test]
async fn should_get_todos() {
    let app = spawn_test_app().await.expect("Failed to spawn test app");

    let route = "/todos";

    dbg!(&app.address);
    let res = reqwest::get(format!("{}{}", app.address, route))
        .await
        .expect(&format!("Failed to execute GET request at {}", &route));
    dbg!(&res);
    assert_eq!(res.status(), 200);

    let json = res
        .json::<Vec<Todo>>()
        .await
        .expect("Failed to parse JSON response");
    dbg!(&json);
    assert!(json.is_empty());
}

// #[ignore]
#[tokio::test]
async fn should_create_update_and_delete_todos() {
    let app = spawn_test_app().await.expect("Failed to spawn test app");

    // ---- CREATE
    let route = "/todos";

    let text = "Test the POST endpoint".to_string();
    let todo_create = TodoCreate { text: text.clone() };

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}{}", app.address, route))
        .json(&todo_create)
        .send()
        .await
        .expect(&format!("Failed to execute POST request at {}", &route));

    dbg!(&res);
    assert_eq!(res.status(), 200);

    let created = res
        .json::<Todo>()
        .await
        .expect("Failed to parse JSON response");
    dbg!(&created);

    assert!(!created.id.is_nil());
    assert_eq!(created.text, text);
    assert_eq!(created.completed, false);

    // ---- UPDATE
    let route = format!("/todos/{}", created.id);

    let todo_update = TodoUpdate { completed: true };

    let client = reqwest::Client::new();
    let res = client
        .patch(format!("{}{}", app.address, route))
        .json(&todo_update)
        .send()
        .await
        .expect(&format!("Failed to execute PATCH request at {}", &route));

    dbg!(&res);

    let updated = res
        .json::<Todo>()
        .await
        .expect("Failed to parse JSON response");
    dbg!(&updated);

    assert_eq!(updated.completed, true);

    // ---- DELETE
    let route = format!("/todos/{}", created.id);

    let client = reqwest::Client::new();
    let res = client
        .delete(format!("{}{}", app.address, route))
        .send()
        .await
        .expect(&format!("Failed to execute DELETE request at {}", &route));

    dbg!(&res);
    assert_eq!(res.status(), 200);

    let deleted = res
        .json::<Todo>()
        .await
        .expect("Failed to parse JSON response");
    dbg!(&deleted);

    assert_eq!(deleted.id, created.id);
}
