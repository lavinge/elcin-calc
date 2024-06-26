use crate::model::fertilizers::FertilizersListing;
use crate::ui::components::fertilizers::FertilizersListingItem;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::SearchIcon;
use crate::ui::components::utils::{
    Block, Button, Card, Divider, List, Pagination, TextField, Title,
};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersListingProps {
    fertilizers_listing: Signal<FertilizersListing>,
    on_search: EventHandler<String>,
    on_add: EventHandler<()>,
    on_open: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_paginate: EventHandler<usize>,
}

#[component]
pub fn FertilizersListing(props: FertilizersListingProps) -> Element {
    let fertilizers = use_memo(move || props.fertilizers_listing.read().list());

    rsx! {
        Card {
            Block {
                Title {
                    "Список удобрений",
                }
            }

            Divider {}

            Block {
                Row {
                    TextField {
                        value: props.fertilizers_listing.read().search_query(),
                        placeholder: "найти удобрение",
                        on_input: props.on_search,
                        icon_left: rsx! {
                            SearchIcon {}
                        },
                    }

                    Button {
                        style: "primary",
                        on_click: props.on_add,
                        "Добавить удобрение",
                    }
                }
            }

            Block {
                exclude_padding: "top",

                Column {
                    List {
                        limit: 10,
                        empty: fertilizers.read().len() == 0,
                        stub_text: "Сохраненные удобрения отсутствуют",

                        for fertilizer in fertilizers.read().clone() {
                            FertilizersListingItem {
                                key: "{fertilizer.id()}",
                                fertilizer,
                                on_open: props.on_open,
                                on_delete: props.on_delete,
                            }
                        }
                    }

                    Pagination {
                        page_index: props.fertilizers_listing.read().page_index(),
                        limit: props.fertilizers_listing.read().limit(),
                        total: props.fertilizers_listing.read().total(),
                        on_change: props.on_paginate,
                    }
                }
            }
        }
    }
}
