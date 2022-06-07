use hymnal::*;
use itertools::Itertools;
use leptos2::*;

use crate::views::*;
use hymnal::{HymnMetadata, Hymnal, Hymnals};

#[derive(Deserialize)]
pub struct HymnalPageParams {
    hymnal: Option<Hymnals>,
}

#[derive(Deserialize)]
pub struct HymnalPageQuery {
    q: Option<String>,
}

pub struct HymnalPage {
    search: String,
    hymnal: Option<Hymnals>,
    search_results: Vec<HymnMetadata>,
    hymnals: Vec<Hymnal>,
}

impl Page for HymnalPage {
    type Params = HymnalPageParams;
    type Query = HymnalPageQuery;

    fn name() -> &'static str {
        "hymnal"
    }

    fn paths() -> Vec<String> {
        vec!["".into(), "{hymnal}".into()]
    }

    fn build_state(
        _locale: &str,
        _path: &str,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let hymnals = match params.hymnal {
            None => {
                vec![
                    HYMNAL_1982.to_metadata(),
                    LEVAS.to_metadata(),
                    WLP.to_metadata(),
                    EL_HIMNARIO.to_metadata(),
                ]
            }
            Some(hymnal_id) => {
                let hymnal: Hymnal = hymnal_id.into();
                vec![hymnal.to_metadata()]
            }
        };

        let search = query.q.unwrap_or_default();

        let s = Some(match params.hymnal {
            None => HymnalPage {
                search_results: HYMNAL_1982
                    .search(&search)
                    .chain(LEVAS.search(&search))
                    .chain(WLP.search(&search))
                    .chain(EL_HIMNARIO.search(&search))
                    .collect(),
                hymnals,
                search,
                hymnal: params.hymnal,
            },
            Some(hymnal_id) => {
                let hymnal: Hymnal = hymnal_id.into();
                HymnalPage {
                    search_results: hymnal.search(&search).collect(),
                    hymnals,
                    search,
                    hymnal: params.hymnal,
                }
            }
        });
        s
    }

    fn head(&self, _locale: &str) -> Vec<Node> {
        let title = if self.hymnals.len() == 1 {
            self.hymnals[0].title.clone()
        } else {
            t!("menu.hymnal")
        };

        view! {
            <>
                <title>{title} " – " {t!("common_prayer")}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/hymnal-search.css"/>
                <link rel="stylesheet" href="/static/toggle-links.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        /*         let results_query_status = if self.search_results.is_empty() {
                   FetchStatus::Idle
               } else {
                   FetchStatus::Success(Box::new(self.search_results.clone()))
               };

               let results_query_state = NestedState::new(Fetch::with_status(results_query_status));
        */

        let hymns = self
            .search_results
            .iter()
            .group_by(|hymn| hymn.source)
            .into_iter()
            .flat_map(|(hymnal_id, hymns)| {
                let hymnal = self
                    .hymnals
                    .iter()
                    .find(|s_hymnal| {
                        s_hymnal.id == hymnal_id
                            && (self.hymnal.is_none() || self.hymnal == Some(hymnal_id))
                    })
                    .map(hymnal_metadata);

                let hymns = hymns.into_iter().filter_map(|hymn| {
                    if self.hymnal.is_none() || self.hymnal == Some(hymn.source) {
                        Some(hymn_metadata(&locale, hymn))
                    } else {
                        None
                    }
                });

                hymnal.into_iter().chain(hymns)
            })
            .collect::<Vec<_>>();

        let q_link = if self.search.is_empty() {
            String::new()
        } else {
            format!("?q={}", self.search)
        };

        view! {
            <>
                {Header::new(locale, &t!("menu.hymnal")).to_node()}
                <main>
                    <form>
                        // Search bar
                        <label class="stacked">
                            {t!("search")}
                            <input
                                type="search"
                                name="q"
                                value={&self.search}
                                //prop:value={self.search.clone()}
                                //on:input=move |ev| HymnalSearchMsg::Search(event_target_value(&ev), current_hymnal)
                            />
                        </label>
                    </form>
                    <div class="toggle-links">
                        <a href={format!("/{}/hymnal{}", locale, q_link)} class:current={self.hymnal.is_none()}>{t!("hymnal.any")}</a>
                        <a href={format!("/{}/hymnal/Hymnal1982{}", locale, q_link)} class:current={self.hymnal == Some(Hymnals::Hymnal1982)}>{t!("hymnal.h82_abbrev")}</a>
                        <a href={format!("/{}/hymnal/LEVAS{}", locale, q_link)} class:current={self.hymnal == Some(Hymnals::LEVAS)}>{t!("hymnal.levas_abbrev")}</a>
                        <a href={format!("/{}/hymnal/WLP{}", locale, q_link)} class:current={self.hymnal == Some(Hymnals::WLP)}>{t!("hymnal.wlp_abbrev")}</a>
                        <a href={format!("/{}/hymnal/ElHimnario{}", locale, q_link)} class:current={self.hymnal == Some(Hymnals::ElHimnario)}>{t!("hymnal.el_himnario")}</a>
                    </div>
                    <section>{hymns}</section>
                    /* <HymnalSearch
                        locale={locale}
                        search={&self.search}
                        prop:state={results_query_state.clone()}
                        prop:hymnals={self.hymnals.clone()}
                    >
                    </HymnalSearch> */
                </main>
            </>
        }
    }
}

fn hymnal_metadata(hymnal: &Hymnal) -> Node {
    let title = view! { <h2>{&hymnal.title}</h2> };
    let subtitle = if !hymnal.subtitle.is_empty() {
        Some(view! { <h3>{&hymnal.subtitle}</h3> })
    } else {
        None
    };

    let copyright = view! { <p class="copyright">{&hymnal.copyright}</p> };

    view! {
        <article class="hymnal">
            {title}
            {subtitle}
            {copyright}
        </article>
    }
}

fn hymn_metadata(locale: &str, hymn: &HymnMetadata) -> Node {
    let link = format!("/{}/hymn/{:#?}/{}", locale, hymn.source, hymn.number);

    let number_str = match hymn.number {
        HymnNumber::S(n) => format!("S{}", n),
        HymnNumber::H(n) => n.to_string(),
    };

    let tune_name = if hymn.tune.starts_with('[') {
        ""
    } else {
        &hymn.tune
    }
    .to_lowercase();

    view! {
        <article class="hymn-listing">
            <a id={&format!("{:#?}-{}", hymn.source, number_str)}></a>
            <div class="primary">
                <span class="music-available">
                    {if hymn.copyright_restriction {
                        None
                    } else {
                        Some(view! { <img src={Icon::Music.to_string()} alt={t!("hymnal.music_available")}/> })
                    }}
                </span>
                <span class="text-available">
                    {if hymn.text_empty {
                        ""
                    } else {
                        "T"
                    }}
                </span>
                <a class="number" href={&link}>{number_str}</a>
                <a class="title" href={&link}>{&hymn.title}</a>
                <span class="tune">{&tune_name}</span>
            </div>
            <div class="secondary">
                <div>
                    {if hymn.authors.is_empty() {
                        None
                    } else {
                        Some(view! {
                            <span class="list-field author">
                                <span class="label">{t!("hymnal.text")} ": "</span>
                                {&hymn.authors}
                            </span>
                    })}}
                    {if hymn.composers.is_empty() {
                        None
                    } else {
                        Some(view! {
                            <span class="list-field composer">
                                <span class="label">{t!("hymnal.music")} ": "</span>
                                {&hymn.composers}
                            </span>
                    })}}
                </div>
                <span class="list-field meter">{&hymn.meter}</span>
            </div>
            <ul class="tag-list">
                {hymn.tags
                        .iter()
                        .map(|tag| view! {
                            <li>
                                <a href=&format!("?q=tag:{}", tag)>{tag}</a>
                            </li>
                        })
                        .collect::<Vec<_>>()
                }
            </ul>
        </article>
    }
}
