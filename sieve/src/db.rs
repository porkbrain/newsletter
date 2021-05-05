use crate::prelude::*;
use crate::select::{Deal, Voucher};
use sqlite::Connection;

pub fn insert(
    conn: &Connection,
    newsletter_id: &str,
    deals: Vec<Deal>,
    vouchers: Vec<Voucher>,
) -> Result<(), Error> {
    let sql = format!(
        "INSERT INTO offers (s3_key, deal, voucher, link) VALUES {}",
        (0..(deals.len() + vouchers.len()))
            .map(|_| "(?, ?, ?, ?)".to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    let mut statement = conn.prepare(sql)?;
    let mut binding_index = 1;

    for deal in &deals {
        statement.bind(binding_index, newsletter_id)?;
        statement.bind(binding_index + 1, deal.text.as_str())?;
        statement.bind(binding_index + 2, None::<&str>)?; // no voucher
        statement
            .bind(binding_index + 3, deal.link.as_ref().map(|s| s.as_str()))?;
        binding_index += 4;
    }

    for voucher in &vouchers {
        statement.bind(binding_index, newsletter_id)?;
        statement.bind(binding_index + 1, voucher.phrase.as_str())?;
        statement.bind(binding_index + 2, voucher.text.as_str())?;
        statement.bind(
            binding_index + 3,
            voucher.link.as_ref().map(|s| s.as_str()),
        )?;
        binding_index += 4;
    }

    loop {
        if matches!(statement.next()?, sqlite::State::Done) {
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_inserts() {
        let deals = vec![
            Deal::new(0, "deal1".to_string(), 0.9677482825634689),
            Deal::new(0, "deal2".to_string(), 0.964979770972405),
        ];
        let mut vouchers = vec![
            Voucher::new(
                0,
                "voucher1".to_string(),
                "voucher1code".to_string(),
                0.9585623288901614,
            ),
            Voucher::new(
                0,
                "voucher2".to_string(),
                "voucher2code".to_string(),
                0.8922419680443364,
            ),
        ];
        vouchers[1].link = Some("hello".to_string());

        let conn = open_in_memory_conn();

        insert(&conn, "test", deals, vouchers).expect("Cannot insert offers");

        let mut rows = vec![];
        conn.iterate("SELECT * FROM offers", |columns| {
            assert_eq!(columns[0].0, "s3_key");
            assert_eq!(columns[0].1, Some("test"));

            assert_eq!(columns[1].0, "deal");
            assert_eq!(columns[2].0, "voucher");
            assert_eq!(columns[3].0, "link");

            let deal = columns[1].1.unwrap();
            rows.push(deal.to_string());
            match deal {
                "deal1" => {
                    assert_eq!(columns[2].1, None);
                    assert_eq!(columns[3].1, None);
                }
                "deal2" => {
                    assert_eq!(columns[2].1, None);
                    assert_eq!(columns[3].1, None);
                }
                "voucher1" => {
                    assert_eq!(columns[2].1, Some("voucher1code"));
                    assert_eq!(columns[3].1, None);
                }
                "voucher2" => {
                    assert_eq!(columns[2].1, Some("voucher2code"));
                    assert_eq!(columns[3].1, Some("hello"));
                }
                _ => panic!("unexpected deal"),
            }

            assert_eq!(columns[4].0, "state");
            assert_eq!(columns[4].1, Some("new"));

            assert_eq!(columns[5].0, "created_at");
            assert!(columns[5].1.unwrap().parse::<i64>().is_ok());

            true
        })
        .unwrap();

        assert_eq!(
            rows,
            vec![
                "deal1".to_string(),
                "deal2".to_string(),
                "voucher1".to_string(),
                "voucher2".to_string()
            ]
        );
    }

    const MIGRATION_01: &str = include_str!(
        "../../migrations/000001_create_inbound_emails_table.up.sql"
    );

    const MIGRATION_02: &str =
        include_str!("../../migrations/000002_create_offers_table.up.sql");

    fn open_in_memory_conn() -> Connection {
        let conn = Connection::open(":memory:").unwrap();

        conn.execute(MIGRATION_01)
            .expect("Cannot run first migration");
        conn.execute(MIGRATION_02)
            .expect("Cannot run second migration");

        conn
    }
}
