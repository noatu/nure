use crate::Result;
use crate::port::search::{Data, Entry, Mode, Order, SearchRepository};

// use chrono::Utc;
use futures::TryStreamExt;
use sqlx::{Executor, MySql, QueryBuilder, Row};

pub struct SearchAdapter;

impl<E> SearchRepository<E> for SearchAdapter
where
    E: Send,
    for<'a> &'a E: Executor<'a, Database = MySql>,
{
    async fn search(connection: &E, data: Data) -> Result<Vec<Entry>> {
        let mut builder = QueryBuilder::new(
            "SELECT \
                p.id, p.name, p.version, p.url, p.description, \
                p.updated_at, p.created_at, \
                pb.id AS base_id, pb.name AS base_name, \
            ( \
                SELECT COUNT(DISTINCT pbur.user) \
                FROM PackageBaseUserRoles pbur \
                WHERE pbur.base = pb.id AND pbur.role = 3 \
            ) AS maintainers_num \
            FROM \
                Packages p \
            JOIN \
                PackageBases pb ON p.base = pb.id ",
        );

        let mut push_search = |cond, param| {
            builder.push(format_args!(
                " {cond} {param} {} ",
                if data.exact { "=" } else { "LIKE" }
            ));
            builder.push_bind(if data.exact {
                data.search.to_string()
            } else {
                format!("%{}%", data.search.as_str())
            });
        };

        let join_user = " JOIN PackageBaseUserRoles pbur ON pb.id = pbur.base \
                JOIN Users u ON pbur.user = u.id WHERE ";

        match data.mode {
            Mode::Url => push_search("WHERE", "p.url"),
            Mode::Name => push_search("WHERE", "p.name"),
            Mode::PackageBase => push_search("WHERE", "pb.name"),
            Mode::Description => push_search("WHERE", "p.description"),
            Mode::BaseDescription => push_search("WHERE", "pb.description"),
            Mode::NameAndDescription => {
                // WHERE (p.name LIKE '%search_term%' OR p.description LIKE '%search_term%')
                builder.push(" WHERE p.name LIKE ");
                builder.push_bind(format!("%{}%", data.search.as_str()));
                builder.push(" OR p.description LIKE ");
                builder.push_bind(format!("%{}%", data.search.as_str()));
            }
            Mode::User => {
                push_search(
                    "WHERE EXISTS ( \
                    SELECT 1 \
                    FROM PackageBaseUserRoles pbur \
                    JOIN Users u ON pbur.user = u.id \
                    WHERE pbur.base = pb.id AND",
                    "u.name",
                );
                builder.push(" ) ");
            }
            Mode::Flagger => {
                push_search(join_user, "u.name");
                builder.push(" AND pbur.role = 4 ");
            } // 4
            Mode::Packager => {
                push_search(join_user, "u.name");
                builder.push(" AND pbur.role = 2 ");
            } // 2
            Mode::Submitter => {
                push_search(join_user, "u.name");
                builder.push(" AND pbur.role = 1 ");
            } // 1
            Mode::Maintainer => {
                push_search(join_user, "u.name");
                builder.push(" AND pbur.role = 3 ");
            } // 3
        }

        builder.push(format_args!(
            " ORDER BY {} {} LIMIT {};",
            match data.order {
                Order::Name => "p.name",
                Order::Version => "p.version",
                Order::BaseName => "pb.name",
                Order::UpdatedAt => "p.updated_at",
                Order::CreatedAt => "p.created_at",
            },
            if data.ascending { "ASC" } else { "DESC" },
            data.limit
        ));

        let mut entries = Vec::new();

        let mut rows = builder.build().fetch(connection);
        while let Some(row) = rows.try_next().await? {
            entries.push(Entry {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                version: row.try_get("version")?,
                base_id: row.try_get("base_id")?,
                base_name: row.try_get("base_name")?,
                url: row.try_get("url")?,
                description: row.try_get("description")?,
                // submitter_id: row.try_get("submitter_id")?,
                // submitter_name: row.try_get("submitter_name")?,
                updated_at: row.try_get("updated_at")?,
                created_at: row.try_get("created_at")?,
            });
        }

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validation;
    use crate::port::search::Search;
    use sqlx::MySqlPool;

    #[sqlx::test]
    async fn search() -> crate::Result {
        let pool = MySqlPool::connect_lazy(
            &std::env::var("DATABASE_URL")
                .expect("environment variable `DATABASE_URL` should be set"),
        )?;

        let data = Data {
            mode: Mode::NameAndDescription,
            order: Order::UpdatedAt,
            search: Search::new("f").map_err(|e| e.1)?,
            limit: 50,
            exact: true,
            ascending: false,
        };

        SearchAdapter::search(&pool, data).await?;

        Ok(())
    }
}
