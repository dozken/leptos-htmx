use actix_web::{get, put, web, App, HttpRequest, HttpResponse, HttpServer};
use leptos::ssr::*;
use leptos::*;
use leptos_start::app::*;


#[get("/contact/1")]
async fn contact()-> HttpResponse {
    let contactData = ContactData {
        firstName: "John".to_string(),
        lastName: "Doe".to_string(),
        email: "doe@doe.do".to_string(),
    };

    let html = render_to_string(|cx| {
        view! { cx,
            <Contact contact=contactData />
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[put("/contact/1")]
async fn contact_update(form: web::Form<ContactData>)-> HttpResponse {
    let form = form.into_inner();
    let html = render_to_string(|cx| {
        view! { cx,
            <Contact contact=form />
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/contact/1/edit")]
async fn contact_edit() -> HttpResponse {
    let html = render_to_string(|cx| {
        view! { cx,
            <ContactEdit/>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn index(_req: HttpRequest) -> HttpResponse {
    let contactData = ContactData {
        firstName: "John".to_string(),
        lastName: "Doe".to_string(),
        email: "doe@doe.do".to_string(),
    };

    let html = render_to_string(|cx| {
        view! { cx,
            <head>
                <title>Leptos</title>
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <script src="https://unpkg.com/htmx.org@1.9.2"></script>
            </head>
            <body>
                <h1>"Leptos"</h1>
                <p>"Leptos is a Rust framework for building web apps."</p>
                <App />
            </body>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //            .wrap(middleware::Logger::default())
            //          .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
            .service(contact)
            .service(contact_edit)
            .service(contact_update)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// #[cfg(feature = "ssr")]
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_files::Files;
//     use actix_web::*;
//     use leptos::*;
//     use leptos_actix::{generate_route_list, LeptosRoutes};
//     use leptos_start::app::*;
//
//     let conf = get_configuration(None).await.unwrap();
//     let addr = conf.leptos_options.site_addr;
//     // Generate the list of routes in your Leptos App
//     let routes = generate_route_list(|cx| view! { cx, <App/> });
//
//     // Explicit server function registration is no longer required
//     // on the main branch. On 0.3.0 and earlier, uncomment the lines
//     // below to register the server functions.
//     // _ = GetPost::register();
//     // _ = ListPostMetadata::register();
//
//     HttpServer::new(move || {
//         let leptos_options = &conf.leptos_options;
//         let site_root = &leptos_options.site_root;
//
//         App::new()
//             .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
//             .leptos_routes(
//                 leptos_options.to_owned(),
//                 routes.to_owned(),
//                 |cx| view! { cx, <App/> },
//             )
//             .service(Files::new("/", site_root))
//         //.wrap(middleware::Compress::default())
//     })
//     .bind(&addr)?
//     .run()
//     .await
// }
//
// #[cfg(not(feature = "ssr"))]
// pub fn main() {
//     // no client-side main function
//     // unless we want this to work with e.g., Trunk for pure client-side testing
//     // see lib.rs for hydration function instead
// }
