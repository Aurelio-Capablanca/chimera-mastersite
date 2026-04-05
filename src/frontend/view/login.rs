use leptos::prelude::*; 
use leptos_meta::Title;
use leptos::{IntoView, component, view};


#[component]
pub fn Login() -> impl IntoView {
    view! {
        <Title text="welcome admin, please enter !"/>
        <h1>"Welcome Admin!"</h1>        
    }
}