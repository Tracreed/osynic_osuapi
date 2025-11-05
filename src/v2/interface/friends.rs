use crate::error::Result;
use crate::v2::model::friend::{Friend, FriendXApiVersion};

pub trait IFriends {
    fn get_friends(&self) -> impl std::future::Future<Output = Result<Vec<Friend>>>;
    fn get_friends_x_api_version(
        &self,
        x_api_version: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<FriendXApiVersion>>>;
}
