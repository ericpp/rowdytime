use yaserde::YaDeserialize;
// use serde_xml_rs::from_str;

use std::collections::HashMap;

// use quick_xml::de::{from_str, DeError};
// use quick_xml::events::{BytesStart, Event};
// use quick_xml::Reader;
// use quick_xml::de::{from_str, DeError};

use std::str::from_utf8;

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssFeed {
    pub channel: Option<RssChannel>,
}

#[derive(YaDeserialize, Clone, Debug)]
#[yaserde(
  namespaces = {
    "atom" = "http://www.w3.org/2005/Atom",
    "itunes" = "http://www.itunes.com/dtds/podcast-1.0.dtd",
    "podcast" = "https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md",
    "podcast2" = "https://podcastindex.org/namespace/1.0",
  }
)]
// #[yaserde(rename_all = "camelCase")]
pub struct RssChannel {
    pub title: Option<String>,
    pub link: Vec<RssLink>,
    pub language: Vec<String>,
    pub generator: Vec<String>,

    #[yaserde(rename = "link", prefix = "atom")]
    pub atom_link: Vec<RssAtomLink>,

    #[yaserde(rename = "pubDate")]
    pub pub_date: Vec<String>,

    #[yaserde(rename = "lastBuildDate")]
    pub last_build_date: Option<String>,

    #[yaserde(rename = "type", prefix = "itunes")]
    pub itunes_type: Vec<RssItunesType>,

    #[yaserde(rename = "category", prefix = "itunes")]
    pub itunes_category: Vec<RssItunesCategory>,

    #[yaserde(rename = "new-feed-url", prefix = "itunes")]
    pub itunes_new_feed_url: Vec<String>,

    #[yaserde(rename = "author", prefix = "itunes")]
    pub itunes_author: Vec<String>,

    #[yaserde(rename = "owner", prefix = "itunes")]
    pub itunes_owner: Option<RssItunesOwner>,

    #[yaserde(rename = "image", prefix = "itunes")]
    pub itunes_image: Vec<RssItunesImage>,

    pub image: Vec<RssImage>,

    #[yaserde(rename = "explicit", prefix = "itunes", text = true)]
    pub itunes_explicit: Vec<String>,

    pub description: Option<String>,

    #[yaserde(rename = "summary", prefix = "itunes")]
    pub itunes_summary: Vec<String>,

    #[yaserde(rename = "value", prefix = "podcast")]
    #[yaserde(rename = "value", prefix = "podcast2")]
    pub podcast_value: Vec<RssPodcastValue>,

    #[yaserde(rename = "locked", prefix = "podcast")]
    #[yaserde(rename = "locked", prefix = "podcast2")]
    pub podcast_locked: Option<RssPodcastLocked>,

    #[yaserde(rename = "funding", prefix = "podcast")]
    #[yaserde(rename = "funding", prefix = "podcast2")]
    pub podcast_funding: Vec<RssPodcastFunding>,

    #[yaserde(rename = "guid", prefix = "podcast")]
    #[yaserde(rename = "guid", prefix = "podcast2")]
    pub podcast_guid: Vec<String>,

    pub item: Vec<RssItem>,
}

#[derive(YaDeserialize, Clone, Debug)]
#[yaserde(
  namespaces = {
    "atom" = "http://www.w3.org/2005/Atom",
    "itunes" = "http://www.itunes.com/dtds/podcast-1.0.dtd",
    "podcast" = "https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md",
    "podcast2" = "https://podcastindex.org/namespace/1.0",
    "content" = "http://purl.org/rss/1.0/modules/content/",
  }
)]
pub struct RssItem {
    pub title: Option<String>,
    pub description: Option<String>,
    pub guid: Option<String>,
    pub link: Option<RssLink>,

    #[yaserde(rename = "pubDate")]
    pub pub_date: Option<String>,

    pub enclosure: Vec<RssItemEnclosure>,

    #[yaserde(rename = "episode", prefix = "itunes")]
    pub itunes_episode: Option<String>,

    #[yaserde(rename = "episodeType", prefix = "itunes")]
    pub itunes_episode_type: Vec<String>,

    #[yaserde(rename = "season", prefix = "itunes")]
    pub itunes_season: Vec<u64>,

    #[yaserde(rename = "explicit", prefix = "itunes")]
    pub itunes_explicit: Option<String>,

    #[yaserde(rename = "duration", prefix = "itunes")]
    pub itunes_duration: Option<String>,

    #[yaserde(rename = "image", prefix = "itunes")]
    pub itunes_image: Vec<RssItunesImage>,
    pub image: Vec<RssImage>,

    #[yaserde(rename = "title", prefix = "itunes")]
    pub itunes_title: Option<String>,

    #[yaserde(rename = "summary", prefix = "itunes")]
    pub itunes_summary: Option<String>,

    #[yaserde(rename = "encoded", prefix = "content")]
    pub content_encoded: Option<String>,

    #[yaserde(rename = "value", prefix = "podcast")]
    #[yaserde(rename = "value", prefix = "podcast2")]
    pub podcast_value: Vec<RssPodcastValue>,

    #[yaserde(rename = "transcript", prefix = "podcast")]
    #[yaserde(rename = "transcript", prefix = "podcast2")]
    pub podcast_transcript: Vec<RssPodcastTranscript>,

    #[yaserde(rename = "chapters", prefix = "podcast")]
    #[yaserde(rename = "chapters", prefix = "podcast2")]
    pub podcast_chapters: Option<RssPodcastChapters>,

    #[yaserde(rename = "soundbite", prefix = "podcast")]
    #[yaserde(rename = "soundbite", prefix = "podcast2")]
    pub podcast_soundbite: Vec<RssPodcastSoundbite>,

    #[yaserde(rename = "person", prefix = "podcast")]
    #[yaserde(rename = "person", prefix = "podcast2")]
    pub podcast_person: Vec<RssPodcastPerson>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssItemEnclosure {
    #[yaserde(attribute = true)]
    pub url: Option<String>,

    #[yaserde(attribute = true)]
    pub length: Option<u64>,

    #[yaserde(attribute = true, rename = "type")]
    pub enclosure_type: Option<String>,

    #[yaserde(text = true)]
    pub value: Option<String>,
}


#[derive(YaDeserialize, Clone, Debug)]
pub struct RssPodcastTranscript {
    #[yaserde(attribute = true)]
    pub url: Option<String>,

    #[yaserde(attribute = true, rename = "type")]
    pub mime_type: Option<String>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssPodcastChapters {
    #[yaserde(attribute = true)]
    pub url: Option<String>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssPodcastSoundbite {
    #[yaserde(attribute = true, rename = "startTime")]
    pub start_time: Option<String>,

    #[yaserde(attribute = true)]
    pub duration: Option<String>,

    #[yaserde(text = true)]
    pub value: Option<String>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssPodcastPerson {
    #[yaserde(attribute = true)]
    pub img: Option<String>,

    #[yaserde(attribute = true)]
    pub href: Option<String>,

    #[yaserde(attribute = true)]
    pub role: Option<String>,

    #[yaserde(attribute = true)]
    pub group: Option<String>,

    #[yaserde(text = true)]
    pub value: Option<String>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssLink {
    pub rel: Option<String>,
    pub href: Option<String>,
    pub xmlns: Option<String>,
    #[yaserde(text = true)]
    pub value: Option<String>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssPodcastLocked {
    #[yaserde(attribute = true)]
    pub owner: Option<String>,
    #[yaserde(attribute = true)]
    pub email: Option<String>,
    #[yaserde(text = true)]
    pub value: Option<String>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssPodcastFunding {
    pub url: Option<String>,
    #[yaserde(text = true)]
    pub value: Option<String>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssImage {
    pub url: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>,
}

#[derive(YaDeserialize, Clone, Debug)]
#[yaserde(
  namespaces = {
    "itunes" = "http://www.itunes.com/dtds/podcast-1.0.dtd",
    "podcast" = "https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md",
    "podcast2" = "https://podcastindex.org/namespace/1.0",
  }
)]
pub struct RssItunesCategory {
    #[yaserde(attribute = true)]
    pub text: Option<String>,
    #[yaserde(rename = "category", prefix = "itunes")]
    pub sub_category: Vec<RssItunesCategory>,
}

#[derive(YaDeserialize, Clone, Debug)]
#[yaserde(
  namespaces = {
    "itunes" = "http://www.itunes.com/dtds/podcast-1.0.dtd",
    "podcast" = "https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md",
    "podcast2" = "https://podcastindex.org/namespace/1.0",
  }
)]
pub struct RssPodcastValue {
    #[yaserde(attribute = true, rename = "type")]
    pub value_type: Option<String>,

    #[yaserde(attribute = true)]
    pub method: Option<String>,

    #[yaserde(attribute = true)]
    pub suggested: Option<String>,

    #[yaserde(rename = "valueRecipient", prefix = "podcast")]
    #[yaserde(rename = "valueRecipient", prefix = "podcast2")]
    pub value_recipients: Vec<RssValueRecipient>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssValueRecipient {
    #[yaserde(attribute = true)]
    pub address: Option<String>,

    #[yaserde(attribute = true)]
    pub name: Option<String>,

    #[yaserde(attribute = true)]
    pub split: Option<f64>,

    #[yaserde(attribute = true, rename = "type")]
    pub recipient_type: Option<String>,

    #[yaserde(attribute = true, rename = "customKey")]
    pub custom_key: Option<u64>,

    #[yaserde(attribute = true, rename = "customValue")]
    pub custom_value: Option<String>,

    #[yaserde(attribute = true)]
    pub fee: Option<String>,
}

#[derive(YaDeserialize, Clone, Debug)]
#[yaserde(
  namespaces = {
    "itunes" = "http://www.itunes.com/dtds/podcast-1.0.dtd",
    "podcast" = "https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md",
    "podcast2" = "https://podcastindex.org/namespace/1.0",
  }
)]
pub struct RssItunesOwner {
    #[yaserde(rename = "name", prefix = "itunes")]
    pub name: Option<String>,
    #[yaserde(rename = "email", prefix = "itunes")]
    pub email: Option<String>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssItunesImage {
    pub url: Option<String>,

    #[yaserde(attribute = true)]
    pub href: Option<String>,

    #[yaserde(text = true)]
    pub value: Option<String>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssAtomLink {
    #[yaserde(attribute = true)]
    pub href: Option<String>,
    #[yaserde(attribute = true)]
    pub rel: Option<String>,
    #[yaserde(attribute = true, rename = "type")]
    pub link_type: Option<String>,
}

#[derive(YaDeserialize, Clone, Debug)]
pub struct RssItunesType {
    #[yaserde(rename = "text", attribute = true)]
    pub text: Option<String>,
    #[yaserde(text = true)]
    pub value: Option<String>,
}

// #[yaserde(text = true)]
// value: String

// Your helper function to clean titles and links
fn clean_string(value: Option<String>) -> String {
    value.unwrap_or_default().trim().replace(&['\r', '\n'][..], "")
}

// Your helper function to find pubsub links (you might need to define this based on XML structure)
// fn find_pubsub_links(channel: &RssChannel) -> Vec<String> {
//     vec![] // Example implementation, replace with actual logic
// }

pub fn parse_feed(content: &str) -> Result<RssFeed, Box<dyn std::error::Error>> {
    // let mut de = serde_xml_rs::Deserializer::new_from_reader(content.as_bytes())
    //     .non_contiguous_seq_elements(true);

    // let parsed = RssFeed::deserialize(&mut de)?;
    // Ok(parsed)

    // let parsed: RssFeed = from_str(content)?;
    // Ok(parsed)

    // let parsed = parse_xml_with_namespace(content)?;
    // Ok(parsed)

    let parsed: RssFeed = yaserde::de::from_str(content)?;
    Ok(parsed)
}


// fn process_feed(feed: Rss) {
//     // Processing RSS feed like in the JavaScript code
//     println!("Feed Title: {}", feed.channel.title);
//     println!("Feed Link: {}", feed.channel.link);
    
//     // Handle 'itunes:category'
//     if let Some(itunes_category) = feed.channel.itunes_category {
//         for category in itunes_category {
//             if let Some(sub_category) = category.sub_category {
//                 for sub in sub_category {
//                     if let Some(attr) = sub.attributes {
//                         if let Some(value) = attr.get("@_text") {
//                             println!("Category: {}", value);
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     // Process 'value' block
//     if let Some(value_block) = feed.channel.value {
//         println!("{:?}", value_block);
//     }
    
//     // Handle 'itunes:author'
//     if let Some(author) = feed.channel.itunes_author {
//         println!("Author: {}", author);
//     }

//     // Handle explicit content
//     if let Some(explicit) = feed.channel.explicit {
//         println!("Explicit content: {}", explicit);
//     }

//     // Process feed description
//     if let Some(description) = feed.channel.description {
//         println!("Description: {}", description);
//     }

//     // Process podcast value recipients
//     if let Some(value_recipient) = feed.channel.podcast_value {
//         if let Some(destinations) = value_recipient.destinations {
//             for dest in destinations {
//                 if let Some(attr) = dest.attributes {
//                     if let Some(name) = attr.get("@_name") {
//                         println!("Destination Name: {}", name);
//                     }
//                 }
//             }
//         }
//     }
// }

//     if let Some(rss) = parsed_feed.rss {
//         if let Some(channel) = rss.channel {
//             // Handle missing channel case
//             if channel.title.is_none() {
//                 // Set feed type to 0 or handle it
//                 println!("Invalid feed: Missing channel title");
//                 return Ok(());
//             }

//             let mut feed_obj = FeedObj::default();

//             // Extract key attributes
//             feed_obj.title = clean_string(channel.title);
//             feed_obj.link = clean_string(channel.link);
//             feed_obj.language = channel.language.unwrap_or_default();
//             feed_obj.generator = channel.generator.unwrap_or_default();
//             feed_obj.pub_date = channel.pubDate.unwrap_or_default();
//             feed_obj.last_build_date = channel.lastBuildDate.unwrap_or_default();
//             feed_obj.itunes_type = channel.itunes_type.unwrap_or_default();
//             feed_obj.itunes_new_feed_url = channel.itunes_new_feed_url.unwrap_or_default();

//             feed_obj.categories = collect_categories(&channel);

//             // Clean the title and link
//             feed_obj.title = clean_string(Some(feed_obj.title));
//             feed_obj.link = clean_string(Some(feed_obj.link));

//             // Pubsub links
//             feed_obj.pubsub = find_pubsub_links(&channel);

//             println!("{:?}", feed_obj); // Debug print
//         }
//     }

//     Ok(())
// }



// fn collect_categories(channel: &Channel) -> Vec<String> {
//     let mut categories = HashSet::new();

//     if let Some(itunes_category) = &channel.itunes_category {
//         for category in itunes_category {
//             if let Some(text) = &category.text {
//                 let cleaned_category = text
//                     .to_lowercase()
//                     .replace("&amp;", "")
//                     .split_whitespace()
//                     .collect::<Vec<String>>(); 
//                 categories.extend(cleaned_category);
//             }

//             if let Some(sub_categories) = &category.sub_category {
//                 for sub_category in sub_categories {
//                     if let Some(text) = &sub_category.text {
//                         let cleaned_category = text
//                             .to_lowercase()
//                             .replace("&amp;", "")
//                             .split_whitespace()
//                             .collect::<Vec<String>>(); 
//                         categories.extend(cleaned_category);
//                     }
//                 }
//             }
//         }
//     }

//     categories.into_iter().collect()
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let feed_content = r#"
//     <rss>
//         <channel>
//             <title>Sample Feed</title>
//             <link>https://example.com</link>
//             <language>en</language>
//             <generator>SomeGenerator</generator>
//             <pubDate>Wed, 06 Feb 2025 00:00:00 GMT</pubDate>
//             <lastBuildDate>Wed, 06 Feb 2025 00:00:00 GMT</lastBuildDate>
//             <itunes:type>podcast</itunes:type>
//             <itunes:category>Technology</itunes:category>
//             <itunes:new-feed-url>https://example.com/feed</itunes:new-feed-url>
//         </channel>
//     </rss>
//     "#;

//     parse_feed(feed_content)?;

//     Ok(())
// }


// fn parse_xml_with_namespace(xml_data: &str) -> Result<RssFeed, DeError> {
//     let mut reader = Reader::from_str(xml_data);
//     reader.config_mut().trim_text(true);

//     let mut buf = Vec::new();

//     let mut channel = RssChannel {
//         title: None,

//         language: None,
//         generator: None,
//         pub_date: None,
//         last_build_date: None,

//         itunes_type: None,

//         itunes_category: Vec::new(),

//         itunes_new_feed_url: None,

//         categories: None,
//         value: None,

//         itunes_author: None,

//         itunes_owner: None,

//         podcast_locked: None,

//         itunes_explicit: None,
//         explicit: None,

//         description: None,

//         podcast_guid: None,

//         podcast_funding: None,

//         podcast_value: None,

//         atom_link: None,

//         link: None,

//         itunes_image: Vec::new(),

//         // pub image: None,

//         itunes_summary: None,
//     };

//     // Manually parse XML and extract data, including namespaces
//     loop {
//         match reader.read_event_into(&mut buf)? {
//             Event::Start(e) => {
//                 println!("{:#?}", e);

//                 // if e.name().as_ref() == b"itunes:summary" {
//                 //     let text = String::from(reader.read_text(e.name()).unwrap().as_ref());
//                 //     println!("{:#?}", text);
//                 // }

//                 match e.name().as_ref() {
//                     // b"itunes:author" => {
//                     //     channel.itunes_author = Some(String::from(reader.read_text(e.name()).unwrap().as_ref()));
//                     // }
//                     // b"itunes:subtitle" => {
//                     //     channel.itunes_subtitle = Some(String::from(reader.read_text(e.name()).unwrap().as_ref()));
//                     // }
//                     b"itunes:summary" => {
//                         channel.itunes_summary = Some(String::from(reader.read_text(e.name()).unwrap().as_ref()));
//                     }
//                     b"title" => {
//                         channel.title = Some(String::from(reader.read_text(e.name()).unwrap().as_ref()));
//                     }
//                     // b"link" => {
//                     //     channel.link = Some(String::from(reader.read_text(e.name()).unwrap().as_ref()));
//                     // }
//                     b"description" => {
//                         channel.description = Some(String::from(reader.read_text(e.name()).unwrap().as_ref()));
//                     }
//                     // b"pubDate" => {
//                     //     channel.pubDate = Some(String::from(reader.read_text(e.name()).unwrap().as_ref()));
//                     // }
//                     _ => (),
//                 }
//             }

//             // Event::Text(e) => txt.push(e.unescape().unwrap().into_owned()),
//             Event::Eof => break,

//             // There are several other `Event`s we do not consider here
//             _ => (),
//         }

//         buf.clear();
//     }

//     let mut feed = RssFeed {
//         channel: Some(channel),
//     };
// println!("{:#?}", feed);
//     Ok(feed)
// }