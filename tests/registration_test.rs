mod common;

use dotenv::dotenv;
use std::path::Path;
use thirtyfour::prelude::*;

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn registration() -> WebDriverResult<()> {
    dotenv().ok();

    let driver = common::Config::new().get_driver().await?;

    driver.get("http://localhost:9095").await?;

    // Click the search button.
    let elem_button = driver
        .find_element(By::Css("button[type='submit']"))
        .await?;
    elem_button.click().await?;

    assert!(driver
        .page_source()
        .await?
        .contains("Invalid email or password"));

    let email = common::random_email();

    // Let's go and register
    driver
        .find_element(By::LinkText("SIGN UP"))
        .await?
        .click()
        .await?;
    driver
        .find_element(By::Id("email"))
        .await?
        .send_keys(&email)
        .await?;
    driver
        .find_element(By::Id("password"))
        .await?
        .send_keys(&email)
        .await?;
    driver
        .find_element(By::Id("confirm_password"))
        .await?
        .send_keys(&email)
        .await?;
    driver
        .find_element(By::Css("button[type='submit']"))
        .await?
        .click()
        .await?;

    // Doesn't work in CI CD
    //assert!(
    //    driver
    //        .page_source()
    //        .await?
    //        .contains("User-Agent")
    //);

    let cookie = driver.get_cookie("session").await;

    assert!(cookie.is_ok());

    driver.get("http://localhost:9095/auth/sign_out").await?;

    driver
        .screenshot(Path::new("./target/registration.png"))
        .await?;

    let cookie = driver.get_cookie("session").await;

    assert!(cookie.is_err());

    Ok(())
}
