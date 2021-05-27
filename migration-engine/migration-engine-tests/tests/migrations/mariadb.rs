use migration_engine_tests::sync_test_api::*;
use quaint::ast as quaint_ast;

#[test_connector(tags(Mariadb))]
fn foreign_keys_to_indexes_being_renamed_must_work(api: TestApi) {
    let dm1 = r#"
        model User {
            id String @id
            name String
            posts Post[]

            @@unique([name], map: "idxname")
        }

        model Post {
            id String @id
            author String
            author_rel User @relation(fields: [author], references: name)
        }
    "#;

    api.schema_push(dm1).send_sync().assert_green_bang();

    api.assert_schema()
        .assert_table("User", |table| {
            table.assert_index_on_columns(&["name"], |idx| idx.assert_name("idxname"))
        })
        .unwrap()
        .assert_table("Post", |table| {
            table.assert_fk_on_columns(&["author"], |fk| fk.assert_references("User", &["name"]))
        })
        .unwrap();

    let insert_post = quaint_ast::Insert::single_into(api.render_table_name("Post"))
        .value("id", "the-post-id")
        .value("author", "steve");

    let insert_user = quaint::ast::Insert::single_into(api.render_table_name("User"))
        .value("id", "the-user-id")
        .value("name", "steve");

    api.query(insert_user.into());
    api.query(insert_post.into());

    let dm2 = r#"
        model User {
            id String @id
            name String
            posts Post[]

            @@unique([name], map: "idxrenamed")
        }

        model Post {
            id String @id
            author String
            author_rel User @relation(fields: [author], references: name)
        }
    "#;

    api.schema_push(dm2).send_sync().assert_green_bang();

    api.assert_schema()
        .assert_table("User", |table| {
            table.assert_index_on_columns(&["name"], |idx| idx.assert_name("idxrenamed"))
        })
        .unwrap()
        .assert_table("Post", |table| {
            table.assert_fk_on_columns(&["author"], |fk| fk.assert_references("User", &["name"]))
        })
        .unwrap();
}
