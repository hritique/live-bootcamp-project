use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn signup_returns_422_for_invalid_body() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = vec![
        serde_json::json!({
            "email": random_email,
            "password": "password",
        }),
        serde_json::json!({
            "password": "password",
            "require2FA": false
        }),
        serde_json::json!({
            "email": random_email,
            "require2FA": true
        }),
    ];

    for test_case in test_cases {
        let response = app.post_signup(&test_case).await;

        assert_eq!(response.status().as_u16(), 422);
    }
}
