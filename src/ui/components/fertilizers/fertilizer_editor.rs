use crate::model::fertilizers::{Fertilizer, SourceType};
use crate::model::formulas::Formula;
use crate::model::labels::{Component, Label, Units};
use crate::ui::components::fertilizers::FertilizersComposition;
use crate::ui::components::utils::{Block, Button, Card, Divider, TextField, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerEditorProps {
    fertilizer: Memo<Fertilizer>,
    source_type: Memo<SourceType>,
    label: Memo<Label>,
    formula: Memo<Formula>,
    on_name_update: EventHandler<String>,
    on_vendor_update: EventHandler<String>,
    on_source_type_update: EventHandler<SourceType>,
    on_label_units_update: EventHandler<Units>,
    on_label_component_update: EventHandler<Component>,
    on_formula_update: EventHandler<String>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn FertilizerEditor(props: FertilizerEditorProps) -> Element {
    rsx! {
        Card {
            Block {
                Title {
                    text: "Редактор удобрения",
                }
            }

            Divider {}

            Block {
                div {
                    class: "fertilizer-editor__details",

                    TextField {
                        value: props.fertilizer.read().name(),
                        label: "Название",
                        on_input: props.on_name_update,
                    }

                    TextField {
                        value: props.fertilizer.read().vendor(),
                        label: "Производитель",
                        on_input: props.on_vendor_update,
                    }
                }
            }

            Divider {}

            Block {
                FertilizersComposition {
                    source_type: props.source_type,
                    label: props.label,
                    formula: props.formula,
                    on_source_type_update: props.on_source_type_update,
                    on_label_units_update: props.on_label_units_update,
                    on_label_component_update: props.on_label_component_update,
                    on_formula_update: props.on_formula_update,
                }
            }

            Divider {}

            Block {
                div {
                    class: "fertilizer-editor__controls",

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