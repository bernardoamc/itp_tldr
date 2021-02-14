use rusqlite::{params, Connection, Result as SQLiteResult, Row, NO_PARAMS};

use crate::Config;
extern crate dirs;

const OBSERVED_DOMAINS: &'static str = "SELECT domainID, registrableDomain FROM ObservedDomains";
const SCOPED_DOMAINS: &'static str =
    "SELECT domainID, registrableDomain FROM ObservedDomains WHERE registrableDomain = ?";
const DOMAINS_AMOUNT: &'static str = "SELECT count(*) FROM ObservedDomains";
const DOMAIN_INFO: &'static str = "SELECT isPrevalent, isVeryPrevalent, timesAccessedAsFirstPartyDueToUserInteraction, timesAccessedAsFirstPartyDueToStorageAccessAPI FROM ObservedDomains WHERE domainID = ?";
const IFRAME_DOMAIN_INFO: &'static str =
    "SELECT count(*) FROM SubframeUnderTopFrameDomains WHERE subFrameDomainID = ?";
const SUBRESOURCE_DOMAIN_INFO: &'static str =
    "SELECT count(*) FROM SubresourceUnderTopFrameDomains WHERE subresourceDomainID = ?";
const TOPFRAME_DOMAIN_REDIRECT: &'static str =
    "SELECT count(*) FROM TopFrameUniqueRedirectsTo WHERE toDomainID = ?";

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

#[derive(Default, Debug)]
pub struct DomainInteraction {
    pub iframes: i32,
    pub requests: i32,
    pub redirects: i32,
}

pub struct Database {
    connection: Connection,
    scope: Option<Vec<String>>,
}

impl Database {
    pub fn connect(config: Config) -> SQLiteResult<Self> {
        dbg!(&config.path);
        let connection = Connection::open(config.path.unwrap())?;
        Ok(Database {
            connection,
            scope: config.domains,
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

        match &self.scope {
            Some(scoped_domains) => {
                let domains = scoped_domains
                    .into_iter()
                    .map(|domain| {
                        self.connection
                            .query_row(&SCOPED_DOMAINS, params![domain], map_domains)
                            .expect("ObservedDomains table to exist")
                    })
                    .collect();

                Ok(domains)
            }
            None => {
                let mut stmt = self.connection.prepare(&OBSERVED_DOMAINS)?;
                let domains = stmt
                    .query_map(NO_PARAMS, map_domains)
                    .expect("ObservedDomains table to exist")
                    .filter_map(|d| d.ok())
                    .collect();
                Ok(domains)
            }
        }
    }

    pub fn get_info(&self, domain: &Domain) -> SQLiteResult<Domain> {
        let info = self
            .connection
            .query_row(&DOMAIN_INFO, params![domain.id], |row| {
                Ok(Domain {
                    id: domain.id,
                    name: domain.name.clone(),
                    prevalent: row.get(0)?,
                    very_prevalent: row.get(1)?,
                    first_party_interaction: row.get(2)?,
                    first_party_store_access: row.get(3)?,
                })
            })
            .expect("Failed to query row");

        Ok(info)
    }

    pub fn domains_len(&self) -> SQLiteResult<i32> {
        match &self.scope {
            Some(domains) => Ok(domains.len() as i32),
            None => self
                .connection
                .query_row(&DOMAINS_AMOUNT, NO_PARAMS, |r| Ok(r.get(0)))
                .expect("select failed"),
        }
    }

    pub fn domain_interaction(&self, domain: &Domain) -> DomainInteraction {
        let iframe_count = self.iframed_count(domain);
        let requests_count = self.requests_count(domain);
        let redirects_count = self.redirects_count(domain);

        DomainInteraction {
            iframes: iframe_count,
            requests: requests_count,
            redirects: redirects_count,
        }
    }

    fn iframed_count(&self, domain: &Domain) -> i32 {
        self.connection
            .query_row(&IFRAME_DOMAIN_INFO, params![domain.id], |row| {
                Ok(row.get(0).unwrap_or(0))
            })
            .unwrap_or(0)
    }

    fn requests_count(&self, domain: &Domain) -> i32 {
        self.connection
            .query_row(&SUBRESOURCE_DOMAIN_INFO, params![domain.id], |row| {
                Ok(row.get(0).unwrap_or(0))
            })
            .unwrap_or(0)
    }

    fn redirects_count(&self, domain: &Domain) -> i32 {
        self.connection
            .query_row(&TOPFRAME_DOMAIN_REDIRECT, params![domain.id], |row| {
                Ok(row.get(0).unwrap_or(0))
            })
            .unwrap_or(0)
    }
}
