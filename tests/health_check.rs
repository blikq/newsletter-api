#[cfg(test)]
mod tests {
    use actix_web::*;
    use reqwest;
    use dotenv::dotenv;


    use super::*;

    #[actix_web::test]
    async fn health_check_works() {
        // Arrange
        // spawn_app().await.expect("Failed to spawn our app.");
        // We need to bring in `reqwest`
        // to perform HTTP requests against our
        let client = reqwest::Client::new();
        // Act
        let host = dotenv::var("HOST").unwrap();
        let port = dotenv::var("PORT").unwrap();
        let response = client
            .get(format!("{}:{}/health_check", host, port))
            .send()
            .await
            .expect("Failed to execute request.");
            // Assert
            assert!(response.status().is_success());
            assert_eq!(Some(0), response.content_length());
    }
        
    
}