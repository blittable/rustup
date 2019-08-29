use crate::exceptions::ServiceException;
use crate::models::Buyer;
use crate::services::customer::BuyerService;

pub trait BuyerServiceTrait {
    fn new() -> BuyerService;
    fn load(&self, buyer_name: &'static str, credit: f32) -> Result<Buyer, ServiceException>;
}
