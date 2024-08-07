use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::pages::fertilizers::{FertilizerAddPage, FertilizerEditPage, FertilizersMainPage};
use super::pages::profiles::{ProfileAddPage, ProfileEditPage, ProfilesMainPage};
use super::pages::reference::ReferenceMainPage;
use super::pages::solutions::{
    SolutionAddPage, SolutionEditPage, SolutionsMainPage, StockSolutionPage,
};
use super::Layout;

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
    #[redirect("", || Route::ReferenceMainPage {})]
    #[layout(Layout)]
    #[route("/reference")]
    ReferenceMainPage {},

    #[route("/solutions")]
    SolutionsMainPage {},

    #[route("/solutions/add?:profile_id")]
    SolutionAddPage { profile_id: String },

    #[route("/solutions/edit/:solution_id")]
    SolutionEditPage { solution_id: String },

    #[route("/solutions/stock?:solution_id")]
    StockSolutionPage { solution_id: String },

    #[route("/profiles")]
    ProfilesMainPage {},

    #[route("/profiles/add")]
    ProfileAddPage {},

    #[route("/profiles/edit/:profile_id")]
    ProfileEditPage { profile_id: String },

    #[route("/fertilizers")]
    FertilizersMainPage {},

    #[route("/fertilizers/add")]
    FertilizerAddPage {},

    #[route("/fertilizers/edit/:fertilizer_id")]
    FertilizerEditPage { fertilizer_id: String },
}
