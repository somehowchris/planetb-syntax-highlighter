use log::*;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{EnumIter, ToString};

use yew::format::Json;
use yew::prelude::*;

use yew::services::storage::{Area, StorageService};

mod highlighter;

const KEY: &str = "yew.codestyle.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub enum  Language {
    C,
    CSharp,
    Python,
    CSS,
    Delphi,
    VisualBasic,
    Java,
    JavaScript,
    Ruby,
    SQL,
    XML,
    PHP
}

impl Language {
    pub fn to_class(self) -> String {
        match self {
            Self::C => "cpp".to_string(),
            Self::CSharp => "csharp".to_string(),
            Self::Python => "python".to_string(),
            Self::CSS => "css".to_string(),
            Self::Delphi => "delphi".to_string(),
            Self::VisualBasic => "vb".to_string(),
            Self::Java => "java".to_string(),
            Self::JavaScript => "js".to_string(),
            Self::Ruby => "ruby".to_string(),
            Self::SQL => "sql".to_string(),
            Self::XML => "xml".to_string(),
            Self::PHP => "php".to_string(),
        }
    }

    pub fn to_name(self) -> String {
        match self {
            Self::C => "C / C++".to_string(),
            Self::CSharp => "C#".to_string(),
            Self::Python => "Python".to_string(),
            Self::CSS => "CSS".to_string(),
            Self::Delphi => "Delphi".to_string(),
            Self::VisualBasic => "VisualBasic".to_string(),
            Self::Java => "Java".to_string(),
            Self::JavaScript => "JavaScript".to_string(),
            Self::Ruby => "Ruby".to_string(),
            Self::SQL => "SQL".to_string(),
            Self::XML => "HTML / XML".to_string(),
            Self::PHP => "PHP".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    show_info: bool,
    current_code: String,
    code_formatted: Option<String>,
    selected_programming_language: Option<Language>,
    fromatted_programming_language: Option<Language>,
    show_formatted: bool
}

pub enum Msg {
    HideInitMessage,
    Format,
    ChooseLangauge(Language),
    InputCode(String)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();

        let state = State {
            show_info: true,
            current_code: "".to_string(),
            code_formatted: None,
            selected_programming_language: None,
            fromatted_programming_language: None,
            show_formatted: true,
        };

        App {
            link,
            storage,
            state,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HideInitMessage => {
                self.state.show_info = false;
            },
            Msg::ChooseLangauge(langauge) => {
                self.state.selected_programming_language = Some(langauge);
            },
            Msg::InputCode(code) => {
                self.state.current_code = code;
            },
            Msg::Format => {
                self.state.show_formatted = false;
                self.state.code_formatted = Some(self.state.current_code);
                self.state.fromatted_programming_language = self.state.selected_programming_language;
                self.state.show_formatted = true;
                highlighter::highlight();
            }
        }
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");

        html! {
            <header>
                <div
                    class="page-header min-vh-100"
                    style="background-image: url(./assets/images/background.svg)"
                    loading="lazy"
                >
                    <span class="mask"></span>
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
                                            <div class="container">
                                                <div class="row">
                                                    <div class="col-lg-12 my-auto">
                                                        <h3>{"Do you need to highlight some code?"}</h3>
                                                            <p class="pe-4">
                                                                {"This small webapp uses the"}
                                                                <a
                                                                href="https://code.google.com/archive/p/syntaxhighlighter/"
                                                                >{"syntaxhighlighter"}</a
                                                                >
                                                                {"from Google Code Archive. Maybe you have seen it on"}
                                                                <a href="http://planetb.ca/syntax-highlight-word"
                                                                >{"planetb.ca"}</a
                                                                >{". Sadly the planetb one is most of the times just
                                                                not available. So have a go and see what you get."}

                                                                <br />
                                                                <br />
                                                                {"All the code is held inside your browser, nothing
                                                            leaves it. If you still worry and would like to host
                                                            it yourselves, have a look at the docker part in"}
                                                                <a
                                                                href="https://github.com/somehowchris/plantetb-syntax-highlighter#docker"
                                                                >{"this github repository"}</a
                                                                >
                                                                <br />
                                                                <br />
                                                            </p>

                                                            <div class="row">
                                                                <div class="col-md-12">
                                                                    {"It supports the following languages:"}
                                                                    <div class="row">
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"C++"}</li>
                                                                        </div>
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"C#"}</li>
                                                                        </div>
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"Python"}</li>
                                                                        </div>
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"CSS"}</li>
                                                                        </div>
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"Delphi"}</li>
                                                                        </div>
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"VB"}</li>
                                                                        </div>
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"Java"}</li>
                                                                        </div>
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"JavaScript"}</li>
                                                                        </div>
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"Ruby"}</li>
                                                                        </div>
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"Sql"}</li>
                                                                        </div>
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"XML/HTML"}</li>
                                                                        </div>
                                                                        <div
                                                                        class="col-sm-6 col-md-6 col-lg-4 col-xl-3"
                                                                        >
                                                                            <li>{"PHP"}</li>
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                            <div class="row">
                                                                <div class="col-12">
                                                                <br />
                                                                <button
                                                                    type="button"
                                                                    class="btn btn-outline-success"
                                                                    onclick=self.link.callback(|_| Msg::HideInitMessage)
                                                                >
                                                                    {"Let's go formatting"}
                                                                </button>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </section>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="container-fluid" style="height: 75vh" hidden={self.state.show_info}>
                        <div class="row" style="height: 100%;">
                            <div class="col-md-6">
                                <div class="card" style="height: 100%;">
                                    <div class="card-body">
                                        <div>
                                            <div class="col-md-12" style="height: 100%">
                                                <div class="row">
                                                    <div class="col-md-8 col-lg-7">
                                                        <div class="dropdown w-100">
                                                            <a
                                                            class="btn bg-gradient-dark dropdown-toggle"
                                                            data-bs-toggle="dropdown"
                                                            id="navbarDropdownMenuLink2"
                                                            >
                                                                <img
                                                                    src="./assets/cpp.png"  height="24"
                                                                />
                                                                { if self.state.selected_programming_language.is_none() { "Select a Programming language ..." } else { self.state.selected_programming_language.to_name() } }
                                                            </a>
                                                            <ul
                                                            class="dropdown-menu"
                                                            aria-labelledby="navbarDropdownMenuLink2"
                                                            >
                                                                <li 
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::C))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img src="./assets/cpp.png" height="24" />{"&nbsp;&nbsp;&nbsp;C++ / C"}
                                                                    </a>
                                                                </li>
                                                                <li
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::CSharp))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img
                                                                        src="./assets/csharp.png"  height="24"
                                                                    />
                                                                {" &nbsp;&nbsp;C#"}
                                                                    </a>
                                                                </li>
                                                                <li
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Python))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img
                                                                        src="./assets/python.png" height="24"
                                                                    />
                                                                    {"&nbsp;&nbsp;Python"}
                                                                    </a>
                                                                </li>
                                                                <li
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::CSS))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img
                                                                        src="./assets/css.png"  height="24"
                                                                    />
                                                                    {"&nbsp;&nbsp;CSS"}
                                                                    </a>
                                                                </li>
                                                                <li
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Delphi))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img
                                                                        src="./assets/delphi.png" height="24"
                                                                    />
                                                                    {"&nbsp;&nbsp;Delphi"}
                                                                    </a>
                                                                </li>
                                                                <li
                                                                onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::VisualBasic))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img
                                                                        src="./assets/viauslbasic.svg" height="24"
                                                                    />
                                                                    {"&nbsp;&nbsp;VisualBasic"}
                                                                    </a>
                                                                </li>
                                                                <li
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Java))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img
                                                                        src="./assets/java.png" height="24"
                                                                    />
                                                                    {"&nbsp;&nbsp;Java"}
                                                                    </a>
                                                                </li>
                                                                <li
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::JavaScript))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img
                                                                        src="./assets/javascript.png" height="24"
                                                                    />
                                                                    {"&nbsp;&nbsp;JavaScript"}
                                                                    </a>
                                                                </li>
                                                                <li
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Ruby))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img
                                                                        src="./assets/ruby.png" height="24"
                                                                    />
                                                                    {"&nbsp;&nbsp;Ruby"}
                                                                    </a>
                                                                </li>
                                                                <li
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::SQL))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img
                                                                        src="./assets/sql.png" height="24"
                                                                    />
                                                                    {"&nbsp;&nbsp;SQL"}
                                                                    </a>
                                                                </li>
                                                                <li
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::XML))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img
                                                                    src="./assets/html.png" height="24"
                                                                    />
                                                                    {"&nbsp;&nbsp;XML/HTML"}
                                                                    </a>
                                                                </li>
                                                                <li
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::PHP))
                                                                >
                                                                    <a class="dropdown-item" href="#">
                                                                    <img
                                                                    src="./assets/php.png" height="24"
                                                                    />
                                                                    {"&nbsp;&nbsp;PHP"}
                                                                    </a>
                                                                </li>
                                                            </ul>
                                                        </div>
                                                    </div>
                                                    <div class="col-md-4 col-lg-5" style="padding-right:0;">
                                                        <div class="text-right">
                                                            <button
                                                                type="button"
                                                                class="btn btn-outline-info"
                                                                style="display: block; margin-left: auto;"
                                                                onclick=self.link.callback(|_| Msg::Format)
                                                            >
                                                            {"Format"}
                                                            </button>
                                                        </div>
                                                    </div>
                                                </div>
                                                <div class="row" style="height: 100%;">
                                                    <div class="col-12" style="padding-right: 0;height: 100%">
                                                        <div class="input-group-outline input-group" style="height: 100%;">
                                                            <textarea
                                                                name="message"
                                                                class="form-control"
                                                                id="message"
                                                                height="100%"
                                                                style="height: calc(100% - 60px);"
                                                                onclick=self.link.callback(|e: InputData| Msg::InputCode(e.value))
                                                                placeholder="Just paste something and see what happens...."
                                                            ></textarea>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            { self.state.show_formatted && 
                                html! {
                                    <div class="col-md-6">
                                        <div class="card" style="height: 100%;">
                                            <div class="card-body">
                                                <textarea name="code" style="width:100%;height:100%" class={self.state.fromatted_programming_language.to_class()}>{self.state.code_formatted}</textarea>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }
                        </div>
                    </div>
                </div>
            </header>
        }
    }
}