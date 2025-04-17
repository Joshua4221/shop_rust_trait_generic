use crate::{
    authenticator::authentication,
    store::{store_model::StoreModel, store_schema::StoreSchema},
    users::buyer_model::BuyerModel,
};

pub fn controller() -> Result<(), String> {
    authentication::<StoreSchema<StoreModel>, StoreSchema<BuyerModel>>()?;

    Ok(())
}
