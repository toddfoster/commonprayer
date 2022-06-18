mod hymn_media;
use std::sync::Arc;

pub use hymn_media::*;

use crate::views::{DocumentView, Header};
use hymnal::{Hymn, HymnNumber, Hymnal, Hymnals};
use leptos2::*;

#[derive(Params)]
pub struct HymnViewParams {
    hymnal: Hymnals,
    number: HymnNumber,
}

pub struct HymnView {
    hymnal: Hymnal,
    hymn: Hymn,
}

#[async_trait(?Send)]
impl Loader for HymnView {
    type Params = HymnViewParams;
    type Query = ();

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let hymnal: Hymnal = params.hymnal.into();
        let hymn = hymnal
            .hymns
            .iter()
            .find(|s_hymn| s_hymn.number == params.number)?
            .clone();

        Some(HymnView { hymnal, hymn })
    }
}

impl View for HymnView {
    fn title(&self) -> String {
        format!(
            "{} {} – {} – {}",
            self.hymn.number,
            self.hymn.title,
            self.hymnal.title,
            t!("common_prayer")
        )
    }

    fn styles(&self) -> Styles {
        vec![include_str!("hymn.css").into()]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        let hymnal = &self.hymnal;
        let hymn = &self.hymn;

        let hymnary_hymnal_id = match hymnal.id {
            Hymnals::Hymnal1982 => "EH1982",
            Hymnals::LEVAS => "LEVS1993",
            Hymnals::WLP => "WLP1997",
            Hymnals::ElHimnario => "EH1998",
        };
        let hymnary_hymn_link = format!(
            "https://hymnary.org/hymn/{}/{}",
            &hymnary_hymnal_id, hymn.number
        );

        view! {
            <div>
                <header><h1>{format!("{} {}", hymn.number, hymn.title)}</h1></header>
                <main>
                    <h2>
                        <a href={&format!("../../{:#?}", hymnal.id)}>
                            {&hymnal.title}
                        </a>
                        " "
                        {hymn.number.to_string()}
                    </h2>

                    // Hymn metadata
                    <h3>{&hymn.title}</h3>

                    <dl>
                        <dt>{t!("hymnal.tune")}</dt>
                        <dd class="tune">{hymn.tune.to_lowercase()}</dd>
                        {possible_field(&t!("hymnal.authors"), &hymn.authors)}
                        {possible_field(&t!("hymnal.composers"), &hymn.composers)}
                        {possible_field(&t!("hymnal.meter"), &hymn.meter)}
                        {possible_field(&t!("hymnal.text_sources"), &hymn.text_sources)}
                        {possible_field(&t!("hymnal.tune_sources"), &hymn.tune_sources)}
                    </dl>

                    // Links to RiteSong and Hymnary
                    <p class="hymnary-link">
                        {t!("hymnal.more_info")}
                        " "
                        {rite_song_link(&hymn.source, &hymn.number).map(|link| view! {
                            <span>
                                <a class="hymnary-link" href={link} target="_blank">"ritesong"</a>
                                " "
                                {t!("or")}
                                " "
                            </span>
                        })}
                        <a class="hymnary-link" href={hymnary_hymn_link} target="_blank">"Hymnary.org"</a>
                        "."
                    </p>

                    <HymnMedia
                        hymnal={hymnal.id}
                        number={hymn.number}
                        text={&hymn.text}
                        copyright={hymn.copyright_restriction}
                        page={hymn.page_number}
                        mode={if !hymn.text.is_empty() {
                            HymnMediaShowing::Text
                        } else if !hymn.copyright_restriction {
                            HymnMediaShowing::PageScan
                        } else {
                            HymnMediaShowing::Video
                        }}
                    />

                    // Copyright notice in footer
                    <footer>
                        {t!("hymnal.copyright_footer")}
                    </footer>
                </main>
            </div>
        }
    }
}

fn possible_field(label: &str, value: &str) -> Vec<Node> {
    if value.is_empty() {
        Vec::new()
    } else {
        view! {
            <>
                <dt>{label}</dt>
                <dd>{escape_italics(value)}</dd>
            </>
        }
    }
}

fn escape_italics(original: &str) -> Vec<Node> {
    original
        .split("<i>")
        .flat_map(|piece| piece.split("</i>"))
        .enumerate()
        .map(|(idx, piece)| {
            if idx % 2 == 0 {
                text(piece.to_string())
            } else {
                // every odd character piece will be *after* a <i> but before a </i>
                view! { <i>{piece}</i> }
            }
        })
        .collect()
}

fn rite_song_link(hymnal: &Hymnals, number: &HymnNumber) -> Option<String> {
    let id = match (hymnal, number) {
        (Hymnals::Hymnal1982, HymnNumber::S(n)) => Some(1353 + (n - 1)),
        (Hymnals::Hymnal1982, HymnNumber::H(n)) => Some(193 + (n - 1)),
        (Hymnals::LEVAS, HymnNumber::H(n)) => Some(913 + (n - 1)),
        (Hymnals::LEVAS, HymnNumber::S(n)) => Some(913 + (n - 1)),
        (Hymnals::WLP, HymnNumber::H(n)) => Some(1968 + (n - 721)),
        (Hymnals::WLP, HymnNumber::S(n)) => Some(1968 + (n - 721)),
        (Hymnals::ElHimnario, _) => None,
    };

    let base = match (hymnal, number) {
        (Hymnals::Hymnal1982, HymnNumber::S(_n)) => {
            Some("https://www.riteseries.org/song/Hymnal1982ServiceMusic/")
        }
        (Hymnals::Hymnal1982, HymnNumber::H(_n)) => {
            Some("https://www.riteseries.org/song/Hymnal1982/")
        }
        (Hymnals::LEVAS, _) => Some("https://www.riteseries.org/song/levs/"),
        (Hymnals::WLP, _) => Some("https://www.riteseries.org/song/wlp/"),
        (Hymnals::ElHimnario, _) => None,
    };

    if let (Some(base), Some(id)) = (base, id) {
        Some(format!("{}{}/", base, id))
    } else {
        None
    }
}
