use leptos::*;
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/ssr_modes.css"/>
        <Title text="Welcome to Leptos"/>
        // <Contact />
    }
}

// #[component]
// pub fn Contact(cx: Scope) -> impl IntoView {
//
//     view! { cx,
//         <div hx-target="this" hx-swap="outerHTML">
//             <div><label>"First Name"</label>": Joe"</div>
//             <div><label>"Last Name"</label>": Blow"</div>
//             <div><label>"Email"</label>": joe@blow.com"</div>
//             <button hx-get="/contact/1/edit" class="btn btn-primary">
//                "Click To Edit"
//             </button>
//         </div>
//     }
// }
//
// #[component]
// pub fn ContactEdit(cx: Scope) -> impl IntoView {
//
//     view! { cx,
//         <form hx-put="/contact/1" hx-target="this" hx-swap="outerHTML">
//             <div>
//                 <label>"First Name"</label>
//                 <input type="text" name="firstName" value="Joe" />
//             </div>
//             <div class="form-group">
//                 <label>"Last Name"</label>
//                 <input type="text" name="lastName" value="Blow" />
//             </div>
//             <div class="form-group">
//                 <label>"Email Address"</label>
//                 <input type="email" name="email" value="joe@blow.com">
//             </div>
//             <button class="btn">"Submit"</button>
//             <button class="btn" hx-get="/contact/1">"Cancel"</button>
//         </form> 
//     }
// }
