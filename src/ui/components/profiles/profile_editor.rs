use crate::model::chemistry::Nutrient;
use crate::model::profiles::Profile;
use crate::ui::components::layout::Row;
use crate::ui::components::profiles::ProfileNutrients;
use crate::ui::components::utils::{Block, Button, Card, Divider, TextField, Title};
use crate::ui::components::ReferenceSubject;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileEditorProps {
    profile: Memo<Profile>,
    on_name_update: EventHandler<String>,
    on_nutrient_update: EventHandler<Nutrient>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn ProfileEditor(props: ProfileEditorProps) -> Element {
    rsx! {
        Card {
            Block {
                Title {
                    "Редактор профиля питания",
                }
            }

            Divider {}

            Block {
                TextField {
                    value: props.profile.read().name(),
                    label: "Название",
                    on_input: props.on_name_update,
                }
            }

            Divider {}

            ReferenceSubject {
                Block {
                    ProfileNutrients {
                        profile: props.profile,
                        on_nutrient_update: props.on_nutrient_update,
                    }
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
