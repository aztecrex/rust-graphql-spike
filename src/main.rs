#[macro_use] extern crate juniper;
extern crate serde_json;

use juniper::{FieldResult, Variables};


#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// There is also a custom derive for mapping GraphQL input objects.

#[derive(GraphQLInputObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// Now, we create our root Query and Mutation types with resolvers by using the
// graphql_object! macro.
// Objects can have contexts that allow accessing shared state like a database
// pool.

struct Context {
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

struct Query;


graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field human(&executor, id: String) -> FieldResult<Human> {

        let human = Human {
            id: "1234".to_string(),
            name: "Carl".to_string(),
            appears_in: Vec::new(),
            home_planet: "Mars".to_string(),

        };

        Ok(human)
    }
});


struct Mutation;

graphql_object!(Mutation: Context |&self| {

    field createHuman(&executor, new_human: NewHuman) -> FieldResult<NewHuman> {
        let human = NewHuman {
            name: "Nora".to_string(),
            appears_in: Vec::new(),
            home_planet: "Venus".to_string(),
        };
        Ok(human)
    }
});


type Schema = juniper::RootNode<'static, Query,Mutation>;


fn main() {

    // Create a context object.
    let ctx = Context {};

    // find a human
    let (res, _errors) = juniper::execute(
        "{ human (id: \"3\") {id name} }",
        None,
        &Schema::new(Query, Mutation),
        &Variables::new(),
        &ctx,
    ).unwrap();
    let s = serde_json::to_string(&res).unwrap();
    println!("a human: {}",s);

    // get the schema
    let (res, _errors) = juniper::execute(
        "{__schema {types {name}}}",
        // "{ human (id: \"3\") {id name} }",
        None,
        &Schema::new(Query, Mutation),
        &Variables::new(),
        &ctx,
    ).unwrap();
    let s = serde_json::to_string(&res).unwrap();
    println!("schema: {}",s);

}
