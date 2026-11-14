use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum NoticeType {
    Success,
    Error,
}

#[derive(Clone, PartialEq)]
pub struct Notice {
    pub kind: NoticeType,
    pub text: String,
}

pub fn success_notice(text: impl Into<String>) -> Notice {
    Notice {
        kind: NoticeType::Success,
        text: text.into(),
    }
}

pub fn error_notice(text: impl Into<String>) -> Notice {
    Notice {
        kind: NoticeType::Error,
        text: text.into(),
    }
}

pub fn render_notice(notice: Option<Notice>) -> Html {
    match notice {
        Some(notice) => {
            let class = match notice.kind {
                NoticeType::Success => "notice success",
                NoticeType::Error => "notice error",
            };
            html! { <p class={class}>{ notice.text }</p> }
        }
        None => html! {},
    }
}
