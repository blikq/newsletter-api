#[cfg(test)]
mod testd{
    use std::vec;

    use actix_web::*;
    use super::*;
    use reqwest;
    use dotenv::dotenv;

    #[actix_web::test]
    async fn suscribe_returns_200_success() {
        let host = dotenv::var("HOST").unwrap();
        let port = dotenv::var("PORT").unwrap();
        let client = reqwest::Client::new();

        let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
        let response = client
                .post(format!("{}:{}/suscribe", host, port))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body)
                .send() 
                .await
                .expect("Failed to execute request.");

        assert_eq!(200, response.status().as_u16());
    }


    #[actix_web::test]
    async fn suscribe_returns_400_when_missing_info() {
        let host = dotenv::var("HOST").unwrap();
        let port = dotenv::var("PORT").unwrap();
        let client = reqwest::Client::new();

        let test_case = vec![
            ("name=le%20guin", "missing the email"),
            ("email=ursula_le_guin%40gmail.com", "missing the name"),
            ("", "missing both name and email")
        ];

        for (body, error_msg) in test_case{
            let response = client
                    .post(format!("{}:{}/suscribe", host, port))
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(body)
                    .send()
                    .await
                    .expect("failed to execute request");
            
            assert_eq!(
                400,
                response.status().as_u16(),
                // Additional customised error message on test failure
                "The API did not fail with 400 Bad Request when the payload was {}.",
                error_msg
                );
                    
        }
    }
}
