use chrono::Local;
use chrono::DateTime;

use encoding_rs;
use ini::Ini;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use md5;
use regex::Regex;

// use sqlx::mysql::MySqlPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
// use sqlx::Row;
use sqlx::FromRow;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime,UNIX_EPOCH};

use serde::Serialize;

mod rss;
// mod feed;

#[derive(Debug)]
struct Feed {
    id: u32,
    itunes_id: Option<u32>,
    podcast_guid: String,
    url: String,
    content_length: usize,
    feed_type: u8,
    language: String,
    last_item_update_time: i64,
    newest_item_pubdate: i64,
    oldest_item_pubdate: i64,
    item_count: u64,
    update_frequency: u8,
    item_url_strings: String,
    chash: String,
    pubsub: Option<PubSubLinks>,
    podcast_chapters: String,
    podcast_locked: u8,
    podcast_owner: String,
    item_content: String,
    item_content_hash: String,
    old_item_content_hash: String,
    categories: HashSet<String>,

    title: String,
    link: String,
    generator: String,
    pub_date: String,
    last_build_date: String,
    itunes_type: String,
    itunes_category: Vec<rss::RssItunesCategory>,
    itunes_new_feed_url: String,
    explicit: u8,
    itunes_author: String,
    itunes_owner_email: String,
    itunes_owner_name: String,
    itunes_image: String,
    image: String,
    description: String,
    last_update: i64,

    podcast_value: Option<FeedValue>,
    podcast_funding: Option<PodcastFunding>,

    items: Vec<FeedItem>,
}

#[derive(Debug)]
struct FeedItem {
    title: String,
    link: String,
    pub_date: i64,
    image: String,
    itunes_duration: u32,
    itunes_episode: Option<u32>,
    itunes_episode_type: String,
    itunes_season: Option<i64>,
    itunes_image: String,
    itunes_explicit: u8,
    enclosure: Option<FeedItemEnclosure>,
    guid: String,
    description: String,
    podcast_value: Option<FeedValue>,
    podcast_transcripts: Vec<FeedItemTranscript>,
    podcast_chapters: Option<FeedItemChapters>,
    podcast_soundbites: Vec<FeedItemSoundbite>,
    podcast_persons: Vec<FeedItemPerson>,
}

#[derive(Debug)]
struct FeedItemEnclosure {
    url: String,
    length: u32,
    enclosure_type: String,
}

#[derive(Debug)]
struct FeedItemTranscript {
    url: String,
    transcript_type: u8,
}

#[derive(Debug)]
struct FeedItemChapters {
    url: String,
    chapter_type: u8,
}

#[derive(Debug)]
struct FeedItemSoundbite {
    start_time: String,
    duration: String,
    title: String,
}

#[derive(Debug)]
struct FeedItemPerson {
    name: String,
    role: String,
    group: String,
    img: String,
    href: String,
}

#[derive(Debug, Serialize)]
struct FeedValue {
    model: ValueModel,
    destinations: Vec<ValueRecipient>,
}

#[derive(Default, Debug, FromRow)]
#[sqlx(default)]
struct DbFeed {
    id: u32,
    title: Option<String>,
    url: String,
    content: String,
    newest_item_pubdate: Option<i64>,
    update_frequency: Option<u8>,
    podcast_owner: Option<String>,
    parsenow: u8,
    apple_itunes_id: Option<u32>,
    itunes_id: Option<u32>,
    chash: Option<String>,
    item_count: u64,
    podcastguid: Option<String>,
    item_content_hash: Option<String>,
    #[sqlx(rename = "type")]
    feed_type: Option<u8>,
}

#[derive(Debug, Serialize)]
struct ValueModel {
    value_type: String,
    method: String,
    suggested: String,
}

#[derive(Debug, Serialize)]
struct ValueRecipient {
    name: String,
    recipient_type: String,
    address: String,
    split: f64,
    custom_key: u64,
    custom_value: String,
    fee: bool,
}

#[derive(Debug)]
struct PodcastFunding {
    url: String,
    message: String,
}

struct LoggingConfig {
    log_folder: String,
    log_errors_only: u8,
    acclog: String,
    errlog: String,
    dbglog: String,
}

#[tokio::main]
async fn main() {
    //Globals
    let netcalls = 0;
    let mut dbcalls = 0;
    let dbcheck = 0;
    let query = 0;
    let netwait = 240;
    let mut feedcount = 0;
    let mut force = false;
    let max_rows_to_return = 300;
    let max_content_length = 25000000;

    let time_started = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let time_started: i64 = time_started.try_into().unwrap();

    // let time_400_days_ago = time_started - (86400 * 400);
    // let time_200_days_ago = time_started - (86400 * 200);
    // let time_100_days_ago = time_started - (86400 * 100);
    // let time_70_days_ago = time_started - (86400 * 70);
    // let time_40_days_ago = time_started - (86400 * 40);
    // let time_20_days_ago = time_started - (86400 * 20);
    // let time_10_days_ago = time_started - (86400 * 10);
    // let time_5_days_ago = time_started - (86400 * 5);
    let mut still_waiting_for_db = true;
    // let waiting_for_db_count = 240;
    let mut feed_work_count = 0;
    let mut total_items_added = 0;
    // let stmt_pre_catmap = "INSERT INTO nfcategories (feedid, catid1, catid2, catid3, catid4, catid5, catid6, catid7, catid8, catid9, catid10) VALUES ";
    // let stmt_post_catmap = " ON DUPLICATE KEY UPDATE catid1 = VALUES(catid1),catid2 = VALUES(catid2),catid3 = VALUES(catid3),catid4 = VALUES(catid4),catid5 = VALUES(catid5),catid6 = VALUES(catid6),catid7 = VALUES(catid7),catid8 = VALUES(catid8),catid9 = VALUES(catid9),catid10 = VALUES(catid10) ";
    // let sql_statement_catmap = "".to_string();
    let mut inserts_catmap = "".to_string();
    // let stmt_pre_pubsub = "INSERT INTO pubsub (feedid, hub_url, self_url) VALUES ";
    // let stmt_post_pubsub = " ON DUPLICATE KEY UPDATE hub_url = VALUES(hub_url),self_url = VALUES(self_url) ";
    let mut inserts_pubsub = "".to_string();
    let mut inserts_pubsub_bind: Vec<String> = vec![];
    // let stmt_pre_value = "INSERT INTO nfvalue (feedid, value_block, type, createdon) VALUES ";
    // let stmt_post_value = " ON DUPLICATE KEY UPDATE value_block = VALUES(value_block), type = VALUES(type) ";
    // let inserts_value = "".to_string();
    // let inserts_value_bind: Vec<String> = vec![];
    // let stmt_pre_chapters = "INSERT INTO nfitem_chapters (itemid, url, type) VALUES ";
    // let stmt_post_chapters = " ON DUPLICATE KEY UPDATE url = VALUES(url), type = VALUES(type) ";
    let mut inserts_chapters = "".to_string();
    let mut inserts_chapters_bind: Vec<String> = vec![];
    // let stmt_pre_transcripts = "INSERT INTO nfitem_transcripts (itemid, url, type) VALUES ";
    // let stmt_post_transcripts = " ON DUPLICATE KEY UPDATE url = VALUES(url), type = VALUES(type) ";
    let mut inserts_transcripts = "".to_string();
    let mut inserts_transcripts_bind: Vec<String> = vec![];
    // let stmt_pre_funding = "INSERT INTO nffunding (feedid, url, message) VALUES ";
    // let stmt_post_funding = " ON DUPLICATE KEY UPDATE url = VALUES(url), message = VALUES(message) ";
    // let inserts_funding = "".to_string();
    // let inserts_funding_bind: Vec<String> = vec![];
    // let stmt_pre_soundbites = "INSERT INTO nfitem_soundbites (itemid, title, start_time, duration) VALUES ";
    // let stmt_post_soundbites = " ON DUPLICATE KEY UPDATE title = VALUES(title) ";
    let mut inserts_soundbites = "".to_string();
    let mut inserts_soundbites_bind: Vec<String> = vec![];
    // let stmt_pre_persons = "INSERT INTO nfitem_persons (itemid, name, role, grp, img, href) VALUES ";
    // let stmt_post_persons = " ON DUPLICATE KEY UPDATE name = VALUES(name), role = VALUES(role), grp = VALUES(grp), img = VALUES(img), href = VALUES(href) ";
    let mut inserts_persons = "".to_string();
    let mut inserts_persons_bind: Vec<String> = vec![];
    // let stmt_pre_guid = "INSERT INTO nfguids (feedid, guid) VALUES ";
    // let stmt_post_guid = " ON DUPLICATE KEY UPDATE guid = VALUES(guid) ";
    // let inserts_guid = "".to_string();
    // let inserts_guid_bind: Vec<String> = vec![];
    // let stmt_pre_value_item = "INSERT INTO nfitem_value (itemid, value_block, type, createdon) VALUES ";
    // let stmt_post_value_item = " ON DUPLICATE KEY UPDATE value_block = VALUES(value_block), type = VALUES(type) ";
    let mut inserts_value_item = "".to_string();
    let mut inserts_value_item_bind: Vec<String> = vec![];


    let mut checkall = false;
    let mut checkone = false;
    let mut checkdead = false;
    let mut checkerror = false;
    let mut ckoneurl = "".to_string();

    //Get command line args
    for (i, val) in env::args().enumerate() {
        println!("{}: [{}]", i, val);

        if i >= 1 && val == "checkall" {
            println!("Checking all feeds.");
            checkall = true;
        }
        if i >= 1 && val == "checkdead" {
            println!("Checking dead feeds.");
            checkall = true;
        }
        if i >= 1 && val == "checkerror" {
            println!("Checking high error feeds.");
            checkerror = true;
        }
        if i >= 1 && val == "force" {
            println!("Ignoring last-modified.");
            force = true;
        }
        if !checkall && i >= 1 && val.contains("http") {
            println!("Checking feed: [{}]", val);
            ckoneurl = val;
            checkone = true;
        }
    }

    //Get the database and table info
    let config_content = fs::read_to_string("./global.conf").unwrap();
    let config = Ini::load_from_str(&config_content).unwrap();

    let folders = config.section(Some("folders")).unwrap();
    let logging = config.section(Some("logging")).unwrap();

    let mut cfg = LoggingConfig {
        log_folder: folders.get("cg_log").unwrap_or_default().to_string(),
        log_errors_only: logging.get("log_errors_only")
            .and_then(|val| val.parse::<u8>().ok())
            .unwrap_or_default(),
        acclog: logging.get("cg_acclog").unwrap_or_default().to_string(),
        errlog: logging.get("cg_errlog").unwrap_or_default().to_string(),
        dbglog: logging.get("cg_dbglog").unwrap_or_default().to_string(),
    };

    //println!(config.database);
    loggit(&cfg, 3, "DEBUG: It's party time!");
    println!("Connecting to database...");

    //Get a connection to mysql
    // Replace with your MySQL connection details
    let database = config.section(Some("database")).unwrap();

    // let database_url = format!(
    //     "mysql://{}:{}@{}:3306/{}",
    //     database.get("cg_dbuser").unwrap_or_default(),
    //     database.get("cg_dbpass").unwrap_or_default(),
    //     database.get("cg_dbhost").unwrap_or_default(),
    //     database.get("cg_dbname").unwrap_or_default(),
    //     //charset: "utf8mb4"
    // );

    // let pool = MySqlPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&database_url)
    //     .await
    //     .unwrap();


    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:///tmp/rowdytime.db")
        // .connect(&database_url)
        .await
        .unwrap();

    let folders = config.section(Some("folders")).unwrap();
    let cg_feeds = folders.get("cg_feeds").unwrap();

    let tables = config.section(Some("tables")).unwrap();
    let cg_table_newsfeeds = tables.get("cg_table_newsfeeds").unwrap();
    let cg_table_newsfeed_items = tables.get("cg_table_newsfeed_items").unwrap();

    let partytime = config.section(Some("partytime")).unwrap();
    let cg_partytime_hostid = partytime.get("cg_partytime_hostid").unwrap();

    println!("Done");

    //Timestamp for one month ago
    let monthago = (SystemTime::now() - std::time::Duration::from_secs(28 * 86400)).duration_since(UNIX_EPOCH).unwrap().as_secs();

    //Assemble query
    //Get all of the rows marked as updated, but make sure they have actual content
    let mut query = format!(
        "SELECT
            feeds.id,
            feeds.title,
            feeds.url,
            feeds.content,
            feeds.newest_item_pubdate,
            feeds.update_frequency,
            feeds.podcast_owner,
            feeds.parsenow,
            MIN(apple.itunes_id) AS apple_itunes_id,
            feeds.itunes_id AS itunes_id,
            feeds.chash,
            COUNT(nfitems.id) AS item_count,
            guids.guid AS podcastguid,
            feeds.podcast_chapters AS item_content_hash
        FROM {} AS feeds
        LEFT JOIN directory_apple AS apple ON feeds.url = apple.feed_url
        LEFT JOIN nfitems AS nfitems ON feeds.id = nfitems.feedid
        LEFT JOIN nfguids AS guids ON feeds.id = guids.feedid
        WHERE feeds.updated='{}'
        GROUP BY feeds.id
        ORDER BY feeds.parsenow DESC, feeds.lastcheck ASC
        LIMIT {}",
        cg_table_newsfeeds,
        cg_partytime_hostid,
        max_rows_to_return
    );

    if checkone {
        query = format!(
            "SELECT
                feeds.id,
                feeds.url,
                feeds.content,
                apple.itunes_id
            FROM {} AS feeds
            LEFT JOIN directory_apple AS apple ON feeds.url = apple.feed_url
            WHERE feeds.url = '{}'
            ORDER BY feeds.id ASC LIMIT {}",
            cg_table_newsfeeds,
            ckoneurl,
            max_rows_to_return
        );
    }

    println!("QUERY: [{}]", query);

    //Pull the feed list
    dbcalls += 1;

    let rows: Vec<DbFeed> = sqlx::query_as::<_, DbFeed>(&query)
        .fetch_all(&pool)
        .await
        .unwrap();

    still_waiting_for_db = false;

    loggit(&cfg, 3, &format!("Pulled [{}] feed bodies to parse...", rows.len()));
    println!("Pulled [{}] feed bodies to parse...", rows.len());

    //println!(rows);
    if rows.len() < 1 && checkone {
        println!("Couldn't find feed: [{}] in the database.", ckoneurl);
    }

    //Iterate through all the returned feeds and parse each one"s content to search for feed items and enclosures
    for mut feed in rows {
        let mut error_encountered = false;
        let feed_unparseable = false;

        feedcount += 1;

        println!("{}", feed.parsenow);

        //Call out feeds marked for immediate processing
        if feed.parsenow > 0 {
            println!("{} PARSENOW: [{} | {}]", "\x1b[33m%s\x1b[0m", feed.id, feed.url);
        }

        // let options = {
        //     attributeNamePrefix: "@_",
        //     attrNodeName: "attr", //default is "false"
        //     textNodeName: "#text",
        //     ignoreAttributes: false,
        //     ignoreNameSpace: false,
        //     allowBooleanAttributes: false,
        //     parseNodeValue: true,
        //     parseAttributeValue: false,
        //     trimValues: true,
        //     //cdataTagName: "__cdata", //default is "false"
        //     //cdataPositionChar: "\\c",
        //     parseTrueNumberOnly: false,
        //     arrayMode: false, //"strict"
        //     attrValueProcessor: (val, attrName) => he.decode(val, {isAttributeValue: true}), //default is a=>a
        //     tagValueProcessor: (val, tagName) => he.decode(val), //default is a=>a
        //     stopNodes: ["parse-me-as-string"]
        // };

        //Create the feed object
        let mut feed_obj = Feed {
            id: feed.id,
            itunes_id: feed.itunes_id,
            podcast_guid: feed.podcastguid.clone().unwrap_or_default(),
            url: feed.url.clone(),
            content_length: feed.content.len(),
            feed_type: 0,
            language: "en".to_string(),
            last_item_update_time: feed.newest_item_pubdate.unwrap_or_default(),
            newest_item_pubdate: 0,
            oldest_item_pubdate: 0,
            item_count: 0,
            update_frequency: feed.update_frequency.unwrap_or_default(),
            item_url_strings: "".to_string(),
            chash: "".to_string(),
            pubsub: None,
            podcast_chapters: "".to_string(),
            podcast_locked: 0,
            podcast_owner: feed.podcast_owner.clone().unwrap_or_default(),
            item_content: "".to_string(),
            item_content_hash: feed.item_content_hash.clone().unwrap_or_default(),
            old_item_content_hash: feed.item_content_hash.clone().unwrap_or_default(),
            categories: HashSet::new(),

            title: feed.title.clone().unwrap_or_default(),
            link: "".to_string(),
            generator: "".to_string(),
            pub_date: "".to_string(),
            last_build_date: "".to_string(),
            itunes_type:  "".to_string(),
            itunes_category: vec![],
            itunes_new_feed_url:  "".to_string(),
            explicit: 0,
            itunes_author: "".to_string(),
            itunes_owner_email: "".to_string(),
            itunes_owner_name: "".to_string(),
            itunes_image: "".to_string(),
            image: "".to_string(),
            description: "".to_string(),
            last_update: 0,

            podcast_value: None,
            podcast_funding: None,
            items: vec![],
        };

        //Check itunes id
        if feed_obj.itunes_id.is_none() && feed.apple_itunes_id.is_some() {
            feed_obj.itunes_id = feed.apple_itunes_id;
        }

        println!("\x1b[35miTunesID: [{}]\x1b[0m", feed_obj.itunes_id.unwrap_or_default());

        if !feed_file_exists(&cg_feeds, feed.id) {
            println!("Feed file: [{}.txt] does not exist for feed: [{}]. Reverting update flag.", feed.id, feed.url);

            dbcalls += 1;

            let query = format!("UPDATE {} SET updated=0 WHERE id=?", cg_table_newsfeeds);

            let result = sqlx::query(&query)
                .bind(feed_obj.id)
                .execute(&pool)
                .await
                .unwrap();

            if result.rows_affected() == 0 {
                println!("Error updating feed record for feed: [{}]", feed.url);
            }

            dbcalls -= 1;
            continue;
        }

        feed.content = read_feed_file(&cg_feeds, feed.id);
        delete_feed_file(&cg_feeds, feed.id);

        //println!(feed.content);
        let parsed = rss::parse_feed(&feed.content);
        if let Err(err) = &parsed {
            println!("parse error: {:#?}", err)
        }

        if let Ok(the_rss) = parsed {
            if checkone || feed_obj.id == 3506553 {
                println!("{:#?}", the_rss);
            }

            if the_rss.channel.is_none() {
                feed.feed_type = Some(0);
                mark_feed_as_unparseable(&pool, &cg_table_newsfeeds, &feed).await.unwrap();
                continue;
            }

            if let Some(the_channel) = the_rss.channel {
                //Key attributes
                feed_obj.title = the_channel.title.clone().unwrap_or_default();

                if let Some(language) = the_channel.language.get(0) {
                    feed_obj.language = language.clone();
                }

                if let Some(generator) = the_channel.generator.get(0) {
                    feed_obj.generator = generator.clone();
                }

                if let Some(pub_date) = the_channel.pub_date.get(0) {
                    feed_obj.pub_date = pub_date.clone();
                }

                feed_obj.last_build_date = the_channel.last_build_date.clone().unwrap_or_default();

                //Pubsub links?
                feed_obj.pubsub = find_pubsub_links(&the_channel);

                //Clean the title
                feed_obj.title = feed_obj.title.trim().replace("\r\n", "").replace("\n", "").replace("\r", "");

                //Clean the link
                feed_obj.link = feed_obj.link.trim().replace("\r\n", "").replace("\n", "").replace("\r", "");

                //Feed categories
                feed_obj.categories = HashSet::new();

                for item in the_channel.itunes_category {
                    if let Some(ref text) = item.text {
                        let name = text.to_lowercase().replace("&amp;", "");
                        let normalized = name.chars().filter(|c| !c.is_whitespace()).collect();
                        feed_obj.categories.insert(normalized);
                    }

                    for subitem in item.sub_category {
                        if let Some(ref text) = item.text {
                            let subname = text.to_lowercase().replace("&amp;", "");
                            let subnormalized = subname.chars().filter(|c| !c.is_whitespace()).collect();

                            feed_obj.categories.insert(subnormalized);
                        }
                    }
                }

                //Feed owner/author
                feed_obj.itunes_author = "".to_string();

                if let Some(itunes_author) = the_channel.itunes_author.get(0) {
                    feed_obj.itunes_author = itunes_author.clone();
                }

                if let Some(owner) = the_channel.itunes_owner {
                    feed_obj.itunes_owner_email = owner.email.unwrap_or_default();
                    feed_obj.itunes_owner_name = owner.name.unwrap_or_default();
                }

                //Itunes specific stuff
                if let Some(itunes_type) = the_channel.itunes_type.get(0) {
                    if let Some(value) = &itunes_type.value {
                        feed_obj.itunes_type = value.clone();
                    }

                    if let Some(text) = &itunes_type.text {
                        feed_obj.itunes_type = text.clone();
                    }
                }

                if let Some(itunes_new_feed_url) = the_channel.itunes_new_feed_url.get(0) {
                    feed_obj.itunes_new_feed_url = itunes_new_feed_url.clone();
                }

                //Feed image
                // feed_obj.itunes_image = "".to_string();
                if let Some(itunes_image) = the_channel.itunes_image.get(0) {
                    if let Some(url) = &itunes_image.url {
                        feed_obj.itunes_image = url.clone();
                    }
                    if let Some(href) = &itunes_image.href {
                        feed_obj.itunes_image = href.clone();
                    }
                    if let Some(value) = &itunes_image.value {
                        feed_obj.itunes_image = value.clone();
                    }
                }

                feed_obj.itunes_image = sanitize_url(&feed_obj.itunes_image);

                // feed_obj.image = "".to_string();

                if let Some(image) = the_channel.image.get(0) {
                    if let Some(url) = &image.url {
                        feed_obj.image = url.clone();
                    }
                }

                if feed_obj.image.is_empty() && !feed_obj.itunes_image.is_empty() {
                    feed_obj.image = feed_obj.itunes_image.clone();
                }

                feed_obj.image = sanitize_url(&feed_obj.image);

                //Feed explicit content
                // feed_obj.explicit = 0;
                if let Some(itunes_explicit) = the_channel.itunes_explicit.get(0) {
                    let explicit = itunes_explicit.to_lowercase();

                    if explicit == "yes" || explicit == "true" {
                        feed_obj.explicit = 1;
                    }
                }

                //Feed description
                // feed_obj.description = theFeed.rss.channel.description;
                if let Some(itunes_summary) = the_channel.itunes_summary.get(0) {
                    feed_obj.description = itunes_summary.clone();
                }

                //Feed link
                if let Some(link) = the_channel.link.get(0) {
                    if let Some(value) = &link.value {
                        feed_obj.link = value.clone();
                    } else if let Some(href) = &link.href {
                        feed_obj.link = href.clone();
                    } else {
                        if !feed_obj.url.is_empty() {
                            feed_obj.link = feed_obj.url.clone();
                        }
                    }
                }

                //Value block
                //If there are more than one, give priority to the lightning one
                let mut podcast_value = None;

                for (index, item) in the_channel.podcast_value.iter().enumerate() {
                    if let Some(vtype) = &item.value_type {
                        if vtype == "lighting" {
                            podcast_value = Some(item);
                            break;
                        }
                    }
                }

                //Now parse the value block
                if let Some(podvalue) = podcast_value {
                    println!("{:#?}", podvalue);

                    //Get the model
                    let model = ValueModel {
                        value_type: podvalue.value_type.clone().unwrap_or_default(),
                        method: podvalue.method.clone().unwrap_or_default(),
                        suggested: podvalue.suggested.clone().unwrap_or_default(),
                    };

                    //Get the recipients
                    let mut destinations = vec![];

                    for recipient in &podvalue.value_recipients {
                        let fee_text = recipient.fee.clone().unwrap_or_default().to_lowercase();
                        let fee = if fee_text == "true" || fee_text == "yes" {
                            true
                        } else {
                            false
                        };

                        destinations.push(ValueRecipient {
                            name: recipient.name.clone().unwrap_or_default(),
                            recipient_type: recipient.recipient_type.clone().unwrap_or_default(),
                            address: recipient.address.clone().unwrap_or_default(),
                            split: recipient.split.clone().unwrap_or_default(),
                            custom_key: recipient.custom_key.clone().unwrap_or_default(),
                            custom_value: recipient.custom_value.clone().unwrap_or_default(),
                            fee: fee,
                        });
                    }

                    //Get value block type
                    let this_value_block_type = match model.value_type.as_str() {
                        "HBD" => 1,
                        "bitcoin" => 2,
                        _ => 0,
                    };

                    feed_obj.podcast_value = Some(FeedValue {
                        model,
                        destinations,
                    });

                    println!("{:#?}", feed_obj.podcast_value);
                    // insertsValue.push_str(" (?,?,?,?),");
                    // insertsValueBind.push(feed_obj.id);
                    // insertsValueBind.push(JSON.stringify(feed_obj.podcast_value));
                    // insertsValueBind.push(this_value_block_type);
                    // insertsValueBind.push((chrono::Utc::now().timestamp()) as i64);
                }

                //Locked?
                if let Some(podcast_locked) = the_channel.podcast_locked {
                    let locked = podcast_locked.value.unwrap_or_default().to_lowercase();

                    if locked == "yes" || locked == "true" {
                        feed_obj.podcast_locked = 1;
                    }

                    let owner = podcast_locked.owner.unwrap_or_default();

                    if owner != "" {
                        feed_obj.podcast_owner = owner;
                    }

                    let email = podcast_locked.email.unwrap_or_default();

                    if email != "" {
                        feed_obj.podcast_owner = email;
                    }

                    let lock_log = format!("LOCKED: {}[{}] - {}", feed_obj.podcast_owner, feed_obj.podcast_locked, &feed_obj.url);

                    println!("\x1b[33m{}\x1b[0m", lock_log);
                }

                if feed_obj.podcast_owner.is_empty() && !feed_obj.itunes_owner_email.is_empty() {
                    println!("\x1b[33m{}\x1b[0m - OWNER EMAIL OVERRIDE: [{}|{}]", feed_obj.id, feed_obj.podcast_owner, feed_obj.itunes_owner_email);
                    feed_obj.podcast_owner = feed_obj.itunes_owner_email.clone();
                }

                //Funding
                if let Some(podcast_funding) = the_channel.podcast_funding.get(0) {
                    let podcast_funding = &podcast_funding;
                    let mut funding_message = "".to_string();

                    if let Some(message) = &podcast_funding.value {
                        if message != "" {
                            funding_message = message.clone();
                        }
                    }

                    if let Some(url) = &podcast_funding.url {
                        if url != "" {
                            feed_obj.podcast_funding = Some(PodcastFunding {
                                message: funding_message,
                                url: url.clone(),
                            });
                        }
                    }

                    if feed_obj.podcast_funding.is_some() {
                        println!("{:#?}", feed_obj.podcast_funding);
                        // insertsFunding.push_str(" (?,?,?),");
                        // insertsFundingBind.push(feed_obj.id);
                        // insertsFundingBind.push(feed_obj.podcastFunding.url);
                        // insertsFundingBind.push(feed_obj.podcastFunding.message);
                    }
                }

                //GUID
                if let Some(podcast_guid) = the_channel.podcast_guid.get(0) {
                    if !podcast_guid.is_empty() {
                        feed_obj.podcast_guid = the_channel.podcast_guid[0].clone();

                        println!("\x1b[34mGUID: {}\x1b[0m", feed_obj.podcast_guid);

                        println!("{:#?}", feed_obj.podcast_guid);
                        // insertsGUID.push_str(" (?,?),");
                        // insertsGUIDBind.push(feed_obj.id);
                        // insertsGUIDBind.push(feed_obj.podcastguid);
                    }
                }

                //println!("DEBUG: {}", the_channel.item);

                //ITEM PARSING! -------------------------------------------------------------------------
                //---------------------------------------------------------------------------------------
                //Items
                let mut i = 0;
                // feed_obj.items = vec![];

                for the_item in the_channel.item {
                    feed_obj.item_count += 1;

                    //If there is more than one enclosure in the item, just get the first one
                    let enclosure = the_item.enclosure.get(0);

                    //If there is no enclosure, just skip this item and move on to the next
                    if enclosure.is_none() {
                        continue;
                    }

                    let enclosure = enclosure.unwrap();

                    if let Some(url) = &enclosure.url {
                        let pos = url.to_lowercase().find("http").unwrap_or_default();

                        if pos != 0 {
                            continue;
                        }
                    } else {
                        //If the enclosure url is not present or sane, skip this item
                        continue;
                    }

                    //Get the GUID if there is one.  If not, use the enclosure url as the GUID.
                    let mut itemguid = "".to_string();

                    if let Some(guid) = the_item.guid {
                        itemguid = guid.clone();
                    }

                    if itemguid.is_empty() {
                        if let Some(url) = &enclosure.url {
                            if url.len() > 10 {
                                itemguid = truncate_string(&url, 738);
                            }
                        }
                    }

                    if itemguid.is_empty() {
                        continue;
                    }

                    let mut item_obj = FeedItem {
                        title: the_item.title.unwrap_or_default(),
                        link: "".to_string(),
                        image: "".to_string(),
                        itunes_duration: 0,
                        itunes_image: "".to_string(),
                        itunes_episode: None,
                        itunes_episode_type: "".to_string(),
                        itunes_season: None,
                        itunes_explicit: 0,
                        enclosure: None,
                        pub_date: 0,
                        guid: itemguid,
                        description: "".to_string(),
                        podcast_value: None,
                        podcast_transcripts: vec![],
                        podcast_chapters: None,
                        podcast_soundbites: vec![],
                        podcast_persons: vec![],
                    };

                    if let Some(pub_date) = the_item.pub_date {
                        item_obj.pub_date = match pub_date_to_timestamp(&pub_date) {
                            Ok(parsed_timestamp) => parsed_timestamp,
                            Err(err) => {
                                println!("Unable to parse pubdate: {}", pub_date);
                                0
                            },
                        }
                    }

                    // feed_obj.item_content.push_str(&item_obj.enclosure.url;
                    if let Some(enclosure) = the_item.enclosure.get(0) {
                        if let Some(url) = &enclosure.url {
                            feed_obj.item_content.push_str(&url);
                        }
                    }

                    if feed_obj.id == 950633 {
                        println!("\x1b[33m  GUID: {}\x1b[0m", item_obj.guid);
                    }

                    //Item title
                    item_obj.title = String::from(item_obj.title.trim());

                    if let Some(itunes_title) = &the_item.itunes_title {
                        if itunes_title != "" {
                            item_obj.title = itunes_title.clone();
                        }
                    }

                    feed_obj.item_content.push_str(&item_obj.title);

                    //Item link
                    if let Some(link) = &the_item.link {
                        if let Some(value) = &link.value {
                            item_obj.link = value.clone();
                        }
                        if let Some(href) = &link.href {
                            item_obj.link = href.clone();
                        }
                    }

                    // if (typeof item_obj.link !== "string") {
                    //     item_obj.link = "";
                    // }

                    feed_obj.item_content.push_str(&item_obj.link);

                    //Item image
                    item_obj.itunes_image = "".to_string();

                    if let Some(itunes_image) = the_item.itunes_image.get(0) {
                        if let Some(url) = &itunes_image.url {
                            item_obj.itunes_image = url.clone();
                        }
                        if let Some(href) = &itunes_image.href {
                            item_obj.itunes_image = href.clone();
                        }
                        if let Some(value) = &itunes_image.value {
                            item_obj.itunes_image = value.clone();
                        }
                    }

                    item_obj.itunes_image = sanitize_url(&item_obj.itunes_image);
                    feed_obj.item_content.push_str(&item_obj.itunes_image);

                    item_obj.image = "".to_string();

                    if let Some(image) = the_item.image.get(0) {
                        if let Some(url) = &image.url {
                            item_obj.image = url.clone();
                        }
                    }

                    if item_obj.image.is_empty() && !item_obj.itunes_image.is_empty() {
                        item_obj.image = item_obj.itunes_image.clone();
                    }

                    item_obj.image = sanitize_url(&item_obj.image);
                    feed_obj.item_content.push_str(&item_obj.image);

                    //Itunes specific stuff
                    if let Some(itunes_explicit) = &the_item.itunes_explicit {
                        let explicit = itunes_explicit.to_lowercase();

                        if explicit == "yes" || explicit == "true" {
                            item_obj.itunes_explicit = 1;
                        }
                    }

                    if let Some(itunes_duration) = &the_item.itunes_duration {
                        item_obj.itunes_duration = time_to_seconds(&itunes_duration);
                    } else {
                        item_obj.itunes_duration = 0;
                    }

                    if let Some(itunes_episode) = &the_item.itunes_episode {
                        let episode: String = itunes_episode.chars()
                            .filter(|c| c.is_digit(10))  // Only keep digits
                            .collect();

                        if episode != "" {
                            if let Ok(episode) = episode.parse::<u32>() {
                                item_obj.itunes_episode = Some(episode);
                            }
                        }
                    }

                    if let Some(itunes_episode_type) = the_item.itunes_episode_type.get(0) {
                        item_obj.itunes_episode_type = itunes_episode_type.clone();
                    }

                    if let Some(itunes_season) = the_item.itunes_season.get(0) {
                        item_obj.itunes_season = Some(*itunes_season);
                    }

                    //Item description
                    if let Some(itunes_summary) = &the_item.itunes_summary {
                        item_obj.description = itunes_summary.clone();
                    }

                    if let Some(content_encoded) = &the_item.content_encoded {
                        item_obj.description = String::from(content_encoded.clone().trim());
                    } else if let Some(description) = &the_item.description {
                        item_obj.description = String::from(description.clone().trim());
                    }

                    //Enclosure
                    if let Some(enclosure) = the_item.enclosure.get(0) {
                        if let (Some(url), Some(length), Some(enc_type)) = (&enclosure.url, &enclosure.length, &enclosure.enclosure_type) {
                            let enclosure_type = if !enc_type.is_empty() {
                                enc_type.clone()
                            } else {
                                guess_enclosure_type(&url).to_string()
                            };

                            item_obj.enclosure = Some(FeedItemEnclosure {
                                url: url.clone(),
                                length: length.clone() as u32,
                                enclosure_type: enclosure_type.clone(),
                            })
                        }
                    }

                    //Transcripts
                    //-----------------------------------------------------------------
                    item_obj.podcast_transcripts = vec![];

                    for transcript in the_item.podcast_transcript {
                        let url = transcript.url.unwrap_or_default();
                        let mime_type = transcript.mime_type.unwrap_or_default();

                        if !url.is_empty() && !mime_type.is_empty() {
                            let transcript_type = match mime_type.as_str() {
                                mime if mime.contains("json") => 1,
                                mime if mime.contains("srt") => 2,
                                mime if mime.contains("vtt") => 3,
                                _ => 0,
                            };

                            feed_obj.item_content.push_str(&url);

                            item_obj.podcast_transcripts.push(FeedItemTranscript {
                                url,
                                transcript_type,
                            });
                        }
                    }

                    if let Some(podcast_chapters) = &the_item.podcast_chapters {
                        if let Some(url) = &podcast_chapters.url {
                            item_obj.podcast_chapters = Some(FeedItemChapters {
                                url: url.clone(),
                                chapter_type: 0,
                            });

                            feed_obj.item_content.push_str(&url);
                        }
                    }

                    //Soundbites
                    //-----------------------------------------------------------------
                    item_obj.podcast_soundbites = vec![];
                    for soundbite in the_item.podcast_soundbite {
                        if let Some(start_time) = &soundbite.start_time {
                            if let Some(duration) = &soundbite.duration {
                                let title = soundbite.value.unwrap_or_default();
                                let title = truncate_string(&title, 500);

                                feed_obj.item_content.push_str(&start_time);
                                feed_obj.item_content.push_str(&duration);
                                feed_obj.item_content.push_str(&title);

                                item_obj.podcast_soundbites.push(FeedItemSoundbite {
                                    start_time: start_time.clone(),
                                    duration: duration.clone(),
                                    title,
                                });
                            }
                        }
                    }

                    //Persons
                    //-----------------------------------------------------------------
                    item_obj.podcast_persons = vec![];

                    for person in the_item.podcast_person {
                        let name = person.value.unwrap_or_default();

                        let mut person_to_add = FeedItemPerson {
                            name: truncate_string(&name, 128),
                            role: "".to_string(),
                            group: "".to_string(),
                            img: "".to_string(),
                            href: "".to_string(),
                        };

                        if let Some(img) = &person.img {
                            person_to_add.img = truncate_string(&img, 768);
                            feed_obj.item_content.push_str(&person_to_add.img);
                        }
                        if let Some(href) = &person.href {
                            person_to_add.href = truncate_string(&href, 768);
                            feed_obj.item_content.push_str(&person_to_add.href);
                        }
                        if let Some(role) = &person.role {
                            person_to_add.role = truncate_string(&role.to_lowercase(), 128);
                            feed_obj.item_content.push_str(&person_to_add.role);

                        }
                        if let Some(group) = &person.group {
                            person_to_add.group = truncate_string(&group.to_lowercase(), 128);
                            feed_obj.item_content.push_str(&person_to_add.group);
                        }

                        item_obj.podcast_persons.push(person_to_add);
                    }

                    //Value block
                    //If there are more than one, give priority to the lightning one
                    let mut podcast_value = None;

                    for (index, item) in the_item.podcast_value.iter().enumerate() {
                        if let Some(vtype) = &item.value_type {
                            if vtype == "lighting" {
                                podcast_value = Some(item);
                                break;
                            }
                        }
                    }

                    //Now parse the value block
                    if let Some(podvalue) = &podcast_value {
                        println!("{:#?}", podvalue);

                        //Get the model
                        let model = ValueModel {
                            value_type: podvalue.value_type.clone().unwrap_or_default(),
                            method: podvalue.method.clone().unwrap_or_default(),
                            suggested: podvalue.suggested.clone().unwrap_or_default(),
                        };

                        //Get the recipients
                        let mut destinations = vec![];

                        for recipient in &podvalue.value_recipients {
                            let fee_text = recipient.fee.clone().unwrap_or_default().to_lowercase();
                            let fee = if fee_text == "true" || fee_text == "yes" {
                                true
                            } else {
                                false
                            };

                            let value_block = ValueRecipient {
                                name: recipient.name.clone().unwrap_or_default(),
                                recipient_type: recipient.recipient_type.clone().unwrap_or_default(),
                                address: recipient.address.clone().unwrap_or_default(),
                                split: recipient.split.clone().unwrap_or_default(),
                                custom_key: recipient.custom_key.clone().unwrap_or_default(),
                                custom_value: recipient.custom_value.clone().unwrap_or_default(),
                                fee: fee,
                            };

                            //Item content tracking
                            feed_obj.item_content.push_str(&value_block.name);
                            feed_obj.item_content.push_str(&value_block.recipient_type);
                            feed_obj.item_content.push_str(&value_block.address);
                            feed_obj.item_content.push_str(&value_block.split.to_string());
                            feed_obj.item_content.push_str(&value_block.custom_key.to_string());
                            feed_obj.item_content.push_str(&value_block.custom_value);
                            feed_obj.item_content.push_str(&value_block.fee.to_string());

                            destinations.push(value_block);
                        }

                        item_obj.podcast_value = Some(FeedValue {
                            model,
                            destinations,
                        });
                    }

                    feed_obj.items.push(item_obj);

                    //DEBUG
                    feed_obj.item_content_hash = format!("{:x}", md5::compute(&feed_obj.item_content));
                    println!("\x1b[33m  item_content: {} | {}\x1b[0m", feed_obj.item_content_hash, feed_obj.old_item_content_hash);

                    //Get the pub_date of the most recent item
                    let mut most_recent_pub_date = 0;

                    for item in &feed_obj.items {
                        // let this_pub_date = pub_date_to_timestamp(item.pub_date);
                        let this_pub_date = item.pub_date;

                        if this_pub_date > most_recent_pub_date && this_pub_date <= time_started {
                            most_recent_pub_date = this_pub_date;
                        }

                        if checkone {
                            // println!("{}: {}", item.pub_date, pub_date_to_timestamp(item.pub_date));
                            // println!("{}", item.pub_date);
                        }
                    }

                    feed_obj.newest_item_pubdate = most_recent_pub_date;

                    //Get the pub_date of the oldest item
                    let mut oldest_pub_date = most_recent_pub_date;

                    for item in &feed_obj.items {
                        // let this_pub_date = pub_date_to_timestamp(item.pub_date);
                        let this_pub_date = item.pub_date;

                        if this_pub_date < oldest_pub_date && this_pub_date > 0 {
                            oldest_pub_date = this_pub_date;
                        }

                        if checkone {
                            // println!("{}: {}", item.pub_date, pub_date_to_timestamp(item.pub_date));
                            // println!("{}", item.pub_date);
                        }
                    }

                    feed_obj.oldest_item_pubdate = oldest_pub_date;
                }

                if checkone {
                    println!("pub_date: {}", feed_obj.pub_date);
                }

                //Make sure we have a valid pub_date if possible
                // if (feed_obj.pub_date.is_empty() || feed_obj.pub_date == 0 || isNaN(feed_obj.pub_date)) {
                //     if (typeof feed_obj.last_build_date !== "string") {
                //         feed_obj.pub_date = 0;
                //     } else {
                //         feed_obj.pub_date = feed_obj.last_build_date;
                //     }
                // }

                // if (typeof feed_obj.pub_date == "string") {
                //     feed_obj.pub_date = pub_date_to_timestamp(feed_obj.pub_date);
                // }

                // if (typeof feed_obj.newest_item_pubdate == "number") {
                //     if (typeof feed_obj.pub_date !== "number" || feed_obj.pub_date == 0) {
                //         feed_obj.pub_date = feed_obj.newest_item_pubdate;
                //     }
                // }
            }
            // else if () {} // atom format
            else {
                feed.feed_type = Some(9);
                println!("NO CHANNEL OBJECT");
                mark_feed_as_unparseable(&pool, &cg_table_newsfeeds, &feed).await.unwrap();
                continue;
            }

            if checkone {
                //println!(theFeed.rss.channel.item);
            }



            //Get the last 15 characters of each item title, of the first 20 items, building a long string
            //to then use as part of the identity hash
            feed_obj.item_url_strings = "".to_string();
            // feed_obj.items.forEach(function (item, index, array) {
            //     let itemTitle = truncate_string(item.title, 15);
            //     if (index < 20) {
            //         if(itemTitle !== "") {
            //             feed_obj.item_url_strings = feed_obj.item_url_strings + itemTitle;
            //         }
            //     }
            // });

            //Create a hash from some key, stable info in the feed
            let mut feed_hash = "".to_string();
            feed_hash.push_str(&feed_obj.title);
            feed_hash.push_str(&feed_obj.link);
            feed_hash.push_str(&feed_obj.language);
            feed_hash.push_str(&feed_obj.generator);
            feed_hash.push_str(&feed_obj.itunes_author);
            feed_hash.push_str(&feed_obj.itunes_owner_name);
            feed_hash.push_str(&feed_obj.itunes_owner_email);

            let feed_hash = format!("{:x}", md5::compute(&feed_hash));

            if feed_obj.id == 312849 || feed_obj.id == 1330254 {
                println!(
                    "[{}] {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                    feed_hash,
                    feed_obj.title,
                    feed_obj.link,
                    feed_obj.language,
                    feed_obj.generator,
                    feed_obj.itunes_author,
                    feed_obj.itunes_owner_name,
                    feed_obj.itunes_owner_email,
                    feed_obj.item_url_strings
                );
            }

            //Calculate an update_frequency value
            let mut item_times = vec![];
            for item in &feed_obj.items {
                item_times.push(item.pub_date);

                // if(typeof item.pub_date == "number") {
                //     item_times.push(item.pub_date);
                // } else {
                //     println!('\x1b[33m%s\x1b[0m', '  UPDATE_FREQUENCY_ERROR[Bad Item pub_date]: ' + feed_obj.update_frequency);
                // }
            }

            feed_obj.update_frequency = calculate_update_frequency(item_times);
            println!("\x1b[33m  UPDATE_FREQUENCY: {}\x1b[0m", feed_obj.update_frequency);

            //Add the items and enclosures we found if there was a newer feed item
            //discovered
            if feed_obj.newest_item_pubdate != feed_obj.last_item_update_time || feed.item_count == 0 || feed.parsenow > 0 {
                println!(
                    "[{}|{}|{}] Adding [{}] items...",
                    feed.id,
                    feed.title.unwrap_or_default(),
                    feed.item_count,
                    feed_obj.items.len(),
                );

                //Purge all of the old items first
                if feed.parsenow == 2 {
                    // dbcalls += 1;
                    let query = format!("UPDATE {} SET `purge`=? WHERE feedid=?", cg_table_newsfeed_items);

                    let result = sqlx::query(&query)
                        .bind(cg_partytime_hostid)
                        .bind(feed.id)
                        .execute(&pool)
                        .await
                        .unwrap();

                    if result.rows_affected() == 0 {
                        println!("Error purging items: [{}]", feed.url);
                    } else {
                        println!("Done purging items.");
                    }
                }

                //--------------------------------------------------------------------------
                //-----------------ITEM PROCESSING INTO DB----------------------------------
                feed_obj.item_count = 0;
                for item in &feed_obj.items {
                    let enclosure_url = match &item.enclosure {
                        Some(enclosure) => sanitize_url(&enclosure.url),
                        None => "".to_string(),
                    };

                    let enclosure_length = match &item.enclosure {
                        Some(enclosure) => enclosure.length,
                        None => 0,
                    };

                    let enclosure_type = match &item.enclosure {
                        Some(enclosure) => &enclosure.enclosure_type,
                        None => &"".to_string(),
                    };

                    let re = Regex::new(r"&amp;").unwrap();
                    let enclosure_url = re.replace_all(&enclosure_url, "&").to_string();

                    if feed_obj.id == 950633 {
                        //println!(item.guid);
                    }

                    //Don't add an item if the enclosure url is not valid
                    //TODO: Is this the right way to handle?
                    if !enclosure_url.starts_with("http") {
                        if feed_obj.id == 950633 {
                            //println!(index + ". skipped...")
                        }
                        continue;
                    }

                    let mut itunes_episode = item.itunes_episode;

                    if itunes_episode.unwrap_or_default() > 1000000 {
                        itunes_episode = Some(1000000);
                    }

                    // if item.enclosure.len() > 922337203685477580 {
                    //     item.enclosure.length = 0; // ???
                    // }

                    //Set a time in the feed obj to use as the "last_update" time
                    feed_obj.last_update = (chrono::Utc::now().timestamp()) as i64;

                    feed_obj.item_count += 1;

                    if feed_obj.id == 950633 {
                        //println!(index + ". doing insert...")
                    }

                    //Assemble SQL
                    let replaceable = match feed.parsenow {
                        2 => "OR REPLACE",
                        _ => "",
                    };

                    let mut sql_item_insert = format!(
                        "INSERT {} INTO {} (
                            feedid,
                            title,
                            link,
                            description,
                            guid,
                            timestamp,
                            timeadded,
                            enclosure_url,
                            enclosure_length,
                            enclosure_type,
                            itunes_episode,
                            itunes_episode_type,
                            itunes_explicit,
                            itunes_duration,
                            itunes_season,
                            purge,
                            image
                        )
                        VALUES (
                            ?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,0,?
                        )",
                        replaceable,
                        cg_table_newsfeed_items
                    );
println!("{} {:#?}", sql_item_insert, &item);
                    let current_timestamp = chrono::Utc::now().timestamp();

                    let result = sqlx::query(&sql_item_insert)
                        .bind(feed_obj.id)
                        .bind(truncate_string(&item.title, 1024))
                        .bind(sanitize_url(&item.link))
                        .bind(&item.description)
                        .bind(truncate_string(&item.guid, 740))
                        .bind(current_timestamp)
                        .bind(item.pub_date)
                        .bind(enclosure_url)
                        .bind(enclosure_length)
                        .bind(truncate_string(&enclosure_type, 128))
                        .bind(itunes_episode)
                        .bind(&item.itunes_episode_type)
                        .bind(item.itunes_explicit)
                        .bind(item.itunes_duration)
                        .bind(truncate_int(item.itunes_season.unwrap_or_default()))
                        .bind(&item.image)
                        .execute(&pool)
                        .await;
                    //     .unwrap();

                    // if result.rows_affected() == 0 {
                    //     println!("Error purging items: [{}]", feed.url);
                    // } else {
                    //     println!("Done purging items.");
                    // }


                    // dbcalls += 1;
                    //println!("Adding item: ["+item.title+"|"+item.enclosure.url+"] to the database.");
                    if let Err(err) = &result {
                        println!("error: {}", err);
                        // if (err && err.code != 'ER_DUP_ENTRY') {
                        error_encountered = true;
                        // }
                    }

                    if let Ok(result) = &result {
                        if result.rows_affected() > 0 {
                            //println!(result.affectedRows);
                            total_items_added += result.rows_affected();
                        }

                        //Get the inserted item id
                        let item_id = result.last_insert_rowid();

                        //Transcripts
                        if !item.podcast_transcripts.is_empty() {
                            println!(">-----------------");
                            println!("{:#?}", item.podcast_transcripts);
                            println!(">-----------------");
                        }

                        for transcript in &item.podcast_transcripts {
                            println!("{} - TRANSCRIPT", item_id);
                            inserts_transcripts.push_str(" (?,?,?),");
                            inserts_transcripts_bind.push(item_id.to_string());
                            inserts_transcripts_bind.push(transcript.url.clone());
                            inserts_transcripts_bind.push(transcript.transcript_type.to_string());
                        }

                        //Chapters
                        if let Some(chapters) = &item.podcast_chapters {
                            println!("{} - CHAPTER", item_id);
                            inserts_chapters.push_str(" (?,?,?),");
                            inserts_chapters_bind.push(item_id.to_string());
                            inserts_chapters_bind.push(chapters.url.clone());
                            inserts_chapters_bind.push(chapters.chapter_type.to_string());
                        }

                        //Soundbites
                        if !item.podcast_soundbites.is_empty() {
                            println!(">-----------------");
                            println!("{:#?}", item.podcast_soundbites);
                            println!(">-----------------");
                        }

                        for soundbite in &item.podcast_soundbites {
                            println!("{} - SOUNDBITE", item_id);
                            inserts_soundbites.push_str(" (?,?,?,?),");
                            inserts_soundbites_bind.push(item_id.to_string());
                            inserts_soundbites_bind.push(soundbite.title.clone());
                            inserts_soundbites_bind.push(soundbite.start_time.clone());
                            inserts_soundbites_bind.push(soundbite.duration.clone());
                        }

                        //Persons
                        for person in &item.podcast_persons {
                            println!("{} - PERSON", item_id);
                            inserts_persons.push_str(" (?,?,?,?,?,?),");
                            inserts_persons_bind.push(item_id.to_string());
                            inserts_persons_bind.push(person.name.clone());
                            inserts_persons_bind.push(person.role.clone());
                            inserts_persons_bind.push(person.group.clone());
                            inserts_persons_bind.push(person.img.clone());
                            inserts_persons_bind.push(person.href.clone());
                        }

                        //Value
                        if let Some(value) = &item.podcast_value {
                            let this_value_block_type = match value.model.value_type.as_str() {
                                "HBD" => 1,
                                "bitcoin" => 2,
                                _ => 0,
                            };

                            println!("{} - VALUE", item_id);
                            inserts_value_item.push_str(" (?,?,?,?),");
                            inserts_value_item_bind.push(item_id.to_string());
                            inserts_value_item_bind.push(serde_json::to_string(&item.podcast_value).unwrap());
                            inserts_value_item_bind.push(this_value_block_type.to_string());
                            inserts_value_item_bind.push((chrono::Utc::now().timestamp()).to_string());
                        }

                        // dbcalls--;
                    }
                }
                //-----------------ITEM PROCESSING INTO DB----------------------------------
                //--------------------------------------------------------------------------

                //Category updates
                if !feed_obj.categories.is_empty() {
                    if let Some(catmap) = insert_categories(feed_obj.id, feed_obj.categories) {
                        inserts_catmap.push_str(&catmap);
                    }
                }

                //Pubsub updates
                if let Some(pubsub) = feed_obj.pubsub {
                    if pubsub.hub.starts_with("http") && pubsub.self_link.starts_with("http") {
                        //println!("Pubsub: " + feed_obj.id + " - "+pubsub.hub+" -> " + pubsub.self_link);
                        inserts_pubsub.push_str(" (?,?,?),");
                        inserts_pubsub_bind.push(feed_obj.id.to_string());
                        inserts_pubsub_bind.push(pubsub.hub);
                        inserts_pubsub_bind.push(pubsub.self_link);
                    }
                }
            }

            if !ckoneurl.is_empty() {
                // println!(theFeed);
                // println!(feed_obj.image);
                // println!(feed_obj.itunes_image);
                println!("{}", feed_obj.pub_date);
            }

            //Update the feed record with what we discovered
            if !error_encountered {
                // if (feed_obj.newest_item_pubdate != feed_obj.last_item_update_time) {
                //     feed_obj.update_frequency = calculateDays(feed_obj.newest_item_pubdate, feed_obj.last_item_update_time);
                //     //println!(feed_obj.id + ' : ' + feed_obj.newest_item_pubdate + ' - ' + feed_obj.last_item_update_time);
                // }

                //Set a decent timestamp for 'last_update' if one is set in the feed_obj
                let last_update_clause = if feed_obj.last_update > 0 {
                    println!("lastUpdate: {}", feed_obj.last_update);
                    format!(",lastupdate='{}'", feed_obj.last_update)
                } else {
                    "".to_string()
                };

                // dbcalls += 1;
                //println!("Updating feed: ["+feed_obj.id+" | "+feed_obj.url+"] in the database.");

                let mut sql_item_update = format!(
                    "UPDATE {} SET 
                        content='',
                        type=?,
                        generator=?,
                        title=?,
                        link=?,
                        description=?,
                        itunes_author=?,
                        itunes_owner_name=?,
                        itunes_owner_email=?,
                        itunes_new_feed_url=?,
                        explicit=?,
                        image=?,
                        artwork_url_600=?,
                        itunes_type=?,
                        itunes_id=?,
                        parse_errors=0,
                        errors=0,
                        updated=0,
                        lastparse=CURRENT_TIMESTAMP,
                        parsenow=0,
                        newest_item_pubdate=?,
                        update_frequency=?,
                        language=?,
                        chash=?,
                        oldest_item_pubdate=?,
                        item_count=?,
                        podcast_chapters=?,
                        podcast_locked=?,
                        podcast_owner=? 
                        {}
                    WHERE id=?",
                    cg_table_newsfeeds,
                    last_update_clause
                );

                let result = sqlx::query(&sql_item_update)
                    .bind(feed_obj.feed_type)
                    .bind(feed_obj.generator)
                    .bind(truncate_string(&feed_obj.title, 768))
                    .bind(sanitize_url(&feed_obj.link))
                    .bind(feed_obj.description)
                    .bind(feed_obj.itunes_author)
                    .bind(feed_obj.itunes_owner_name)
                    .bind(feed_obj.itunes_owner_email)
                    .bind(sanitize_url(&feed_obj.itunes_new_feed_url))
                    .bind(feed_obj.explicit)
                    .bind(sanitize_url(&feed_obj.image))
                    .bind(sanitize_url(&feed_obj.itunes_image))
                    .bind(feed_obj.itunes_type)
                    .bind(feed_obj.itunes_id)
                    .bind(truncate_int(feed_obj.newest_item_pubdate))
                    .bind(feed_obj.update_frequency)
                    .bind(truncate_string(&feed_obj.language, 8))
                    .bind(feed_hash)
                    .bind(truncate_int(feed_obj.oldest_item_pubdate))
                    .bind(truncate_u64(feed_obj.item_count))
                    .bind(feed_obj.item_content_hash)
                    .bind(feed_obj.podcast_locked)
                    .bind(truncate_string(&feed_obj.podcast_owner, 255))
                    .bind(feed_obj.id)
                    .execute(&pool)
                    .await
                    .unwrap();

                    if result.rows_affected() == 0 {
                        println!("Error updating feed record for feed: [{}]", feed.url);
                    }

                    // dbcalls--;
            }

        } else {
            //error parsing feed
            println!("Error parsing feed.");
            // println!("{:#?}", parsed_content);
            //process.exit(1);

            //Update the feed record with what we discovered
            // dbcalls += 1;
            println!("Updating error feed: [{}|{}] in the database.", feed_obj.id, feed_obj.url);

            let query = format!(
                "UPDATE {} SET content='', itunes_id=?, updated=0, parse_errors=parse_errors+1, parsenow=0 WHERE id=?",
                cg_table_newsfeeds
            );

            let result = sqlx::query(&query)
                .bind(feed_obj.itunes_id)
                .bind(feed_obj.id)
                .execute(&pool)
                .await
                .unwrap();

            if result.rows_affected() == 0 {
                println!("Error updating feed record for feed: [{}]", feed.url);
            }
        }

        feed_work_count += 1;
    }
}

fn loggit(cfg: &LoggingConfig, lognum: u8, message: &str) -> Result<bool, Box<dyn Error>> {
    //Timestamp for this log
    let current_date = Local::now();
    let tstamp = current_date.format("%-m/%-d/%Y").to_string();

    if lognum == 1 && cfg.log_errors_only == 1 {
        return Ok(true);
    }

    let filename = match lognum {
        1 => format!("{}/{}", cfg.log_folder, cfg.acclog),
        2 => format!("{}/{}", cfg.log_folder, cfg.errlog),
        3 => format!("{}/{}", cfg.log_folder, cfg.dbglog),
        _ => "".to_string(),
    };

    //Open the file
    if filename.is_empty() {
        return Ok(false);
    }

    let mut fd = File::open(filename)?;

    //Write the message
    write!(fd, "[{}] [LOCAL] ({}) {}\n", tstamp, file!(), message)?;

    //Return
    return Ok(true);
}

fn write_file(filename: &str, content: &str) -> Result<bool, Box<dyn Error>> {
    let tmp_filename = format!("/tmp/{}", filename);

    let mut fd = File::open(tmp_filename)?;
    write!(fd, "{}", content)?;

    return Ok(true);
}

fn get_params(input: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    for param in input.split(';') {
        let parts: Vec<String> = param.split('=').map(|part| part.trim().to_string()).collect();
        if parts.len() == 2 {
            params.insert(parts[0].clone(), parts[1].clone());
        }
    }

    return params;
}

fn maybe_translate(content: &str, charset: Option<&str>) -> String {
    let bytes = content.as_bytes();

    if let Some(charset) = charset {
        // Check if charset is not UTF-8
        if !charset.to_lowercase().contains("utf-8") {
            // Attempt encoding conversion
            match encoding_rs::Encoding::for_label(charset.as_bytes()) {
                Some(encoding) => {
                    let (result, _, _) = encoding.decode(bytes);
                    println!("Converting from charset {} to utf-8", charset);
                    return result.into();
                }
                None => {
                    println!("Error: Unsupported charset '{}'.", charset);
                    return String::from_utf8_lossy(bytes).to_string();
                }
            }
        }
    }

    // If the charset is already UTF-8, or no charset is provided, return the content as-is.
    String::from_utf8_lossy(bytes).to_string()
}

fn pub_date_to_timestamp(pub_date: &str) -> Result<i64, chrono::ParseError> {
    if let Ok(timestamp) = pub_date.parse::<i64>() {
        return Ok(timestamp); // If it's already a number (as a string), return it as is
    }

    // likely need more formats
    let format = "%a, %d %b %Y %H:%M:%S %z";

    // try to parse
    DateTime::parse_from_str(pub_date, format).map(|dt| dt.timestamp())
}

//Get a mime-type string for an unknown media enclosure
fn guess_enclosure_type(url: &str) -> &str {
    if url.contains(".m4v") {
        return "video/mp4";
    }
    if url.contains(".mp4") {
        return "video/mp4";
    }
    if url.contains(".avi") {
        return "video/avi";
    }
    if url.contains(".mov") {
        return "video/quicktime";
    }
    if url.contains(".mp3") {
        return "audio/mpeg";
    }
    if url.contains(".m4a") {
        return "audio/mp4";
    }
    if url.contains(".wav") {
        return "audio/wav";
    }
    if url.contains(".ogg") {
        return "audio/ogg";
    }
    if url.contains(".wmv") {
        return "video/x-ms-wmv";
    }

    "" // Return an empty string if no match
}

//Parse out all of the links from an atom entry and see which ones are enclosures

#[derive(Debug)]
struct PubSubLinks {
    hub: String,
    self_link: String,
}

fn find_pubsub_links(channel: &rss::RssChannel) -> Option<PubSubLinks> {
    let mut pubsub_links = PubSubLinks {
        hub: String::new(),
        self_link: String::new(),
    };

    //Multiple link objects in an array?
    for item in &channel.link {
        if let Some(rel) = &item.rel {
            if rel == "hub" {
                if let Some(href) = &item.href {
                    if !href.is_empty() {
                        pubsub_links.hub = href.clone();
                    }
                }
            } else if rel == "self" {
                if let Some(href) = &item.href {
                    if !href.is_empty() {
                        pubsub_links.self_link = href.clone();
                    }
                }
            }
        }
    }

    // Check links in the "atom:link" field
    for item in &channel.atom_link {
        if let Some(rel) = &item.rel {
            if rel == "hub" {
                if let Some(href) = &item.href {
                    if !href.is_empty() {
                        pubsub_links.hub = href.clone();
                    }
                }
            } else if rel == "self" {
                if let Some(href) = &item.href {
                    if !href.is_empty() {
                        pubsub_links.self_link = href.clone();
                    }
                }
            }
        }
    }

    // If both hub and self are found, return the pubsub links
    if !pubsub_links.hub.is_empty() && !pubsub_links.self_link.is_empty() {
        Some(pubsub_links)
    } else {
        None
    }
}

fn contains_non_latin_codepoints(s: &str) -> bool {
    // Check if the string contains characters outside the Latin-1 range (0x00 to 0xFF)
    for c in s.chars() {
        if (c as u32) > 0xFF {
            return true;
        }
    }

    // Check if the string contains characters outside the ASCII range (0x00 to 0x7F)
    s.chars().any(|c| c > '\x7F')
}

fn sanitize_url(url: &str) -> String {
    if url.is_empty() {
        return String::new();
    }

    if !contains_non_latin_codepoints(url) {
        return url.chars().take(768).collect();
    }

    // Percent-encode the URL and limit the length to 768 characters
    let mut new_url = utf8_percent_encode(url, NON_ALPHANUMERIC).to_string();

    // Check again for non-Latin characters after encoding
    if contains_non_latin_codepoints(&new_url) {
        // Replace non-Latin characters with space
        new_url = new_url.chars().map(|c| {
            if c > '\u{007F}' {
                ' '  // Replace non-Latin characters with space
            } else {
                c
            }
        }).collect();
    }

    // Ensure that the URL is at most 768 characters long
    return new_url.chars().take(768).collect();
}

async fn mark_feed_as_unparseable(pool: &sqlx::SqlitePool, newsfeeds_table: &str, feed: &DbFeed) -> Result<(), Box<dyn Error>> {
    println!("Marking feed: [{} | {}] as unparseable in the database.", feed.id, feed.url);

    let query = format!(
        "UPDATE {} SET 
        content='',
        type=?,
        generator='',
        title='',
        link='',
        description='',
        itunes_author='',
        itunes_owner_name='',
        itunes_owner_email='',
        itunes_new_feed_url='',
        explicit=0,
        image='',
        itunes_type='',
        itunes_id=?,
        updated=0, 
        parsenow=0 
        WHERE id=?",
        newsfeeds_table
    );

    let result = sqlx::query(&query)
        .bind(feed.feed_type)
        .bind(feed.itunes_id)
        .bind(feed.id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        println!("Error updating unparseable feed record for feed: [{}]", feed.url);
    }

    Ok(())
}

fn truncate_string(s: &str, length: usize) -> String {
    if s.is_empty() {
        return String::new();
    }

    // Truncate the string to the specified length
    let truncated = s.chars().take(length).collect::<String>();
    truncated
}

fn truncate_int(number: i64) -> i32 {
    if number > 2147483647 {
        return 2147483647;
    }

    if number < -2147483647 {
        return -2147483647;
    }

    // Return 0 if the number is not a valid integer (NaN in JS)
    // if new_number.is_nan() {
    //     return 0;
    // }

    number as i32
}

fn truncate_u64(number: u64) -> i32 {
    if number > 2147483647 {
        return 2147483647;
    }

    number as i32
}

fn read_feed_file(folder: &str, feed_id: u32) -> String {
    let file_path = format!("{}{}.txt", folder, feed_id); // Assuming config.folders.cg_feeds is set up

    match fs::read_to_string(&file_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            String::new()
        }
    }
}

fn delete_feed_file(folder: &str, feed_id: u32) -> bool {
    return true;
    let file_path = format!("{}{}.txt", folder, feed_id); // Assuming config.folders.cg_feeds is set up

    match fs::remove_file(&file_path) {
        Ok(_) => true,   // Successfully deleted the file
        Err(err) => {
            eprintln!("Error deleting file: {}", err); // Print the error to stderr
            false // Return false if an error occurred
        }
    }
}

fn feed_file_exists(folder: &str, feed_id: u32) -> bool {
    let file_path = format!("{}{}.txt", folder, feed_id);
    Path::new(&file_path).exists() // Check if the file exists
}

fn calculate_days(new_item_time: i64, old_item_time: i64) -> i32 {
    let diff_seconds = new_item_time - old_item_time;

    if diff_seconds < 0 {
        return 9;
    }
    if diff_seconds < 108000 {
        return 1; // 30 hours
    }
    if diff_seconds < 259200 {
        return 2; // 3 days
    }
    if diff_seconds < 864000 {
        return 3; // 10 days
    }
    if diff_seconds < 1728000 {
        return 4; // 20 days
    }
    if diff_seconds < 3456000 {
        return 5; // 40 days
    }
    if diff_seconds < 7776000 {
        return 6; // 90 days
    }
    if diff_seconds < 17280000 {
        return 7; // 200 days
    }
    if diff_seconds < 31536000 {
        return 8; // 365 days
    }
    return 0;
}

fn calculate_update_frequency(items: Vec<i64>) -> u8 {
    let time_400_days_ago = 400 * 86400; // 400 days in seconds
    let time_200_days_ago = 200 * 86400; // 200 days in seconds
    let time_100_days_ago = 100 * 86400; // 100 days in seconds
    let time_40_days_ago = 40 * 86400;   // 40 days in seconds
    let time_20_days_ago = 20 * 86400;   // 20 days in seconds
    let time_10_days_ago = 10 * 86400;   // 10 days in seconds
    let time_5_days_ago = 5 * 86400;     // 5 days in seconds

    // Feeds that rarely update
    if items.iter().filter(|&&time| time > time_400_days_ago).count() == 0 {
        return 9;
    }
    if items.iter().filter(|&&time| time > time_200_days_ago).count() == 0 {
        return 8;
    }
    if items.iter().filter(|&&time| time > time_100_days_ago).count() == 0 {
        return 7;
    }

    // Frequency checks
    if items.iter().filter(|&&time| time > time_5_days_ago).count() > 1 {
        return 1;
    }
    if items.iter().filter(|&&time| time > time_10_days_ago).count() > 1 {
        return 2;
    }
    if items.iter().filter(|&&time| time > time_20_days_ago).count() > 1 {
        return 3;
    }
    if items.iter().filter(|&&time| time > time_40_days_ago).count() > 1 {
        return 4;
    }
    if items.iter().filter(|&&time| time > time_100_days_ago).count() > 1 {
        return 5;
    }
    if items.iter().filter(|&&time| time > time_200_days_ago).count() > 1 {
        return 6;
    }
    if items.iter().filter(|&&time| time > time_400_days_ago).count() >= 1 {
        return 7;
    }

    // Give up
    return 0;
}

//Determine categories list and update the database to reflect
fn insert_categories(feed_id: u32, mut feed_categories: HashSet<String>) -> Option<String> {
    // Static map of categories (converted to lowercase)
    let cat_lookup = vec![
        "", "arts", "books", "design", "fashion", "beauty", "food", "performing", "visual",
        "business", "careers", "entrepreneurship", "investing", "management", "marketing", "nonprofit",
        "comedy", "interviews", "improv", "standup", "education", "courses", "howto", "language", "learning",
        "selfimprovement", "fiction", "drama", "history", "health", "fitness", "alternative", "medicine", "mental",
        "nutrition", "sexuality", "kids", "family", "parenting", "pets", "animals", "stories", "leisure", "animation",
        "manga", "automotive", "aviation", "crafts", "games", "hobbies", "home", "garden", "videogames", "music",
        "commentary", "news", "daily", "entertainment", "government", "politics", "buddhism", "christianity",
        "hinduism", "islam", "judaism", "religion", "spirituality", "science", "astronomy", "chemistry", "earth",
        "life", "mathematics", "natural", "nature", "physics", "social", "society", "culture", "documentary", "personal",
        "journals", "philosophy", "places", "travel", "relationships", "sports", "baseball", "basketball", "cricket",
        "fantasy", "football", "golf", "hockey", "rugby", "running", "soccer", "swimming", "tennis", "volleyball",
        "wilderness", "wrestling", "technology", "truecrime", "tv", "film", "aftershows", "reviews", "climate", "weather",
        "tabletop", "role-playing", "cryptocurrency"
    ];
    
    let max = 8;
    let mut cat_count = 0;
    let mut arr_categories = vec![0; 13]; // Array of 13 elements (index 0 is unused)

    // Do compound categories
    if feed_categories.contains(&"video".to_string()) && feed_categories.contains(&"games".to_string()) {
        feed_categories.insert("videogames".to_string());
    }
    if feed_categories.contains(&"true".to_string()) && feed_categories.contains(&"crime".to_string()) {
        feed_categories.insert("truecrime".to_string());
    }
    if feed_categories.contains(&"after".to_string()) && feed_categories.contains(&"shows".to_string()) {
        feed_categories.insert("aftershows".to_string());
    }
    if feed_categories.contains(&"self".to_string()) && feed_categories.contains(&"improvement".to_string()) {
        feed_categories.insert("selfimprovement".to_string());
    }
    if feed_categories.contains(&"how".to_string()) && feed_categories.contains(&"to".to_string()) {
        feed_categories.insert("howto".to_string());
    }

    // Index lookup
    for (index, item) in feed_categories.iter().enumerate() {
        if index >= max {
            break;
        }

        let lookup = item.replace(' ', "").replace('-', "");

        if let Some(cat) = cat_lookup.iter().position(|cat| cat == &lookup) {
            if cat > 0 { // Ignore the first empty string entry
                arr_categories[cat_count + 1] = cat;
                cat_count += 1;
            }
        }
    }

    if cat_count == 0 {
        return None;
    }

    Some(format!(
        "({},{},{},{},{},{},{},{},{},{},{}),",
        feed_id,
        arr_categories[1],
        arr_categories[2],
        arr_categories[3],
        arr_categories[4],
        arr_categories[5],
        arr_categories[6],
        arr_categories[7],
        arr_categories[8],
        arr_categories[9],
        arr_categories[10]
    ))
}

/*
* Convert time string to seconds
* 01:02 = 62 seconds
* Thanks to Glenn Bennett!
*/
fn time_to_seconds(time_string: &str) -> u32 {
    let mut seconds: u32 = 0;
    let parts: Vec<&str> = time_string.split(':').collect();

    match parts.len() - 1 {
        1 => {
            // mm:ss format
            if let (Ok(min), Ok(sec)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                seconds = min * 60 + sec;
            }
        }
        2 => {
            // hh:mm:ss format
            if let (Ok(hr), Ok(min), Ok(sec)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                seconds = hr * 60 * 60 + min * 60 + sec;
            }
        }
        _ => {
            // If the string doesn't match expected format, use the original value if it’s a number
            if let Ok(val) = time_string.parse::<u32>() {
                seconds = val;
            }
        }
    }

    // Return 30 minutes (1800 seconds) if the value is NaN or not parsable
    if seconds == 0 && !time_string.is_empty() {
        seconds = 30 * 60;
    }

    seconds
}