use chrono::{Local, NaiveDate};
use leptos::prelude::*;
use leptos_calendar::*;

#[component]
pub fn CalendarPage() -> impl IntoView {
    let calendar_config = CalendarConfig {
        active_classes: Some("border-pink-400 border-2".to_string()),
        header_classes: Some("bg-slate-900 text-white".to_string()),
        month_classes: Some("bg-black p-3 text-white text-center text-4xl font-normal".to_string()),
        cell_classes: Some(
            "bg-black text-white text-center py-3 px-4 border aspect-square".to_string(),
        ),
        ..Default::default()
    };
    let selected_date = RwSignal::new(
        NaiveDate::from_ymd_opt(2025, 7, 28)
            .and_then(|x| x.and_hms_opt(12, 12, 12))
            .and_then(|x| Some(x.and_local_timezone(Local).unwrap()))
            .unwrap(),
    );

    view! {
        <CalendarRoot config=calendar_config>
            <Calendar date=selected_date class="border-2 border-white bg-slate-50"/>
        </CalendarRoot>
    }
}
