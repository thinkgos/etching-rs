use entity::{dict, dict::Column as DictColumn, dict::Entity as Dict};
use sea_query::Iden;

fn main() {
    println!("{}", DictColumn::Id.to_string())
}
