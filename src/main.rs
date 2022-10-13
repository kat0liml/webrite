#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_codegen;
#[macro_use] extern crate rocket_contrib;

use maud::{DOCTYPE, Markup, html};
use rocket_contrib::serve::StaticFiles;

const TITLE: &'static str = "Webrite";

fn footer(sitename: &'static str, link: &'static str) -> Markup {
  html! {
    footer {
      "Copyright (C) "
      a href=(link) { (sitename) };
      " All Right Reserved."
    }
  }
}

fn layout(body: Markup) -> Markup {
  html! {
    (DOCTYPE)
    html {
      head {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width,initial-scale=1";
        link rel="stylesheet" type="text/css" href="/static/css/style.css";
        title { (TITLE) }
      }
      body {
        main {
          (body)
          (footer(TITLE, "/"))
        }
      }
    }
  }
}


#[get("/")]
fn index() -> Markup {
  layout(html! {
    h1 { "ok" }
  })
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index])
    .mount("/static", StaticFiles::from("./static"))
    .launch();
}
