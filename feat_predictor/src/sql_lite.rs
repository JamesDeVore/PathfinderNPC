extern crate rusqlite;

use rusqlite::Connection;
use rusqlite::Error;
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
// use std::io::{self, prelude::*, BufReader};

// pub fn create() -> Result<()> {
//     let conn = Connection::open("feats.db")?;

//     conn.execute(
//         "CREATE TABLE feats(id VARCHAR(4)  NOT NULL PRIMARY KEY ,feat_name    VARCHAR(34) NOT NULL,feat_type  VARCHAR(15) NOT NULL,description_text VARCHAR(339));",
//         NO_PARAMS,
//     )?;

//     Ok(())
// }

#[derive(Debug)]
struct Feat {
    id: String,
    feat_name: String,
}

// pub fn select() -> Result<()> {
//     let conn = Connection::open("feats.db")?;
//     // let file = File::open("./src/data/feats1.sql").unwrap();
//     // let reader = BufReader::new(file);

//     // for line in reader.lines() {
//     //     let formatted = line.unwrap();
//     //     println!("{}",&formatted );
//     //     conn.execute(&formatted,NO_PARAMS)?;
//     //     println!("Inserted one");
//     // }

//     let mut stmt = conn.prepare("SELECT * from feats where id = '50'")?;
//     let feats = stmt.query_map(NO_PARAMS, |row| {
//         Ok(Feat {
//             id: row.get(0)?,
//             feat_name: row.get(1)?,
//         })
//     })?;
//     for feat in feats {
//         println!("{:?}", feat.unwrap());
//     }

//     Ok(())
// }

pub fn select_all_feats() -> std::result::Result<HashMap<String, String>, Error> {
    let mut all_feats = HashMap::new();
    let conn = Connection::open("feats.db")?;
    let mut stmt = conn.prepare("SELECT * from feats")?;
    let feats = stmt.query_map(NO_PARAMS, |row| {
        Ok(Feat {
            id: row.get(0)?,
            feat_name: row.get(1)?,
        })
    })?;
    for feat in feats {
        let unwrapped = feat.unwrap();
        all_feats.insert(
            unwrapped.feat_name.to_string().trim().to_lowercase(),
            unwrapped.id.to_string(),
        );
    }
    Ok(all_feats)
}
pub fn select_all_feats_id() -> std::result::Result<HashMap<String, String>, Error> {
    let mut all_feats = HashMap::new();
    let conn = Connection::open("feats.db")?;
    let mut stmt = conn.prepare("SELECT * from feats")?;
    let feats = stmt.query_map(NO_PARAMS, |row| {
        Ok(Feat {
            id: row.get(0)?,
            feat_name: row.get(1)?,
        })
    })?;
    for feat in feats {
        let unwrapped = feat.unwrap();
        all_feats.insert(
            unwrapped.id.to_string(),
            unwrapped.feat_name.to_string().trim().to_lowercase(),
        );
    }
    Ok(all_feats)
}
