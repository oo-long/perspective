////////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) 2018, the Perspective Authors.
//
// This file is part of the Perspective library, distributed under the terms
// of the Apache License 2.0.  The full license can be found in the LICENSE
// file.

use super::modal::*;
use crate::utils::WeakScope;
use crate::*;

use web_sys::*;
use yew::prelude::*;

static CSS: &str = include_str!("../../../build/css/filter-dropdown.css");

pub enum FilterDropDownMsg {
    SetValues(Vec<String>),
    SetCallback(Callback<String>),
    ItemDown,
    ItemUp,
    ItemSelect,
}

pub struct FilterDropDown {
    values: Option<Vec<String>>,
    selected: usize,
    on_select: Option<Callback<String>>,
    // link: Scope<Self>,
}

#[derive(Properties, PartialEq)]
pub struct FilterDropDownProps {
    #[prop_or_default]
    pub weak_link: WeakScope<FilterDropDown>,
}

impl ModalLink<FilterDropDown> for FilterDropDownProps {
    fn weak_link(&self) -> &'_ WeakScope<FilterDropDown> {
        &self.weak_link
    }
}

impl Component for FilterDropDown {
    type Message = FilterDropDownMsg;
    type Properties = FilterDropDownProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.set_modal_link();
        FilterDropDown {
            values: Some(vec![]),
            selected: 0,
            on_select: None,
            // link,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FilterDropDownMsg::SetCallback(callback) => {
                self.on_select = Some(callback);
                false
            }
            FilterDropDownMsg::SetValues(values) => {
                self.values = Some(values);
                self.selected = 0;
                true
            }
            FilterDropDownMsg::ItemSelect => {
                if let Some(ref values) = self.values {
                    match values.get(self.selected) {
                        None => {
                            console::error_1(&"Selected out-of-bounds".into());
                            false
                        }
                        Some(x) => {
                            self.on_select.as_ref().unwrap().emit(x.clone());
                            false
                        }
                    }
                } else {
                    console::error_1(&"No Values".into());
                    false
                }
            }
            FilterDropDownMsg::ItemDown => {
                self.selected += 1;
                if let Some(ref values) = self.values {
                    if self.selected >= values.len() {
                        self.selected = 0;
                    };
                };

                true
            }
            FilterDropDownMsg::ItemUp => {
                if let Some(ref values) = self.values {
                    if self.selected < 1 {
                        self.selected = values.len();
                    }
                }

                self.selected -= 1;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let body = html! {
            if let Some(ref values) = self.values {
                if !values.is_empty() {
                    {
                        for values
                            .iter()
                            .enumerate()
                            .map(|(idx, value)| {
                                let click = self.on_select.as_ref().unwrap().reform({
                                    let value = value.clone();
                                    move |_: MouseEvent| value.clone()
                                });

                                html! {
                                    if idx == self.selected {
                                        <span onmousedown={ click } class="selected">{ value }</span>
                                    } else {
                                        <span onmousedown={ click }>{ value }</span>
                                    }
                                }
                            })
                    }
                } else {
                    <span class="no-results">{ "No Completions" }</span>
                }
            }
        };

        html_template! {
            <style>
                { &CSS }
            </style>
            { body }
        }
    }
}
