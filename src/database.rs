use rusqlite::{params, Connection, Result as SQLiteResult, Row, NO_PARAMS};
extern crate dirs;

const DATABASE_PATH: &'static str = 
    "Library/Containers/com.apple.Safari/Data/Library/WebKit/WebsiteData/ResourceLoadStatistics/observations.db";

const OBSERVED_DOMAINS: &'static str = "SELECT domainID, registrableDomain FROM ObservedDomains";
const SCOPED_DOMAINS: &'static str = "SELECT domainID, registrableDomain FROM ObservedDomains WHERE registrableDomain = ?";
const DOMAINS_AMOUNT: &'static str = "SELECT count(*) FROM ObservedDomains";
const DOMAIN_INFO: &'static str = "SELECT isPrevalent, isVeryPrevalent, timesAccessedAsFirstPartyDueToUserInteraction, timesAccessedAsFirstPartyDueToStorageAccessAPI FROM ObservedDomains WHERE domainID = ?";
#[derive(Default, Debug)]
pub struct Domain {
    pub id: i64,
    pub name: String,
    prevalent: bool,
    very_prevalent: bool,
    pub first_party_interaction: i32,
    pub first_party_store_access: i32,
}

impl Domain {
    pub fn is_prevalent(&self) -> &str {
        match self.prevalent {
            true => "Yes",
            false => "No",
        }
    } 

    pub fn is_very_prevalent(&self) -> &str {
        match self.very_prevalent {
            true => "Yes",
            false => "No",
        }
    } 
}

pub struct Database {
    connection: Connection,
    domains_scope: Option<Vec<String>>,
}

impl Database {
    pub fn connect(domains_scope: Option<Vec<String>>) -> SQLiteResult<Self> {
        let mut db_path = match dirs::home_dir() {
            Some(dir) => dir,
            None => panic!("Could not infer home directory."),
        };
        db_path.push(DATABASE_PATH);

        let connection = Connection::open(&db_path)?;
        Ok(Database {
            connection,
            domains_scope,
        })
    }

    pub fn get_domains(&self) -> SQLiteResult<Vec<Domain>> {
        let map_domains = |row: &Row| {
            Ok(Domain {
                id: row.get(0)?,
                name: row.get(1)?,
                ..Default::default()
            })
        };

        match &self.domains_scope {
            Some(scoped_domains) => {
                let domains = scoped_domains
                    .into_iter()
                    .map(|domain| {
                        self.connection
                            .query_row(&SCOPED_DOMAINS, params![domain], map_domains)
                            .expect("Failed to query row")
                    })
                    .collect();

                Ok(domains)
            }
            None => {
                let mut stmt = self.connection.prepare(&OBSERVED_DOMAINS)?;
                let domains = stmt
                    .query_map(NO_PARAMS, map_domains)?
                    .filter_map(|d| d.ok())
                    .collect();
                Ok(domains)
            }
        }
    }

    pub fn get_info(&self, domain: &Domain) -> SQLiteResult<Domain> {
        let info = self.connection
        .query_row(&DOMAIN_INFO, params![domain.id], |row| 
            Ok(Domain {
                id: domain.id,
                name: domain.name.clone(),
                prevalent: row.get(0)?,
                very_prevalent: row.get(1)?,
                first_party_interaction: row.get(2)?,
                first_party_store_access: row.get(3)?,
            })
        )
        .expect("Failed to query row");
    
        Ok(info)
    }

    pub fn domains_len(&self) -> SQLiteResult<i32> {
        match &self.domains_scope {
            Some(domains) => Ok(domains.len() as i32),
            None => self
                .connection
                .query_row(&DOMAINS_AMOUNT, NO_PARAMS, |r| Ok(r.get(0)))
                .expect("select failed"),
        }
    }
}
