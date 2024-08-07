use super::{FertilizersBrowser, FertilizersSet, SolutionProfile};
use crate::controller::reference::TopicId;
use crate::controller::Validation;
use crate::repository::{FertilizersListing, ProfilesListing};
use crate::ui::components::layout::Row;
use crate::ui::components::reference::ReferenceBadge;
use crate::ui::components::utils::{Block, Button, Card, Divider, TextField, Title};
use dioxus::prelude::*;
use nutca::chemistry::{NutrientAmount, Volume};
use nutca::profiles::Profile;
use nutca::solutions::Solution;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionEditorProps {
    solution: Memo<Solution>,
    validation: Memo<Validation>,
    profile: Memo<Profile>,
    profiles_listing: Signal<ProfilesListing>,
    fertilizers_listing: Signal<FertilizersListing>,
    on_name_update: EventHandler<String>,
    on_volume_update: EventHandler<Volume>,
    on_profile_change: EventHandler<String>,
    on_profile_search: EventHandler<String>,
    on_profile_nutrient_update: EventHandler<NutrientAmount>,
    on_fertilizer_select: EventHandler<String>,
    on_fertilizer_exclude: EventHandler<String>,
    on_fertilizer_search: EventHandler<String>,
    on_fertilizers_paginate: EventHandler<usize>,
    on_save: EventHandler<()>,
}

#[component]
pub fn SolutionEditor(props: SolutionEditorProps) -> Element {
    rsx! {
        Card {
            Block {
                Row {
                    Title {
                        {TopicId::NutrientSolutionEditor.title()},
                        ReferenceBadge {
                            topic_id: TopicId::NutrientSolutionEditor,
                        },
                    }
                }
            }

            Divider {}

            Block {
                TextField {
                    label: "Название",
                    value: props.solution.read().name(),
                    error: props.validation.read().get("solution-name"),
                    on_input: props.on_name_update,
                }
            }

            Divider {}

            SolutionProfile {
                solution: props.solution,
                profile: props.profile,
                profiles_listing: props.profiles_listing,
                on_profile_change: props.on_profile_change,
                on_profile_search: props.on_profile_search,
                on_profile_nutrient_update: props.on_profile_nutrient_update,
            }

            Divider {}

            Block {
                Row {
                    FertilizersBrowser {
                        fertilizers_listing: props.fertilizers_listing,
                        on_select: props.on_fertilizer_select,
                        on_search: props.on_fertilizer_search,
                        on_paginate: props.on_fertilizers_paginate,
                    }

                    FertilizersSet {
                        solution: props.solution,
                        on_exclude: props.on_fertilizer_exclude,
                        on_volume_update: props.on_volume_update,
                    }
                }
            }

            Divider {}

            Block {
                Row {
                    horizontal: "end",

                    Button {
                        style: "primary",
                        on_click: props.on_save,
                        "Сохранить",
                    }
                }
            }
        }
    }
}
