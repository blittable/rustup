pub mod traits;
use crate::exceptions::ServiceException;
use crate::models::Buyer;
use crate::services::customer::traits::BuyerServiceTrait;
use uuid::Uuid;

// -- BuyerService
#[derive(Debug)]
pub struct BuyerService {}

impl BuyerServiceTrait for BuyerService {
    fn new() -> BuyerService {
        BuyerService {}
    }

    fn load(
        &self,
        buyer_name: &'static str,
        credit_amount: f32,
    ) -> Result<Buyer, ServiceException> {
        if buyer_name == "" {
            return Err(ServiceException {
                message: "buyer name is empty!".to_string(),
            });
        }

        // Create new buyer
        Ok(Buyer {
            id: Uuid::new_v4(),
            name: buyer_name.to_string(),
            credit: credit_amount,
        })
    }
}
