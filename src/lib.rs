use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::Client;
use wasm_bindgen::prelude::*;
use std::fmt::Write;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/book_collection.graphql",
    response_derives = "Debug"
)]
struct BookCollection;

/* 
* Getting and Rendering Contents from Contentful
* @author=gsingh
*/
#[wasm_bindgen(start)]
pub async fn run()  {
    let contentful_space_id ="CONTENTFUL_SPACE_ID";
    let contentful_access_token = "CONTENTFUL_ACCESS_TOKEN";

     //Graphql URL
     let graphql_url=format!("https://graphql.contentful.com/content/v1/spaces/{}",contentful_space_id);

     //Graphql Varaibles to Store Response Informations
     let variables=book_collection::Variables;
     
 
     //Building Request Cleint with Necessary Headers
     let client = Client::builder()
         .default_headers(std::iter::once((
             reqwest::header::AUTHORIZATION,
             reqwest::header::HeaderValue::from_str(&format!("Bearer {}", contentful_access_token))
             .unwrap()
         )).collect(),
     )
         .default_headers(std::iter::once((
             reqwest::header::CONTENT_TYPE,
             reqwest::header::HeaderValue::from_str("application/json")
             .unwrap()
         )).collect(),
     )
     .build()
     .unwrap();
 
     let response_body = post_graphql::<BookCollection,_>(&client, graphql_url, variables).await.unwrap();
 
     //function for rendering the Response into HTML Templates
     render_response(response_body); 

}

/*
* Writing the inner html by formating the Parsed Responses
* @author = gsingh
*/
fn render_response(response: graphql_client::Response<book_collection::ResponseData>) {

    let parent = document().body().expect_throw("no body");
    let json_data: graphql_client::Response<book_collection::ResponseData> = response;

    let response = document()
        .create_element("div")
        .expect_throw("could not create div");

    let mut inner_html = String::new();

    let book_listings = json_data
        .data
        .expect_throw("response data")
        .book_collection
        .expect_throw("reddit")
        .items;

    for book in book_listings.iter().flatten() {
        write!(
            inner_html,
            r#"
            <div class="card text-white bg-dark mb-3" style="max-width: 30rem;">
            <img class="card-img-top"alt="{}" src="{}">
            <div class="card-body">
              <h5 class="card-title">{}</h5>
              <p class="card-text">{},00€</p>
              <a href="" class="btn btn-primary">Add to Cart</a>
            </div>
          </div>
                    "#
            ,book.image.as_ref().unwrap().title.as_ref().unwrap(),book.image.as_ref().unwrap().url.as_ref().unwrap().as_str(),book.title.as_ref().unwrap(),book.price.as_ref().unwrap()
        )
        .expect_throw("write to string");
    }


    
    response.set_inner_html(&format!(
        "<div class=\"display-1 justify-content-center \">WebR's Book Collection</div>
        <div class=\"container\">
            <div class=\"col\">
            {}
            </div>
        </div>",
        inner_html
    ));

    parent
        .append_child(&response)
        .expect_throw("could not append response");
}

fn document() -> web_sys::Document {
    web_sys::window()
        .expect_throw("no window")
        .document()
        .expect_throw("no document")
}