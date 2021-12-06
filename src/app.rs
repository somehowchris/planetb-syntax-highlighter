use log::*;
use serde_derive::{Deserialize, Serialize};

use yew::prelude::*;

use yew::format::Json;
use yew::services::storage::{Area, StorageService};

mod highlighter;

const STATE_KEY: &str = "codestyle.state";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Language {
    C,
    CSharp,
    Python,
    Css,
    Delphi,
    VisualBasic,
    Java,
    JavaScript,
    Ruby,
    Sql,
    Xml,
    Php,
}

impl Language {
    pub fn to_class(self) -> String {
        match self {
            Self::C => "cpp".to_string(),
            Self::CSharp => "csharp".to_string(),
            Self::Python => "python".to_string(),
            Self::Css => "css".to_string(),
            Self::Delphi => "delphi".to_string(),
            Self::VisualBasic => "vb".to_string(),
            Self::Java => "java".to_string(),
            Self::JavaScript => "js".to_string(),
            Self::Ruby => "ruby".to_string(),
            Self::Sql => "sql".to_string(),
            Self::Xml => "xml".to_string(),
            Self::Php => "php".to_string(),
        }
    }

    pub fn to_name(self) -> String {
        match self {
            Self::C => "C / C++".to_string(),
            Self::CSharp => "C#".to_string(),
            Self::Python => "Python".to_string(),
            Self::Css => "CSS".to_string(),
            Self::Delphi => "Delphi".to_string(),
            Self::VisualBasic => "VisualBasic".to_string(),
            Self::Java => "Java".to_string(),
            Self::JavaScript => "JavaScript".to_string(),
            Self::Ruby => "Ruby".to_string(),
            Self::Sql => "SQL".to_string(),
            Self::Xml => "HTML / XML".to_string(),
            Self::Php => "PHP".to_string(),
        }
    }
    pub fn to_file_path(self) -> String {
        match self {
            Self::C => "./assets/images/cpp.png".to_string(),
            Self::CSharp => "./assets/images/csharp.png".to_string(),
            Self::Python => "./assets/images/python.png".to_string(),
            Self::Css => "./assets/images/css.png".to_string(),
            Self::Delphi => "./assets/images/delphi.png".to_string(),
            Self::VisualBasic => "./assets/images/viauslbasic.svg".to_string(),
            Self::Java => "./assets/images/java.png".to_string(),
            Self::JavaScript => "./assets/images/javascript.png".to_string(),
            Self::Ruby => "./assets/images/ruby.png".to_string(),
            Self::Sql => "./assets/images/sql.png".to_string(),
            Self::Xml => "./assets/images/html.png".to_string(),
            Self::Php => "./assets/images/php.png".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub show_info: bool,
    pub code: String,
    pub programming_language: Option<Language>,
}

#[derive(Serialize, Deserialize)]
pub struct StoredState {
    pub show_info: Option<bool>,
}

pub enum Msg {
    HideInitMessage(bool),
    ChooseLangauge(Language),
    InputCode(String),
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

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();

        let Json(stored_state_json): Json<Result<StoredState, anyhow::Error>> =
            storage.restore(STATE_KEY);

        let mut state = State {
            show_info: true,
            code: "".to_string(),
            programming_language: None,
        };

        if let Ok(stored_state) = stored_state_json {
            if stored_state.show_info.is_some() {
                state.show_info = stored_state.show_info.unwrap();
            }
        }

        App {
            link,
            storage,
            state,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, _first_render: bool) {
        debug!("Rendering code change");
        if !self.state.code.is_empty() && self.state.programming_language.is_some() {
            self.format_code();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HideInitMessage(dont_show_again) => {
                self.state.show_info = false;

                let Json(stored_state_json): Json<Result<StoredState, anyhow::Error>> =
                    self.storage.restore(STATE_KEY);

                if let Ok(mut stored_state) = stored_state_json {
                    stored_state.show_info = Some(!dont_show_again);

                    self.storage.store(STATE_KEY, Json(&stored_state));
                } else {
                    let state = StoredState {
                        show_info: Some(!dont_show_again),
                    };

                    self.storage.store(STATE_KEY, Json(&state));
                }
                true
            }
            Msg::ChooseLangauge(langauge) => {
                self.state.programming_language = Some(langauge);
                debug!("Selected {}", langauge.to_name());
                true
            }
            Msg::InputCode(code) => {
                self.state.code = code;
                true
            }
        }
    }

    fn view(&self) -> Html {
        debug!("rendered!");
        html! {
                <header>
                    <div
                        class="page-header min-vh-100"
                        style="background-image: url(./assets/images/background.svg)"
                        loading="lazy"
                    >
                        <span class="mask"></span>
                        { html! {<div class="container-fluid" hidden={!self.state.show_info}>
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
                                                { html! {<div class="container">
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
                                                                        class="btn btn-outline-danger"
                                                                        onclick=self.link.callback(|_| Msg::HideInitMessage(true))
                                                                        style="margin-right: 8px;"
                                                                    >
                                                                        {"Don't show me this again"}
                                                                    </button>
                                                                    <button
                                                                        type="button"
                                                                        class="btn btn-outline-success"
                                                                        onclick=self.link.callback(|_| Msg::HideInitMessage(false))
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
                        { html! {<div class="container-fluid" style="height: 75vh" hidden={self.state.show_info}>
                            <div class="row" style="height: 100%;">
                                <div class="col-md-6">
                                    <div class="card" style="height: 100%;">
                                        <div class="card-body">
                                            { html!{<div style="height:100%">
                                                <div class="col-md-12" style="height: 100%">
                                                    <div class="row">
                                                        <div class="col-md-8 col-lg-7">
                                                            <div class="dropdown w-100">
                                                                <a
                                                                class="btn bg-gradient-dark dropdown-toggle"
                                                                data-bs-toggle="dropdown"
                                                                id="navbarDropdownMenuLink2"
                                                                >
                                                                    {
                                                                        if self.state.programming_language.is_some() {
                                                                            html!{<img src={self.state.programming_language.unwrap().to_file_path()}  height="24"/>}
                                                                        } else {
                                                                            html!{}
                                                                        }
                                                                    }
                                                                    { if self.state.programming_language.is_none() { "Select a Programming language ...".to_string() } else { format!("  {}",self.state.programming_language.unwrap().to_name()) } }
                                                                </a>
                                                                <ul
                                                                class="dropdown-menu"
                                                                aria-labelledby="navbarDropdownMenuLink2"
                                                                >
                                                                    <li
                                                                        onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::C))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img src="./assets/images/cpp.png" height="24" />{"   C++ / C"}
                                                                        </a>
                                                                    </li>
                                                                    <li
                                                                        onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::CSharp))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img
                                                                            src="./assets/images/csharp.png"  height="24"
                                                                        />
                                                                    {"   C#"}
                                                                        </a>
                                                                    </li>
                                                                    <li
                                                                        onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Python))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img
                                                                            src="./assets/images/python.png" height="24"
                                                                        />
                                                                        {"  Python"}
                                                                        </a>
                                                                    </li>
                                                                    <li
                                                                        onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Css))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img
                                                                            src="./assets/images/css.png"  height="24"
                                                                        />
                                                                        {"  CSS"}
                                                                        </a>
                                                                    </li>
                                                                    <li
                                                                        onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Delphi))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img
                                                                            src="./assets/images/delphi.png" height="24"
                                                                        />
                                                                        {"  Delphi"}
                                                                        </a>
                                                                    </li>
                                                                    <li
                                                                    onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::VisualBasic))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img
                                                                            src="./assets/images/viauslbasic.svg" height="24"
                                                                        />
                                                                        {"  VisualBasic"}
                                                                        </a>
                                                                    </li>
                                                                    <li
                                                                        onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Java))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img
                                                                            src="./assets/images/java.png" height="24"
                                                                        />
                                                                        {"  Java"}
                                                                        </a>
                                                                    </li>
                                                                    <li
                                                                        onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::JavaScript))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img
                                                                            src="./assets/images/javascript.png" height="24"
                                                                        />
                                                                        {"  JavaScript"}
                                                                        </a>
                                                                    </li>
                                                                    <li
                                                                        onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Ruby))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img
                                                                            src="./assets/images/ruby.png" height="24"
                                                                        />
                                                                        {"  Ruby"}
                                                                        </a>
                                                                    </li>
                                                                    <li
                                                                        onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Sql))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img
                                                                            src="./assets/images/sql.png" height="24"
                                                                        />
                                                                        {"  SQL"}
                                                                        </a>
                                                                    </li>
                                                                    <li
                                                                        onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Xml))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img
                                                                        src="./assets/images/html.png" height="24"
                                                                        />
                                                                        {"  XML/HTML"}
                                                                        </a>
                                                                    </li>
                                                                    <li
                                                                        onclick=self.link.callback(|_| Msg::ChooseLangauge(Language::Php))
                                                                    >
                                                                        <a class="dropdown-item" href="#">
                                                                        <img
                                                                        src="./assets/images/php.png" height="24"
                                                                        />
                                                                        {"  PHP"}
                                                                        </a>
                                                                    </li>
                                                                </ul>
                                                            </div>
                                                        </div>
                                                        <div class="col-md-4 col-lg-5" style="padding-right:0;">
                                                            <div class="text-right">
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
                                                                    oninput=self.link.callback(|e: InputData| Msg::InputCode(e.value))
                                                                    placeholder="Just paste something and see what happens...."
                                                                ></textarea>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>}
                                            }
                                        </div>
                                    </div>
                                </div>
                                {
                                    html! {
                                        <div class="col-md-6">
                                            <div class="card" style="height: 100%;">
                                                <div class="card-body">
                                                    <pre name="code" style="width:100%;height:100%" class={ if self.state.programming_language.is_some() { self.state.programming_language.unwrap().to_class()} else {"".to_string()}}>{if !self.state.code.is_empty() {self.state.code.as_str()} else {"Nothing to show...yet"}}</pre>
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
