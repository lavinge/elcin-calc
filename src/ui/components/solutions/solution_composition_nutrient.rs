use crate::model::chemistry::Nutrient;
use crate::model::solutions::NutrientResult;
use crate::ui::components::layout::Column;
use crate::ui::components::utils::{Badge, Text, Tooltip};
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

fn tooltip_text(nutrient_result: NutrientResult) -> String {
    let diff_percent = nutrient_result.diff_percent();

    let diff = nutrient_result.diff();

    if diff_percent == 0. {
        return String::from("Рассчитанное значение равно установленному");
    }

    if diff_percent < 5. {
        return format!(
            "Рассчитанное значение равно установленному. Разница ({:.2}PPM) находится в пределах погрешности.", diff
        );
    }

    if diff < 0. {
        return format!("До установленного значение не хватает {:.2}PPM", diff.abs());
    } else {
        return format!("Установленное значение превышено на {:.2}PPM", diff);
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SolutionCompositionNutrientProps {
    badge: bool,
    nutrient: Nutrient,
    nutrient_result: NutrientResult,
}

#[component]
pub fn SolutionCompositionNutrient(props: SolutionCompositionNutrientProps) -> Element {
    let diff_state = props.nutrient_result.diff_state();

    rsx! {
        div {
            class: "solution-composition-nutrient",

            if !props.badge {
                Badge {
                    size: "small",
                    state: diff_state.clone(),
                    text: "!"
                }
            }

            Tooltip {
                target: rsx! {
                    NutrientValue {
                        state: diff_state,
                        symbol: props.nutrient.symbol(),
                        value: props.nutrient.value(),
                    }
                },

                body: rsx! {
                    Column {
                        gap: "small",

                        Text {
                            size: "x-small",
                            nowrap: true,
                            {props.nutrient.name()},
                        }

                        if !props.badge {
                            Text {
                                size: "x-small",
                                {tooltip_text(props.nutrient_result)},
                            }
                        }
                    }
                },
            }
        }
    }
}
