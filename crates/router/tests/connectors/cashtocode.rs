use common_utils::id_type;
use hyperswitch_domain_models::address::{Address, AddressDetails};
use router::types::{self, domain, storage::enums};

use crate::{
    connector_auth,
    utils::{self, ConnectorActions},
};

#[derive(Clone, Copy)]
struct CashtocodeTest;
impl ConnectorActions for CashtocodeTest {}
impl utils::Connector for CashtocodeTest {
    fn get_data(&self) -> types::api::ConnectorData {
        use router::connector::Cashtocode;
        utils::construct_connector_data_old(
            Box::new(Cashtocode::new()),
            types::Connector::Cashtocode,
            types::api::GetToken::Connector,
            None,
        )
    }

    fn get_auth_token(&self) -> types::ConnectorAuthType {
        utils::to_connector_auth_type(
            connector_auth::ConnectorAuthentication::new()
                .cashtocode
                .expect("Missing connector authentication configuration")
                .into(),
        )
    }

    fn get_name(&self) -> String {
        "cashtocode".to_string()
    }
}

static CONNECTOR: CashtocodeTest = CashtocodeTest {};

impl CashtocodeTest {
    fn get_payment_authorize_data(
        payment_method_type: Option<enums::PaymentMethodType>,
        payment_method_data: domain::PaymentMethodData,
    ) -> Option<types::PaymentsAuthorizeData> {
        let cust_id = id_type::CustomerId::try_from(std::borrow::Cow::from("John Doe"));
        Some(types::PaymentsAuthorizeData {
            amount: 1000,
            currency: enums::Currency::EUR,
            payment_method_data,
            confirm: true,
            statement_descriptor_suffix: None,
            statement_descriptor: None,
            setup_future_usage: None,
            mandate_id: None,
            off_session: None,
            setup_mandate_details: None,
            capture_method: None,
            browser_info: None,
            order_details: None,
            order_category: None,
            email: None,
            customer_name: None,
            payment_experience: None,
            payment_method_type,
            session_token: None,
            enrolled_for_3ds: false,
            related_transaction_id: None,
            router_return_url: Some(String::from("https://google.com")),
            webhook_url: None,
            complete_authorize_url: None,
            customer_id: cust_id.ok(),
            surcharge_details: None,
            request_incremental_authorization: false,
            metadata: None,
            authentication_data: None,
            customer_acceptance: None,
            ..utils::PaymentAuthorizeType::default().0
        })
    }

    fn get_payment_info() -> Option<utils::PaymentInfo> {
        Some(utils::PaymentInfo {
            address: Some(types::PaymentAddress::new(
                None,
                Some(Address {
                    address: Some(AddressDetails {
                        country: Some(api_models::enums::CountryAlpha2::US),
                        ..Default::default()
                    }),
                    phone: None,
                    email: None,
                }),
                None,
                None,
            )),
            ..Default::default()
        })
    }
}

//fetch payurl for payment create
#[actix_web::test]
async fn should_fetch_pay_url_classic() {
    let authorize_response = CONNECTOR
        .make_payment(
            CashtocodeTest::get_payment_authorize_data(
                Some(enums::PaymentMethodType::ClassicReward),
                domain::payments::PaymentMethodData::Reward,
            ),
            CashtocodeTest::get_payment_info(),
        )
        .await
        .unwrap();
    assert_eq!(
        authorize_response.status,
        enums::AttemptStatus::AuthenticationPending
    );
}

#[actix_web::test]
async fn should_fetch_pay_url_evoucher() {
    let authorize_response = CONNECTOR
        .make_payment(
            CashtocodeTest::get_payment_authorize_data(
                Some(enums::PaymentMethodType::Evoucher),
                domain::payments::PaymentMethodData::Reward,
            ),
            CashtocodeTest::get_payment_info(),
        )
        .await
        .unwrap();
    assert_eq!(
        authorize_response.status,
        enums::AttemptStatus::AuthenticationPending
    );
}
