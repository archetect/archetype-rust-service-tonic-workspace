use customer_service_core::proto::customer_service_server::CustomerService;
use customer_service_core::proto::{
    CreateCustomerRequest, CreateCustomerResponse, GetCustomerListRequest, GetCustomerListResponse,
};
use customer_service_core::CustomerServiceCore;
use customer_service_persistence::CustomerServicePersistence;
use tonic::Request;

#[tokio::test]
async fn test_create_customer() -> Result<(), Box<dyn std::error::Error>> {
    let core = core().await?;

    let result = core
        .get_customer_list(Request::new(GetCustomerListRequest { page_size: 0, page: 0 }))
        .await?;
    let GetCustomerListResponse { record, total_pages } = result.into_inner();
    assert_eq!(record.len(), 0);
    assert_eq!(total_pages, 0);

    let response = core
        .create_customer(Request::new(CreateCustomerRequest {
            contents: "Contents".to_string(),
        }))
        .await?;
    let CreateCustomerResponse { record } = response.into_inner();
    let record = record.expect("Record Expected");
    assert_eq!(&record.contents, "Contents");

    let response = core
        .get_customer_list(Request::new(GetCustomerListRequest { page_size: 0, page: 0 }))
        .await?;
    let GetCustomerListResponse { record, total_pages } = response.into_inner();
    assert_eq!(record.len(), 1);
    assert_eq!(total_pages, 1);

    Ok(())
}

async fn core() -> Result<CustomerServiceCore, Box<dyn std::error::Error>> {
    let persistence = CustomerServicePersistence::new().await?;
    let core = CustomerServiceCore::new(persistence).await?;
    Ok(core)
}
