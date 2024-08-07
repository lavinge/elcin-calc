use crate::controller::reference::TopicId;
use crate::controller::Validation;
use crate::ui::components::layout::Row;
use crate::ui::components::profiles::ProfileNutrients;
use crate::ui::components::reference::ReferenceBadge;
use crate::ui::components::utils::{Block, Button, Card, Divider, TextField, Title};
use dioxus::prelude::*;
use nutca::chemistry::NutrientAmount;
use nutca::profiles::Profile;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileEditorProps {
    profile: Memo<Profile>,
    validation: Memo<Validation>,
    on_name_update: EventHandler<String>,
    on_nutrient_update: EventHandler<NutrientAmount>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn ProfileEditor(props: ProfileEditorProps) -> Element {
    rsx! {
        Card {
            Block {
                Row {
                    Title {
                        {TopicId::ProfileEditor.title()},
                        ReferenceBadge {
                            topic_id: TopicId::ProfileEditor,
                        },
                    }
                }
            }

            Divider {}

            Block {
                TextField {
                    label: "Название",
                    value: props.profile.read().name(),
                    error: props.validation.read().get("profile-name"),
                    on_input: props.on_name_update,
                }
            }

            Divider {}

            Block {
                ProfileNutrients {
                    profile: props.profile,
                    on_nutrient_update: props.on_nutrient_update,
                }
            }

            Divider {}

            Block {
                Row {
                    horizontal: "end",

                    Button {
                        style: "stroke",
                        on_click: props.on_cancel,
                        "Сбросить",
                    }

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
