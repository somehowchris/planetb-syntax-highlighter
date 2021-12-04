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
pub struct State {
    show_info: bool,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    HIDE_INIT_MESSAGE,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let state = State { show_info: true };
        App {
            link,
            storage,
            state,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
                <div class="container-fluid">
                    <div class="row">
                        <div class="col-md-6 offset-lg-2">
                            <div class="card" data-animation="true" hidden={self.state.show_info}>
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
                                                                onclick={Callback::from(|_| ())}
                                                            >
                                                                {"Don't show me this again"}
                                                            </button>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </section>
                                </div>
                            </div>
                            <br />
                            <br />
                        </div>
                    </div>
                </div>
            </div>
        </header>
        }
    }
}

impl App {}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {}

impl State {
    pub fn dont_show_again(&mut self) {
        self.show_info = false;
    }
}
