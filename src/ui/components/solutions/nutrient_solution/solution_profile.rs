use super::SolutionComposition;
use crate::repository::ProfilesListing;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::profiles::ProfileForm;
use crate::ui::components::utils::{
    Badge, Block, ButtonsGroup, ButtonsGroupButton, Select, Text, Title,
};
use dioxus::prelude::*;
use nutca::chemistry::NutrientAmount;
use nutca::profiles::Profile;
use nutca::solutions::Solution;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

#[derive(Props, PartialEq, Clone)]
pub struct SolutionProfileProps {
    solution: Memo<Solution>,
    profile: Memo<Profile>,
    profiles_listing: Signal<ProfilesListing>,
    on_profile_change: EventHandler<String>,
    on_profile_search: EventHandler<String>,
    on_profile_nutrient_update: EventHandler<NutrientAmount>,
}

#[component]
pub fn SolutionProfile(props: SolutionProfileProps) -> Element {
    let mut profile_tab = use_signal(|| String::from("profile"));

    let profile_select_value =
        use_memo(move || (props.profile.read().id(), props.profile.read().name()));

    rsx! {
        Block {
            Column {
                gap: "medium",

                Row {
                    horizontal: "space-between",
                    vertical: "center",

                    Row {
                        Title {
                            size: "small",
                            "Профиль питания",
                        }
                    }

                    Row {
                        horizontal: "end",
                        vertical: "center",

                        Text {
                            size: "x-small",
                            "~EC {round(props.solution.read().ec())}",
                        }

                        ButtonsGroup {
                            value: profile_tab.read().clone(),
                            buttons: vec![
                                ButtonsGroupButton {
                                    label: String::from("Желаемый"),
                                    value: String::from("profile"),
                                    badge: None
                                },
                                ButtonsGroupButton {
                                    label: String::from("Рассчитанный"),
                                    value: String::from("solution-composition"),
                                    badge: rsx! {
                                        if !props.solution.read().is_empty() {
                                            Badge {
                                                size: "small",
                                                text: "!",
                                                state: "error",
                                            }
                                        }
                                    }
                                },
                            ],
                            on_change: move |value| profile_tab.set(value),
                        }
                    }
                }

                Select {
                    placeholder: "выбрать готовый профиль",
                    value: profile_select_value,
                    options: props.profiles_listing.read()
                        .list()
                        .iter()
                        .map(|profile| (profile.id(), profile.name()))
                        .collect(),
                    on_search: move |search_query| {
                        props.on_profile_search.call(search_query);
                    },
                    on_change: move |profile_id| {
                        props.on_profile_change.call(profile_id);
                    },
                }

                match profile_tab.read().as_str() {
                    "profile" => rsx! {
                        ProfileForm {
                            profile: props.profile,
                            on_nutrient_update: props.on_profile_nutrient_update,
                        },
                    },

                    "solution-composition" => rsx! {
                        SolutionComposition {
                            solution: props.solution,
                        },
                    },

                    _ => None,
                }
            }
        }
    }
}
