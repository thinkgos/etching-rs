#[cfg(test)]
mod test {
    use entity::{announce::Column as AnnounceColumn, announce::Entity as Announce};

    use sea_orm::{DbBackend, EntityTrait, QuerySelect, QueryTrait};

    #[test]
    fn query_select() {
        let q = Announce::find()
            .select_only()
            .columns(vec![AnnounceColumn::Id, AnnounceColumn::Title])
            .build(DbBackend::MySql)
            .to_string();
        assert_eq!(
            q,
            "SELECT `announce`.`id`, `announce`.`title` FROM `announce`"
        );
    }
}
