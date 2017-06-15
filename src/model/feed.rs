use postgres;
use uuid::Uuid;
use error::Error;
use chrono::{NaiveDateTime, UTC};
use super::{conn, Model, Entry};
use model::state::State;
use rss;

static PROPS: [&'static str; 15]  = ["id",
                                     "url",
                                     "title",
                                     "description",
                                     "language",
                                     "velocity",
                                     "website",
                                     "state",
                                     "last_updated",
                                     "crawled",
                                     "visual_url",
                                     "icon_url",
                                     "cover_url",
                                     "created_at",
                                     "updated_at"];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feed {
    pub id:           Uuid,
    pub url:          String,
    pub title:        String,
    pub description:  Option<String>,
    pub language:     Option<String>,
    pub velocity:     f64,
    pub website:      Option<String>,
    pub state:        State,
    pub last_updated: NaiveDateTime,
    pub crawled:      NaiveDateTime,

    pub visual_url:   Option<String>,
    pub icon_url:     Option<String>,
    pub cover_url:    Option<String>,

    pub created_at:   NaiveDateTime,
    pub updated_at:   NaiveDateTime,
}

impl<'a> Model<'a> for Feed {
    fn table_name() -> String {
        "feeds".to_string()
    }
    fn props_str(prefix: &str) -> String {
        PROPS
            .iter()
            .map(|&p| format!("{}{}", prefix, p))
            .collect::<Vec<String>>().join(",")
    }
    fn rows_to_items(rows: postgres::rows::Rows) -> Vec<Feed> {
        let mut feeds = Vec::new();
        for row in rows.iter() {
            feeds.push(Feed {
                id:           row.get(0),
                url:          row.get(1),
                title:        row.get(2),
                description:  row.get(3),
                language:     row.get(4),
                velocity:     row.get(5),
                website:      row.get(6),
                state:        State::new(row.get(7)),
                last_updated: row.get(8),
                crawled:      row.get(9),

                visual_url:   row.get(10),
                icon_url:     row.get(11),
                cover_url:    row.get(12),

                created_at:   row.get(13),
                updated_at:   row.get(14),
            })
        }
        feeds
    }
    fn create(&self) -> Result<Feed, Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO feeds (id, u) VALUES ($1)"));
        let rows = try!(stmt.query(&[
            &self.id,
            &self.url,
            &self.title,
            &self.description,
            &self.language,
            &self.velocity,
            &self.website,
            &self.state.to_string(),
            &self.last_updated,
            &self.crawled,
            &self.visual_url,
            &self.icon_url,
            &self.cover_url,
            &self.created_at,
            &self.updated_at,
        ]));
        let mut feed = self.clone();
        for row in rows.iter() {
            feed.id = row.get(0);
        }
        Ok(feed)
    }
    fn save(&mut self) -> Result<(), Error> {
        self.updated_at = UTC::now().naive_utc();
        let conn = try!(conn());
        let stmt = try!(conn.prepare("UPDATE feeds SET
                                   url          = $2,
                                   title        = $3,
                                   description  = $4,
                                   language     = $5,
                                   velocity     = $6,
                                   website      = $7,
                                   state        = $8,
                                   last_updated = $9,
                                   crawled      = $10,
                                   visual_url   = $11,
                                   icon_url     = $12,
                                   cover_url    = $13,
                                   created_at   = $14,
                                   updated_at   = $15
                                 WHERE id = $1"));
        let result = stmt.query(&[&self.id,
                                  &self.url,
                                  &self.title,
                                  &self.description,
                                  &self.language,
                                  &self.velocity,
                                  &self.website,
                                  &self.state.to_string(),
                                  &self.last_updated,
                                  &self.crawled,
                                  &self.visual_url,
                                  &self.icon_url,
                                  &self.cover_url,
                                  &self.created_at,
                                  &self.updated_at]);
        try!(result);
        Ok(())
    }
}

impl Feed {
    pub fn find_by_url(url: &str) -> Result<Feed, Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare(
            &format!("SELECT {} FROM feeds
                        WHERE url = $1", Self::props_str(""))));
        let rows = try!(stmt.query(&[&url]));
        let feeds = Feed::rows_to_items(rows);
        if feeds.len() > 0 {
            return Ok(feeds[0].clone());
        }
        return Err(Error::NotFound)
    }

    pub fn find_or_create_by_url(url: String) -> Result<Feed, Error> {
        match Feed::find_by_url(&url) {
            Ok(feed) => Ok(feed),
            Err(_)   => Feed::create_by_url(url)
        }
    }

    pub fn create_by_url(url: String) -> Result<Feed, Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO feeds (url) VALUES ($1) RETURNING id"));
        let rows = try!(stmt.query(&[&url]));
        for row in rows.iter() {
            let feed = Feed {
                id:           row.get(0),
                url:          url,
                title:        "".to_string(),
                description:  None,
                language:     None,
                velocity:     0.0,
                website:      None,
                state:        State::Alive,
                last_updated: UTC::now().naive_utc(),
                crawled:      UTC::now().naive_utc(),
                visual_url:   None,
                icon_url:     None,
                cover_url:    None,
                created_at:   UTC::now().naive_utc(),
                updated_at:   UTC::now().naive_utc(),
            };
            return Ok(feed);
        }
        Err(Error::Unexpected)
    }

    pub fn fetch_props(&mut self) -> Result<(), Error> {
        let rss_feed      = try!(rss::fetch(&self.url));
        let now           = UTC::now().naive_utc();
        self.title        = rss_feed.title.unwrap_or("".to_string());
        self.description  = rss_feed.description;
        self.language     = rss_feed.language;
        self.website      = rss_feed.website;
        self.last_updated = rss_feed.last_updated.unwrap_or(now);
        self.crawled      = now;

        self.visual_url   = rss_feed.visual_url;
        self.icon_url     = rss_feed.icon_url;
        self.cover_url    = rss_feed.cover_url;
        Ok(())
    }

    pub fn crawl(&mut self) -> Result<Vec<Entry>, Error> {
        let rss_feed      = try!(rss::fetch(&self.url));
        let mut entries   = vec![];
        for entry in rss_feed.entries {
            if entry.alternate.len() == 0 {
                continue;
            }
            let alt = entry.alternate.first().unwrap();
            match Entry::find_or_create_by_url_if_invalid(alt.href.to_string()) {
                Ok(mut e) => {
                    println!("Found new entry: {}", e.url);
                    e.update_with_feed_entry(&entry);
                    e.feed_id = Some(self.id);
                    let _ = e.playlistify();
                    if let Ok(_) = e.save() {
                        entries.push(e);
                    }
                },
                Err(_) => (),
            }
        }
        Ok(entries)
    }
}
