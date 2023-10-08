use lite_async_test::async_test;

async fn some_async_fn() -> bool {
    true
}

async fn some_other_async_fn() -> bool {
    false
}


#[async_test]
async fn test_async() {
    assert!(some_async_fn().await);
    assert_eq!(false, some_other_async_fn().await);
}
