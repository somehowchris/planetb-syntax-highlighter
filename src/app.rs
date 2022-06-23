use log::debug;
use serde_derive::{Deserialize, Serialize};

use web_sys::Element;
use yew::prelude::*;
use yew::NodeRef;

use gloo_storage::LocalStorage;
use gloo_storage::Storage;

use crate::utils::{highlighter, images::build_webp_url};
use web_sys::HtmlInputElement;

const STATE_KEY: &str = "codestyle.state";

pub struct App {
    state: State,
    textarea_ref: NodeRef,
    webp_support: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct ProgrammingLanguage {
    pub name: &'static str,
    pub css_class: &'static str,
    pub image_file: &'static str,
    pub image_file_extension: &'static str,
}

impl ProgrammingLanguage {
    fn to_img_url(self, webp_support: bool) -> String {
        build_webp_url(self.image_file, self.image_file_extension, webp_support)
    }
}

const PROGRAMMING_LANGUAGES: [ProgrammingLanguage; 12] = [
    ProgrammingLanguage {
        name: "C / C++",
        css_class: "cpp",
        image_file: "images/cpp",
        image_file_extension: "png",
    },
    ProgrammingLanguage {
        name: "C#",
        css_class: "csharp",
        image_file: "images/csharp",
        image_file_extension: "png",
    },
    ProgrammingLanguage {
        name: "Python",
        css_class: "python",
        image_file: "images/python",
        image_file_extension: "png",
    },
    ProgrammingLanguage {
        name: "CSS",
        css_class: "css",
        image_file: "images/css",
        image_file_extension: "png",
    },
    ProgrammingLanguage {
        name: "Delphi",
        css_class: "delphi",
        image_file: "images/delphi",
        image_file_extension: "png",
    },
    ProgrammingLanguage {
        name: "VisualBasic",
        css_class: "vb",
        image_file: "images/viauslbasic",
        image_file_extension: "svg",
    },
    ProgrammingLanguage {
        name: "Java",
        css_class: "java",
        image_file: "images/java",
        image_file_extension: "png",
    },
    ProgrammingLanguage {
        name: "JavaScript",
        css_class: "js",
        image_file: "images/javascript",
        image_file_extension: "png",
    },
    ProgrammingLanguage {
        name: "Ruby",
        css_class: "ruby",
        image_file: "images/ruby",
        image_file_extension: "png",
    },
    ProgrammingLanguage {
        name: "SQL",
        css_class: "sql",
        image_file: "images/sql",
        image_file_extension: "png",
    },
    ProgrammingLanguage {
        name: "HTML / XML",
        css_class: "xml",
        image_file: "images/html",
        image_file_extension: "png",
    },
    ProgrammingLanguage {
        name: "PHP",
        css_class: "php",
        image_file: "images/php",
        image_file_extension: "png",
    },
];

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct State {
    pub show_info: bool,
    pub code: String,
    pub programming_language: Option<ProgrammingLanguage>,
}

#[derive(Serialize, Deserialize)]
pub struct StoredState {
    pub show_info: Option<bool>,
}

pub enum Msg {
    HideInitMessage(bool),
    ChooseLanguage(&'static ProgrammingLanguage),
    InputCode,
    WebPSupport(bool),
}

impl App {
    fn format_code(&mut self) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let elements = document.query_selector("div.dp-highlighter").unwrap();

        if let Some(element) = elements {
            element.remove();
        }

        highlighter::highlight();
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let mut state = State {
            show_info: true,
            code: "".to_string(),
            programming_language: None,
        };

        let local_storage: Result<StoredState, _> = LocalStorage::get(STATE_KEY);

        if let Ok(stored_state) = local_storage {
            if stored_state.show_info.is_some() {
                state.show_info = stored_state.show_info.unwrap();
            }
        }

        ctx.link().send_future(async {
            match crate::utils::images::has_webp_support().await {
                true => Self::Message::WebPSupport(true),
                false => Self::Message::WebPSupport(false),
            }
        });

        App {
            state,
            textarea_ref: NodeRef::default(),
            webp_support: None,
        }
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, _first_render: bool) {
        debug!("Rendering code change");
        if !self.state.code.trim().is_empty() && self.state.programming_language.is_some() {
            self.format_code();
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HideInitMessage(dont_show_again) => {
                self.state.show_info = false;

                let local_storage: Result<StoredState, _> = LocalStorage::get(STATE_KEY);

                if let Ok(mut stored_state) = local_storage {
                    stored_state.show_info = Some(!dont_show_again);

                    LocalStorage::set(STATE_KEY, stored_state).unwrap();
                } else {
                    LocalStorage::set(
                        STATE_KEY,
                        StoredState {
                            show_info: Some(!dont_show_again),
                        },
                    )
                    .unwrap();
                }
                true
            }
            Msg::ChooseLanguage(language) => {
                debug!("Selected {}", language.name);

                self.state.programming_language = Some(*language);
                true
            }
            Msg::InputCode => {
                self.state.code = self
                    .textarea_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                true
            }
            Msg::WebPSupport(state) => {
                self.webp_support = Some(state);

                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        debug!("rendered!");

        html! {
            <header>
                <div
                    class="page-header min-vh-100"
                    style="background-image: url(images/background.svg)"
                    loading="lazy"
                >
                    {
                        html! {
                            <div class="container-fluid" hidden={!self.state.show_info}>
                                <div class="row">
                                    <div class="col-md-6 offset-lg-2">
                                        <div class="card" data-animation="true">
                                            <div class="card-body">
                                                <section
                                                    class="py-9"
                                                    style="
                                                        padding: 16px !important;
                                                        padding-bottom: 0 !important;
                                                        padding-top: 16px !important;
                                                    "
                                                >
                                                    {
                                                        html! {
                                                            <div class="container">
                                                                <div class="row">
                                                                    <div class="col-lg-12 my-auto">
                                                                        <h3>{"Do you need to highlight some code?"}</h3>
                                                                            <p class="pe-4">
                                                                                {"This small webapp uses the"}
                                                                                <a href="https://code.google.com/archive/p/syntaxhighlighter/">
                                                                                    {"syntaxhighlighter"}
                                                                                </a>

                                                                                {"from Google Code Archive. Maybe you have seen it on"}

                                                                                <a href="http://planetb.ca/syntax-highlight-word">{"planetb.ca"}</a>

                                                                                {". Sadly the planetb one is most of the times just not available. So have a go and see what you get."}

                                                                                <br />
                                                                                <br />
                                                                                {"All the code is held inside your browser, nothing leaves it. If you still worry and would like to host it yourselves, have a look at the container part in"}
                                                                                <a href="https://github.com/somehowchris/planetb-syntax-highlighter#container">
                                                                                    {"this github repository"}
                                                                                </a>
                                                                                <br />
                                                                                <br />
                                                                            </p>
                                                                            <div class="row">
                                                                                <div class="col-md-12">
                                                                                    {"It supports the following languages:"}
                                                                                    <div class="row">
                                                                                        {
                                                                                            PROGRAMMING_LANGUAGES.iter().map(|language|{
                                                                                                html! {
                                                                                                    <div class="col-sm-6 col-md-6 col-lg-4 col-xl-3">
                                                                                                    <li>{language.name}</li>
                                                                                                </div>
                                                                                                }
                                                                                            }).collect::<Vec<_>>()
                                                                                        }
                                                                                    </div>
                                                                                </div>
                                                                            </div>
                                                                            <div class="row">
                                                                                <div class="col-12">
                                                                                <br />
                                                                                <button
                                                                                    type="button"
                                                                                    class="btn btn-outline-danger"
                                                                                    onclick={ctx.link().callback(|_| Msg::HideInitMessage(true))}
                                                                                    style="margin-right: 8px;"
                                                                                >
                                                                                    {"Don't show me this again"}
                                                                                </button>
                                                                                <button
                                                                                    type="button"
                                                                                    class="btn btn-outline-success"
                                                                                    onclick={ctx.link().callback(|_| Msg::HideInitMessage(false))}
                                                                                >
                                                                                    {"Let's go formatting"}
                                                                                </button>
                                                                            </div>
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        }
                                                    }
                                                </section>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                    {
                        html! {
                            <div class="container-fluid" style="height: 100vh;overflow-y: scroll" hidden={self.state.show_info}>
                                <div class="row" style="min-height: 87.5%;margin-top:12.5vh;height: auto;">
                                    <div class="col-md-6">
                                        <div class="card" style="min-height: 75%;">
                                            <div class="card-body">
                                                {
                                                    html!{
                                                        <div class="row">
                                                            <div class="col-md-12 h-100">
                                                                <div class="row">
                                                                    <div class="col-md-8 col-lg-7">
                                                                        <div class="dropdown w-100">
                                                                            <a
                                                                                class="btn bg-gradient-dark dropdown-toggle"
                                                                                data-bs-toggle="dropdown"
                                                                                id="navbarDropdownMenuLink2"
                                                                            >
                                                                                {
                                                                                    if let Some(programming_language) = self.state.programming_language {
                                                                                        if let Some(webp_support) = self.webp_support {
                                                                                            html!{<img src={programming_language.to_img_url(webp_support)}  height="24"/>}
                                                                                        } else {
                                                                                            html!{}
                                                                                        }
                                                                                    } else {
                                                                                        html!{}
                                                                                    }
                                                                                }

                                                                                {
                                                                                    if let Some(programming_language) = self.state.programming_language {
                                                                                        format!("  {name}", name=programming_language.name)
                                                                                    } else {
                                                                                        "Select a Programming language ...".to_string()
                                                                                    }
                                                                                }
                                                                            </a>
                                                                            <ul
                                                                                class="dropdown-menu"
                                                                                aria-labelledby="navbarDropdownMenuLink2"
                                                                            >
                                                                                {
                                                                                    if let Some(webp_support) = self.webp_support {
                                                                                        PROGRAMMING_LANGUAGES.iter().map(|language|{
                                                                                            html! {
                                                                                                <li onclick={ctx.link().callback(|_| Msg::ChooseLanguage(language))}>
                                                                                                    <a class="dropdown-item" href="#">
                                                                                                        <img src={language.to_img_url(webp_support)} height="24" />
                                                                                                        {"   "}{language.name}
                                                                                                    </a>
                                                                                                </li>
                                                                                            }
                                                                                        }).collect::<Vec<_>>()
                                                                                    } else {
                                                                                        [html!{}].to_vec()
                                                                                    }
                                                                                }
                                                                            </ul>
                                                                        </div>
                                                                    </div>
                                                                    <div class="col-md-4 col-lg-5" style="padding-right:0;">
                                                                        <div class="text-right">
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                                <div class="row" style="height: 100%;">
                                                                    <div class="col-12" style="padding-right: 8px;height: 100%;">
                                                                        <div class="input-group-outline input-group" style="height: 100%;">
                                                                            <textarea
                                                                                ref={self.textarea_ref.clone()}
                                                                                name="message"
                                                                                class="form-control"
                                                                                id="message"
                                                                                style={ format!("min-height: calc(75vh - 204px);overflow-y: hidden;{height}", height=if let Some(element) = self.textarea_ref.cast::<Element>(){format!("height: {px}px", px=element.scroll_height())} else {"".to_string()}) }
                                                                                oninput={ctx.link().callback(|_e| Msg::InputCode)}
                                                                                placeholder="Just paste something and see what happens...."
                                                                            ></textarea>
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            </div>
                                        </div>
                                    </div>
                                    {
                                        html! {
                                            <div class="col-md-6">
                                                <div class="card" style="min-height: 75%;">
                                                    <div class="card-body">
                                                        <pre name="code" style="width:100%;height:100%" class={ if let Some(language) = self.state.programming_language { language.css_class } else {""}}>{if !self.state.code.trim().is_empty() {self.state.code.as_str()} else {"Nothing to show...yet"}}</pre>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }
                                </div>
                            </div>
                        }
                    }
                </div>
            </header>
        }
    }
}
