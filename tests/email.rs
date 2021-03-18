mod tests {

    use std::env;

    async fn send_test_email() -> smtp2go::Smtp2goApiResult {
        smtp2go::Email::new()
            .from("Matt <matt@example.com>")
            .to(&["Dave <dave@example.com>".to_string()])
            .subject("Trying out SMTP2GO")
            .text_body("Test message")
            .send()
            .await
    }

    #[actix_rt::test]
    async fn test_missing_from_field() {
        // attempt a send, should return Smtp2goError::MissingRequiredField error
        match smtp2go::Email::new().send().await {
            Ok(_) => panic!("Send didn't fail on missing 'from' field"),
            Err(_) => (),
        }
    }

    #[actix_rt::test]
    async fn test_missing_to_field() {
        // attempt a send, should return Smtp2goError::MissingRequiredField error
        match smtp2go::Email::new()
            .from("Matt <matt@example.com>")
            .send()
            .await
        {
            Ok(_) => panic!("Send didn't fail on missing 'to' field"),
            Err(_) => (),
        }
    }

    #[actix_rt::test]
    async fn test_missing_subject_field() {

        // attempt a send, should return Smtp2goError::MissingRequiredField error
        match smtp2go::Email::new()
            .from("Matt <matt@example.com>")
            .to(&["Data <dave@example.com>".to_string()])
            .send().await  {
            Ok(_) => panic!("Send didn't fail on missing 'subject' field"),
            Err(_) => (),
        }
    }

    #[actix_rt::test]
    async fn test_missing_text_body_field() {

        // attempt a send, should return Smtp2goError::MissingRequiredField error
        match smtp2go::Email::new()
            .from("Matt <matt@example.com>")
            .to(&["Dave <dave@example.com>".to_string()])
            .subject("test subject")
            .send() .await {
            Ok(_) => panic!("Send didn't fail on missing 'text_body' field"),
            Err(_) => (),
        }
    }

    #[actix_rt::test]
    async fn test_missing_apikey() {

        // remove the api key env if set
        env::remove_var("SMTP2GO_API_KEY");

        // send a test email, check it returns failure
        match send_test_email().await {
            Ok(_) => panic!("Send didn't fail on missing ApiKey"),
            Err(_) => (),
        }
    }

    #[actix_rt::test]
    async fn test_invalid_apikey() {

        // remove the api key env if set
        env::set_var("SMTP2GO_API_KEY", "api-00000000000");

        // send a test email, check it returns failure
        match send_test_email().await {
            Ok(_) => panic!("Send didn't fail on incorrect ApiKey"),
            Err(_) => (),
        }
    }

    #[actix_rt::test]
    async fn test_send() {

        // remove the api key env if set
        env::set_var("SMTP2GO_API_KEY", "api-00000000000000000000000000000000");

        match send_test_email().await {
            Ok(_) => (),
            Err(error) => panic!("Failed to send test email: {:?}", error),
        };
    }
}
