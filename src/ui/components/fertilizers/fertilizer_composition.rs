use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;
use crate::ui::components::fertilizers::NutrientContentValue;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{Text, Title};
use crate::ui::components::ReferencePreview;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerCompositionProps {
    fertilizer: Memo<Fertilizer>,
}

#[component]
pub fn FertilizerComposition(props: FertilizerCompositionProps) -> Element {
    let fertilizer = &*props.fertilizer.read();

    rsx! {
        Column {
            gap: "medium",

            Row {
                Title {
                    size: "small",
                    "Питательные вещества",
                    ReferencePreview {
                        show_reference: Signal::new(false),
                        article_id: "fertilizer-editor-nutrients",
                    }
                }
            }

            Column {
                gap: "x-small",

                Text {
                    size: "x-small",
                    "Макроэлементы",
                }

                Row {
                    gap: "small",

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Nitrogen(0.0)],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Phosphorus(0.0)],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Potassium(0.0)],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Calcium(0.0)],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Magnesium(0.0)],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Sulfur(0.0)],
                    }
                }
            }

            Column {
                gap: "x-small",

                Text {
                    size: "x-small",
                    "Формы азота",
                }

                Row {
                    gap: "small",

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::NitrogenNitrate(0.0)],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::NitrogenAmmonium(0.0)],
                    }
                }
            }

            Column {
                gap: "x-small",

                Text {
                    size: "x-small",
                    "Микроэлементы",
                }

                Row {
                    gap: "small",

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Iron(0.0)],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Manganese(0.0)],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Copper(0.0)],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Zinc(0.0)],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Boron(0.0)],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer[Nutrient::Molybdenum(0.0)],
                    }
                }
            }
        }
    }
}
