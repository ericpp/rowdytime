
--
-- Database: pcapi
--

-- --------------------------------------------------------

--
-- Table structure for table api_tokens
--

CREATE TABLE api_tokens (
  id integer PRIMARY KEY AUTOINCREMENT,
  userid bigint NOT NULL,
  keyval varchar(20),
  secretval varchar(40),
  createdon datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  permlevel int NOT NULL DEFAULT '0',
  rate_limited int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table categories
--

CREATE TABLE categories (
  id integer PRIMARY KEY AUTOINCREMENT,
  name varchar(128) NOT NULL DEFAULT ''
);

-- --------------------------------------------------------

--
-- Table structure for table category_map
--

CREATE TABLE category_map (
  id integer PRIMARY KEY AUTOINCREMENT,
  categoryid bigint NOT NULL,
  feedid bigint NOT NULL
);

-- --------------------------------------------------------

--
-- Table structure for table developers
--

CREATE TABLE developers (
  id integer PRIMARY KEY AUTOINCREMENT,
  email varchar(512) NOT NULL,
  name varchar(255) NOT NULL DEFAULT '',
  active tinyint NOT NULL DEFAULT '0',
  password varchar(255),
  lastlogin datetime NOT NULL,
  lastpasschange datetime NOT NULL,
  username varchar(64),
  activationcode varchar(49)
);

-- --------------------------------------------------------

--
-- Table structure for table directory_apple
--

CREATE TABLE directory_apple (
  id integer PRIMARY KEY AUTOINCREMENT,
  description varchar(512) NOT NULL DEFAULT '',
  itunes_id bigint NOT NULL,
  itunes_url varchar(768),
  time_createdon int NOT NULL,
  feed_url varchar(768),
  title_non_english tinyint NOT NULL DEFAULT '0' ,
  artwork_url_30 varchar(768),
  artwork_url_60 varchar(768),
  artwork_url_100 varchar(768),
  artwork_url_600 varchar(768),
  dead tinyint NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table feeds_added
--

CREATE TABLE feeds_added (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint DEFAULT NULL,
  userid int DEFAULT NULL,
  developerid bigint DEFAULT NULL,
  time_added int NOT NULL DEFAULT '0',
  source tinyint NOT NULL DEFAULT '0' ,
  processed tinyint NOT NULL DEFAULT '0',
  stage tinyint NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table flags
--

CREATE TABLE flags (
  name varchar(32),
  value int NOT NULL,
  timeset varchar(64),
  setby varchar(255)
);

-- --------------------------------------------------------

--
-- Table structure for table genres
--

CREATE TABLE genres (
  id integer PRIMARY KEY AUTOINCREMENT,
  itunes_genre_id int NOT NULL DEFAULT '0',
  title varchar(256),
  subgenre int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table newsfeeds
--

CREATE TABLE newsfeeds (
  id integer PRIMARY KEY AUTOINCREMENT,
  title varchar(768) NOT NULL DEFAULT '',
  url varchar(768),
  lastcheck int NOT NULL DEFAULT '0',
  lastupdate int NOT NULL DEFAULT '0',
  lastmod int NOT NULL DEFAULT '0',
  createdon int NOT NULL DEFAULT '0',
  content longtext  NOT NULL,
  link varchar(768) NOT NULL DEFAULT '',
  errors int NOT NULL DEFAULT '0',
  updated tinyint NOT NULL DEFAULT '0',
  lastitemid varchar(768)  NOT NULL DEFAULT '',
  pubdate varchar(64) NOT NULL DEFAULT '',
  contenthash varchar(40)  NOT NULL DEFAULT '',
  lasthttpstatus int NOT NULL DEFAULT '0',
  lastgoodhttpstatus int NOT NULL DEFAULT '0',
  dead tinyint NOT NULL DEFAULT '0',
  contenttype varchar(128) NOT NULL DEFAULT '',
  itunes_id bigint DEFAULT NULL,
  duplicateof bigint DEFAULT NULL,
  original_url varchar(768)  NOT NULL DEFAULT '',
  artwork_url_600 varchar(768)  NOT NULL DEFAULT '',
  description mediumtext NOT NULL,
  itunes_author varchar(2048) DEFAULT NULL,
  itunes_owner_email varchar(1024) DEFAULT NULL,
  itunes_owner_name varchar(1024) DEFAULT NULL,
  itunes_new_feed_url varchar(768),
  explicit tinyint NOT NULL DEFAULT '0',
  image varchar(768),
  itunes_type varchar(32) DEFAULT NULL,
  type tinyint NOT NULL DEFAULT '0',
  generator varchar(128) DEFAULT NULL,
  parse_errors int NOT NULL DEFAULT '0',
  lastparse int NOT NULL DEFAULT '0',
  pullnow tinyint NOT NULL DEFAULT '0' ,
  parsenow tinyint NOT NULL DEFAULT '0' ,
  newest_item_pubdate int NOT NULL DEFAULT '0',
  update_frequency tinyint NOT NULL DEFAULT '0',
  priority tinyint NOT NULL DEFAULT '0',
  language varchar(8) NOT NULL DEFAULT '' ,
  detected_language varchar(8) NOT NULL DEFAULT '' ,
  chash varchar(40)  NOT NULL DEFAULT '',
  oldest_item_pubdate int NOT NULL DEFAULT '0',
  item_count int NOT NULL DEFAULT '0',
  popularity int NOT NULL DEFAULT '0',
  podcast_chapters varchar(768)  NOT NULL DEFAULT '',
  podcast_locked tinyint NOT NULL DEFAULT '0',
  podcast_owner varchar(255) NOT NULL DEFAULT ''
);

insert into newsfeeds (id, url, content, description) values (null, 'https://iono.fm/rss/chan/6203', '', '');
insert into newsfeeds (id, url, content, description) values (null, 'https://feeds.podcastindex.org/pc20.xml', '', '');

-- --------------------------------------------------------

--
-- Table structure for table nfcategories
--

CREATE TABLE nfcategories (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  catid1 bigint DEFAULT NULL,
  catid2 bigint DEFAULT NULL,
  catid3 bigint DEFAULT NULL,
  catid4 bigint DEFAULT NULL,
  catid5 bigint DEFAULT NULL,
  catid6 bigint DEFAULT NULL,
  catid7 bigint DEFAULT NULL,
  catid8 bigint DEFAULT NULL,
  catid9 bigint DEFAULT NULL,
  catid10 bigint DEFAULT NULL
);

-- --------------------------------------------------------

--
-- Table structure for table nfenclosures
--

CREATE TABLE nfenclosures (
  id integer PRIMARY KEY AUTOINCREMENT,
  itemid bigint NOT NULL ,
  url varchar(2048) NOT NULL ,
  mimetype varchar(64) NOT NULL ,
  length bigint NOT NULL ,
  time datetime NOT NULL ,
  type int NOT NULL ,
  marker int NOT NULL ,
  source int NOT NULL
);

-- --------------------------------------------------------

--
-- Table structure for table nfetags
--

CREATE TABLE nfetags (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  updatedon int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table nffunding
--

CREATE TABLE nffunding (
  feedid bigint NOT NULL,
  url varchar(768)  NOT NULL DEFAULT '',
  message varchar(255) NOT NULL DEFAULT ''
);

-- --------------------------------------------------------

--
-- Table structure for table nfguids
--

CREATE TABLE nfguids (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  guid varchar(36)
);

-- --------------------------------------------------------

--
-- Table structure for table nfhashes
--

CREATE TABLE nfhashes (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  hash varchar(40)  NOT NULL DEFAULT '',
  updatedon int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table nfimages
--

CREATE TABLE nfimages (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  crc32 int UNSIGNED NOT NULL DEFAULT '0',
  type tinyint UNSIGNED NOT NULL DEFAULT '0' ,
  resolution int UNSIGNED NOT NULL DEFAULT '300'
);

-- --------------------------------------------------------

--
-- Table structure for table nfitems
--

CREATE TABLE nfitems (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  title varchar(1024) NOT NULL DEFAULT '',
  link varchar(768),
  description longtext NOT NULL,
  guid varchar(740)  NOT NULL DEFAULT '' ,
  timestamp bigint NOT NULL DEFAULT '0',
  timeadded int NOT NULL DEFAULT '0',
  enclosure_url varchar(768),
  enclosure_length bigint NOT NULL DEFAULT '0',
  enclosure_type varchar(128) NOT NULL DEFAULT 'audio/mpeg',
  itunes_episode int DEFAULT NULL,
  itunes_episode_type varchar(12) DEFAULT NULL,
  itunes_explicit tinyint DEFAULT NULL,
  itunes_duration int DEFAULT NULL,
  image varchar(768),
  purge tinyint NOT NULL DEFAULT '0',
  itunes_season int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table nfitem_chapters
--

CREATE TABLE nfitem_chapters (
  itemid bigint NOT NULL,
  url varchar(768) NOT NULL DEFAULT '',
  type tinyint NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table nfitem_images
--

CREATE TABLE nfitem_images (
  id integer PRIMARY KEY AUTOINCREMENT,
  episodeid bigint NOT NULL,
  crc32 int UNSIGNED NOT NULL DEFAULT '0',
  type tinyint UNSIGNED NOT NULL DEFAULT '0' ,
  resolution int UNSIGNED NOT NULL DEFAULT '300'
);

-- --------------------------------------------------------

--
-- Table structure for table nfitem_persons
--

CREATE TABLE nfitem_persons (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  itemid bigint NOT NULL,
  name varchar(128) NOT NULL DEFAULT '',
  role varchar(128) NOT NULL DEFAULT 'host',
  grp varchar(128) NOT NULL DEFAULT 'cast',
  img varchar(768),
  href varchar(768)
);

-- --------------------------------------------------------

--
-- Table structure for table nfitem_socialinteract
--

CREATE TABLE nfitem_socialinteract (
  id integer PRIMARY KEY AUTOINCREMENT,
  itemid bigint NOT NULL,
  uri varchar(760)  NOT NULL DEFAULT '',
  protocol tinyint NOT NULL DEFAULT '0',
  accountId varchar(128)  NOT NULL DEFAULT '',
  accountUrl varchar(768)  NOT NULL DEFAULT '',
  priority int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table nfitem_soundbites
--

CREATE TABLE nfitem_soundbites (
  itemid bigint NOT NULL,
  title varchar(512) NOT NULL DEFAULT '',
  start_time int NOT NULL DEFAULT '0',
  duration int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table nfitem_transcripts
--

CREATE TABLE nfitem_transcripts (
  itemid bigint NOT NULL,
  url varchar(768)  NOT NULL DEFAULT '',
  type tinyint NOT NULL DEFAULT '0',
  captions tinyint NOT NULL DEFAULT '0',
  language varchar(40) NOT NULL DEFAULT ''
);

-- --------------------------------------------------------

--
-- Table structure for table nfitem_value
--

CREATE TABLE nfitem_value (
  itemid bigint NOT NULL,
  value_block text  NOT NULL,
  type tinyint NOT NULL DEFAULT '0' ,
  createdon int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table nflinkage
--

CREATE TABLE nflinkage (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  itunes varchar(400)  NOT NULL DEFAULT '',
  google varchar(400)  NOT NULL DEFAULT '',
  spotify varchar(400)  NOT NULL DEFAULT '',
  stitcher varchar(400)  NOT NULL DEFAULT '',
  luminary varchar(400)  NOT NULL DEFAULT '',
  bullhorn varchar(400)  NOT NULL DEFAULT '',
  iheartradio varchar(400)  NOT NULL DEFAULT '',
  ivoox varchar(400)  NOT NULL DEFAULT '',
  amazon varchar(400)  NOT NULL DEFAULT ''
);

-- --------------------------------------------------------

--
-- Table structure for table nfliveitems
--

CREATE TABLE nfliveitems (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  title varchar(1024) NOT NULL DEFAULT '',
  link varchar(768),
  description longtext NOT NULL,
  guid varchar(740)  NOT NULL DEFAULT '' ,
  timestamp bigint NOT NULL DEFAULT '0',
  timeadded int NOT NULL DEFAULT '0',
  enclosure_url varchar(768),
  enclosure_length bigint NOT NULL DEFAULT '0',
  enclosure_type varchar(128) NOT NULL DEFAULT 'audio/mpeg',
  itunes_explicit tinyint DEFAULT NULL,
  image varchar(768),
  purge tinyint NOT NULL DEFAULT '0',
  start_time int NOT NULL DEFAULT '0',
  end_time int NOT NULL DEFAULT '0',
  status tinyint NOT NULL DEFAULT '0' ,
  content_link varchar(768)
);

-- --------------------------------------------------------

--
-- Table structure for table nflocations
--

CREATE TABLE nflocations (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  locale varchar(255) NOT NULL DEFAULT '',
  osm varchar(128) NOT NULL DEFAULT '',
  latlon varchar(128) NOT NULL DEFAULT ''
);

-- --------------------------------------------------------

--
-- Table structure for table nfmediums
--

CREATE TABLE nfmediums (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  medium varchar(36)
);

-- --------------------------------------------------------

--
-- Table structure for table nfpersons
--

CREATE TABLE nfpersons (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  role tinyint NOT NULL DEFAULT '0',
  name varchar(255) NOT NULL DEFAULT '',
  image varchar(768)  NOT NULL DEFAULT '',
  href varchar(768)  NOT NULL DEFAULT ''
);

-- --------------------------------------------------------

--
-- Table structure for table nfproblematic
--

CREATE TABLE nfproblematic (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  reason tinyint NOT NULL DEFAULT '0',
  updatedon int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table nfpublish
--

CREATE TABLE nfpublish (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  pub_time int NOT NULL,
  pub_dow int NOT NULL,
  pub_dom int NOT NULL,
  pub_slice int NOT NULL
);

-- --------------------------------------------------------

--
-- Table structure for table nfschedule
--

CREATE TABLE nfschedule (
  feedid bigint NOT NULL,
  sun tinyint NOT NULL,
  mon tinyint NOT NULL,
  tue tinyint NOT NULL,
  wed tinyint NOT NULL,
  thu tinyint NOT NULL,
  fri tinyint NOT NULL,
  sat tinyint NOT NULL
);

-- --------------------------------------------------------

--
-- Table structure for table nfsoundbites
--

CREATE TABLE nfsoundbites (
  feedid bigint NOT NULL,
  url varchar(768)  NOT NULL DEFAULT '',
  message varchar(255) NOT NULL DEFAULT ''
);

-- --------------------------------------------------------

--
-- Table structure for table nfsphinx
--

CREATE TABLE nfsphinx (
  feedid bigint NOT NULL,
  node varchar(128),
  updatedon int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table nfsubscriptions
--

CREATE TABLE nfsubscriptions (
  apitoken bigint NOT NULL,
  subscriberid bigint NOT NULL,
  feedid bigint NOT NULL,
  updated int NOT NULL
);

-- --------------------------------------------------------

--
-- Table structure for table nfvalue
--

CREATE TABLE nfvalue (
  feedid bigint NOT NULL,
  value_block text  NOT NULL,
  type tinyint NOT NULL DEFAULT '0' ,
  createdon int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table owners
--

CREATE TABLE owners (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  email varchar(512) NOT NULL
);

-- --------------------------------------------------------

--
-- Table structure for table podcasts
--

CREATE TABLE podcasts (
  id integer PRIMARY KEY AUTOINCREMENT,
  ownerid bigint NOT NULL,
  upid varchar(20)  NOT NULL DEFAULT '',
  feedid bigint NOT NULL,
  createdon int NOT NULL DEFAULT '0',
  validation_code varchar(40) NOT NULL DEFAULT ''
);

-- --------------------------------------------------------

--
-- Table structure for table prefs
--

CREATE TABLE prefs (
  id integer PRIMARY KEY AUTOINCREMENT,
  maxfiles int NOT NULL,
  avatarurl varchar(767),
  timezone varchar(64)
);

-- --------------------------------------------------------

--
-- Table structure for table pubsub
--

CREATE TABLE pubsub (
  id integer PRIMARY KEY AUTOINCREMENT,
  feedid bigint NOT NULL,
  hub_url varchar(768) NOT NULL DEFAULT '',
  self_url varchar(768) NOT NULL DEFAULT '',
  lease_expire int NOT NULL DEFAULT '0'
);

-- --------------------------------------------------------

--
-- Table structure for table sessions
--

CREATE TABLE sessions (
  id varchar(128),
  userid int NOT NULL,
  lastactivity int NOT NULL,
  created int NOT NULL,
  firstsourceip varchar(24),
  lastsourceip varchar(24),
  firstbrowser varchar(128),
  lastbrowser varchar(128),
  type int NOT NULL
);

-- --------------------------------------------------------

--
-- Table structure for table subgenres
--

CREATE TABLE subgenres (
  id integer PRIMARY KEY AUTOINCREMENT,
  title varchar(256)
);

-- --------------------------------------------------------

--
-- Table structure for table tos_accept
--

CREATE TABLE tos_accept (
  id integer PRIMARY KEY AUTOINCREMENT,
  userid int NOT NULL,
  date int NOT NULL,
  version int NOT NULL,
  ip_address varchar(40) NOT NULL,
  browser varchar(255) NOT NULL
);

-- --------------------------------------------------------

--
-- Table structure for table users
--

CREATE TABLE users (
  id integer PRIMARY KEY AUTOINCREMENT,
  name varchar(128),
  password varchar(128),
  email varchar(128),
  active tinyint NOT NULL DEFAULT '0',
  admin tinyint NOT NULL DEFAULT '0',
  badlogins tinyint NOT NULL DEFAULT '0',
  stage tinyint NOT NULL DEFAULT '0',
  lastpasschange datetime NOT NULL,
  username varchar(64),
  totpseed varchar(40),
  lastlogin datetime NOT NULL,
  system tinyint NOT NULL DEFAULT '0' ,
  developer bigint DEFAULT NULL ,
  tos tinyint NOT NULL DEFAULT '0'
);

--
-- Indexes for dumped tables
--

--
-- Indexes for table api_tokens
--
CREATE INDEX api_tokens_userid ON api_tokens(userid);
CREATE INDEX api_tokens_createdon ON api_tokens(createdon);
CREATE INDEX api_tokens_permlevel ON api_tokens(permlevel);
CREATE INDEX api_tokens_rate_limited ON api_tokens(rate_limited);

--
-- Indexes for table categories
--
CREATE UNIQUE INDEX categories_name ON categories(name);

--
-- Indexes for table category_map
--
CREATE UNIQUE INDEX category_map_categoryid_2 ON category_map(categoryid,feedid);
CREATE INDEX category_map_categoryid ON category_map(categoryid);
CREATE INDEX category_map_feedid ON category_map(feedid);

--
-- Indexes for table developers
--
CREATE UNIQUE INDEX developers_email ON developers(email);

--
-- Indexes for table directory_apple
--
CREATE UNIQUE INDEX directory_apple_itunes_id ON directory_apple(itunes_id);
CREATE INDEX directory_apple_description ON directory_apple(description);
CREATE INDEX directory_apple_title_non_english ON directory_apple(title_non_english);
CREATE INDEX directory_apple_feed_url ON directory_apple(feed_url);

--
-- Indexes for table feeds_added
--
CREATE INDEX feeds_added_feedid ON feeds_added(feedid);
CREATE INDEX feeds_added_userid ON feeds_added(userid);
CREATE INDEX feeds_added_time_added ON feeds_added(time_added);
CREATE INDEX feeds_added_source ON feeds_added(source);
CREATE INDEX feeds_added_processed ON feeds_added(processed);
CREATE INDEX feeds_added_developerid ON feeds_added(developerid);
CREATE INDEX feeds_added_stage ON feeds_added(stage);

--
-- Indexes for table flags
--
--ALTER TABLE flags

--
-- Indexes for table genres
--
CREATE UNIQUE INDEX genres_title ON genres(title);
CREATE INDEX genres_subgenre ON genres(subgenre);
CREATE INDEX genres_itunes_genre_id ON genres(itunes_genre_id);

--
-- Indexes for table newsfeeds
--
CREATE UNIQUE INDEX newsfeeds_url ON newsfeeds(url);
CREATE INDEX newsfeeds_title ON newsfeeds(title);
CREATE INDEX newsfeeds_itunes_id ON newsfeeds(itunes_id);
CREATE INDEX newsfeeds_updated ON newsfeeds(updated);
CREATE INDEX newsfeeds_errors ON newsfeeds(errors);
CREATE INDEX newsfeeds_lasthttpstatus ON newsfeeds(lasthttpstatus);
CREATE INDEX newsfeeds_lastgoodhttpstatus ON newsfeeds(lastgoodhttpstatus);
CREATE INDEX newsfeeds_dead ON newsfeeds(dead);
CREATE INDEX newsfeeds_original_url ON newsfeeds(original_url);
CREATE INDEX newsfeeds_lastcheck ON newsfeeds(lastcheck);
CREATE INDEX newsfeeds_lastupdate ON newsfeeds(lastupdate);
CREATE INDEX newsfeeds_pullnow ON newsfeeds(pullnow);
CREATE INDEX newsfeeds_parsenow ON newsfeeds(parsenow);
CREATE INDEX newsfeeds_newest_item_pubdate ON newsfeeds(newest_item_pubdate);
CREATE INDEX newsfeeds_update_frequency ON newsfeeds(update_frequency);
CREATE INDEX newsfeeds_language ON newsfeeds(language);
CREATE INDEX newsfeeds_priority ON newsfeeds(priority);
CREATE INDEX newsfeeds_chash ON newsfeeds(chash);
CREATE INDEX newsfeeds_item_count ON newsfeeds(item_count);
CREATE INDEX newsfeeds_podcast_locked ON newsfeeds(podcast_locked);
CREATE INDEX newsfeeds_podcast_owner ON newsfeeds(podcast_owner);

--
-- Indexes for table nfcategories
--
CREATE UNIQUE INDEX nfcategories_feedid ON nfcategories(feedid);
CREATE INDEX nfcategories_catid1 ON nfcategories(catid1);
CREATE INDEX nfcategories_catid2 ON nfcategories(catid2);
CREATE INDEX nfcategories_catid3 ON nfcategories(catid3);
CREATE INDEX nfcategories_catid4 ON nfcategories(catid4);
CREATE INDEX nfcategories_catid5 ON nfcategories(catid5);
CREATE INDEX nfcategories_catid6 ON nfcategories(catid6);
CREATE INDEX nfcategories_catid7 ON nfcategories(catid7);
CREATE INDEX nfcategories_catid8 ON nfcategories(catid8);
CREATE INDEX nfcategories_catid9 ON nfcategories(catid9);
CREATE INDEX nfcategories_catid10 ON nfcategories(catid10);

--
-- Indexes for table nfenclosures
--
CREATE INDEX nfenclosures_iid ON nfenclosures(itemid);
CREATE INDEX nfenclosures_type ON nfenclosures(type);
CREATE INDEX nfenclosures_time ON nfenclosures(time);

--
-- Indexes for table nfetags
--
CREATE UNIQUE INDEX nfetags_feedid ON nfetags(feedid);
CREATE INDEX nfetags_updatedon ON nfetags(updatedon);

--
-- Indexes for table nffunding
--
CREATE INDEX nffunding_url ON nffunding(url);

--
-- Indexes for table nfguids
--
CREATE UNIQUE INDEX nfguids_feedid ON nfguids(feedid);
CREATE INDEX nfguids_guid ON nfguids(guid);

--
-- Indexes for table nfhashes
--
CREATE UNIQUE INDEX nfhashes_feedid ON nfhashes(feedid);
CREATE INDEX nfhashes_hash ON nfhashes(hash);
CREATE INDEX nfhashes_updatedon ON nfhashes(updatedon);

--
-- Indexes for table nfimages
--
CREATE UNIQUE INDEX nfimages_crc32_2 ON nfimages(crc32,feedid,id,resolution);
CREATE INDEX nfimages_feedid ON nfimages(feedid);
CREATE INDEX nfimages_crc32 ON nfimages(crc32);
CREATE INDEX nfimages_resolution ON nfimages(resolution);
CREATE INDEX nfimages_type ON nfimages(type);

--
-- Indexes for table nfitems
--
CREATE UNIQUE INDEX nfitems_feedid_2 ON nfitems(feedid,guid);
CREATE INDEX nfitems_timeadded ON nfitems(timeadded);
CREATE INDEX nfitems_feedid ON nfitems(feedid);
CREATE INDEX nfitems_timestamp ON nfitems(timestamp);
CREATE INDEX nfitems_purgeable ON nfitems(purge);

--
-- Indexes for table nfitem_chapters
--
CREATE INDEX nfitem_chapters_type ON nfitem_chapters(type);

--
-- Indexes for table nfitem_images
--
CREATE UNIQUE INDEX nfitem_images_crc32_2 ON nfitem_images(crc32,episodeid,id,resolution);
CREATE INDEX nfitem_images_crc32 ON nfitem_images(crc32);
CREATE INDEX nfitem_images_resolution ON nfitem_images(resolution);
CREATE INDEX nfitem_images_type ON nfitem_images(type);
CREATE INDEX nfitem_images_episodeid ON nfitem_images(episodeid);

--
-- Indexes for table nfitem_persons
--
CREATE UNIQUE INDEX nfitem_persons_itemid_2 ON nfitem_persons(itemid,name,href);
CREATE INDEX nfitem_persons_itemid ON nfitem_persons(itemid);
CREATE INDEX nfitem_persons_href ON nfitem_persons(href);
CREATE INDEX nfitem_persons_grp ON nfitem_persons(grp);
CREATE INDEX nfitem_persons_role ON nfitem_persons(role);
CREATE INDEX nfitem_persons_name ON nfitem_persons(name);

--
-- Indexes for table nfitem_socialinteract
--
CREATE UNIQUE INDEX nfitem_socialinteract_itemid_2 ON nfitem_socialinteract(itemid,uri);
CREATE INDEX nfitem_socialinteract_priority ON nfitem_socialinteract(priority);
CREATE INDEX nfitem_socialinteract_itemid ON nfitem_socialinteract(itemid);
CREATE INDEX nfitem_socialinteract_protocol ON nfitem_socialinteract(protocol);
CREATE INDEX nfitem_socialinteract_accountId ON nfitem_socialinteract(accountId);

--
-- Indexes for table nfitem_soundbites
--
CREATE INDEX nfitem_soundbites_itemid ON nfitem_soundbites(itemid);
CREATE INDEX nfitem_soundbites_duration ON nfitem_soundbites(duration);

--
-- Indexes for table nfitem_transcripts
--
CREATE UNIQUE INDEX nfitem_transcripts_itemid_2 ON nfitem_transcripts(itemid,type,language);
CREATE INDEX nfitem_transcripts_type ON nfitem_transcripts(type);
CREATE INDEX nfitem_transcripts_captions ON nfitem_transcripts(captions);
CREATE INDEX nfitem_transcripts_language ON nfitem_transcripts(language);
CREATE INDEX nfitem_transcripts_itemid ON nfitem_transcripts(itemid);

--
-- Indexes for table nfitem_value
--
CREATE INDEX nfitem_value_type ON nfitem_value(type);
CREATE INDEX nfitem_value_createdon ON nfitem_value(createdon);

--
-- Indexes for table nflinkage
--
CREATE INDEX nflinkage_feedid ON nflinkage(feedid);

--
-- Indexes for table nfliveitems
--
CREATE UNIQUE INDEX nfliveitems_feedid_2 ON nfliveitems(feedid,guid);
CREATE INDEX nfliveitems_timeadded ON nfliveitems(timeadded);
CREATE INDEX nfliveitems_feedid ON nfliveitems(feedid);
CREATE INDEX nfliveitems_timestamp ON nfliveitems(timestamp);
CREATE INDEX nfliveitems_purgeable ON nfliveitems(purge);
CREATE INDEX nfliveitems_startTime ON nfliveitems(start_time);
CREATE INDEX nfliveitems_status ON nfliveitems(status);

--
-- Indexes for table nflocations
--
CREATE UNIQUE INDEX nflocations_feedid ON nflocations(feedid);

--
-- Indexes for table nfmediums
--
CREATE UNIQUE INDEX nfmediums_feedid ON nfmediums(feedid);
CREATE INDEX nfmediums_medium ON nfmediums(medium);

--
-- Indexes for table nfpersons
--
CREATE INDEX nfpersons_role ON nfpersons(role);
CREATE INDEX nfpersons_name ON nfpersons(name);
CREATE INDEX nfpersons_feedid ON nfpersons(feedid);

--
-- Indexes for table nfproblematic
--
CREATE UNIQUE INDEX nfproblematic_feedid ON nfproblematic(feedid);
CREATE INDEX nfproblematic_hash ON nfproblematic(reason);
CREATE INDEX nfproblematic_updatedon ON nfproblematic(updatedon);

--
-- Indexes for table nfpublish
--
CREATE INDEX nfpublish_feedid ON nfpublish(feedid);
CREATE INDEX nfpublish_pub_time ON nfpublish(pub_time);
CREATE INDEX nfpublish_pub_dow ON nfpublish(pub_dow);
CREATE INDEX nfpublish_pub_dom ON nfpublish(pub_dom);
CREATE INDEX nfpublish_pub_slice ON nfpublish(pub_slice);

--
-- Indexes for table nfschedule
--
CREATE INDEX nfschedule_sun ON nfschedule(sun);
CREATE INDEX nfschedule_mon ON nfschedule(mon);
CREATE INDEX nfschedule_tue ON nfschedule(tue);
CREATE INDEX nfschedule_wed ON nfschedule(wed);
CREATE INDEX nfschedule_thu ON nfschedule(thu);
CREATE INDEX nfschedule_fri ON nfschedule(fri);
CREATE INDEX nfschedule_sat ON nfschedule(sat);

--
-- Indexes for table nfsoundbites
--
CREATE INDEX nfsoundbites_url ON nfsoundbites(url);

--
-- Indexes for table nfsphinx
--
CREATE INDEX nfsphinx_updatedon ON nfsphinx(updatedon);

--
-- Indexes for table nfsubscriptions
--
CREATE INDEX nfsubscriptions_apitoken ON nfsubscriptions(apitoken);
CREATE INDEX nfsubscriptions_subscriberid ON nfsubscriptions(subscriberid);
CREATE INDEX nfsubscriptions_feedid ON nfsubscriptions(feedid);
CREATE INDEX nfsubscriptions_updated ON nfsubscriptions(updated);

--
-- Indexes for table nfvalue
--
CREATE INDEX nfvalue_type ON nfvalue(type);
CREATE INDEX nfvalue_createdon ON nfvalue(createdon);

--
-- Indexes for table owners
--
CREATE INDEX owners_email ON owners(email);
CREATE INDEX owners_feedid ON owners(feedid);

--
-- Indexes for table podcasts
--
CREATE UNIQUE INDEX podcasts_upid ON podcasts(upid);
CREATE UNIQUE INDEX podcasts_feedid ON podcasts(feedid);
CREATE UNIQUE INDEX podcasts_owner_and_feed ON podcasts(ownerid,feedid);
CREATE INDEX podcasts_ownerid ON podcasts(ownerid);
CREATE INDEX podcasts_validation_code ON podcasts(validation_code);

--
-- Indexes for table prefs
--

--
-- Indexes for table pubsub
--
CREATE UNIQUE INDEX pubsub_feedid ON pubsub(feedid);
CREATE INDEX pubsub_last_sub_time ON pubsub(lease_expire);
CREATE INDEX pubsub_sub_url ON pubsub(hub_url);

--
-- Indexes for table sessions
--
CREATE INDEX sessions_userid ON sessions(userid);

--
-- Indexes for table subgenres
--
CREATE UNIQUE INDEX subgenres_title ON subgenres(title);

--
-- Indexes for table tos_accept
--
CREATE UNIQUE INDEX tos_accept_userid_2 ON tos_accept(userid,version);
CREATE INDEX tos_accept_userid ON tos_accept(userid);
CREATE INDEX tos_accept_version ON tos_accept(version);

--
-- Indexes for table users
--
CREATE INDEX users_developer ON users(developer);
CREATE INDEX users_tos ON users(tos);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table api_tokens
--
ALTER TABLE api_tokens
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table categories
--
ALTER TABLE categories
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table category_map
--
ALTER TABLE category_map
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table developers
--
ALTER TABLE developers
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table directory_apple
--
ALTER TABLE directory_apple
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table feeds_added
--
ALTER TABLE feeds_added
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table newsfeeds
--
ALTER TABLE newsfeeds
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfcategories
--
ALTER TABLE nfcategories
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfenclosures
--
ALTER TABLE nfenclosures
  MODIFY id bigint NOT NULL AUTO_INCREMENT ,

--
-- AUTO_INCREMENT for table nfetags
--
ALTER TABLE nfetags
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfguids
--
ALTER TABLE nfguids
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfhashes
--
ALTER TABLE nfhashes
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfimages
--
ALTER TABLE nfimages
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfitems
--
ALTER TABLE nfitems
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfitem_images
--
ALTER TABLE nfitem_images
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfitem_persons
--
ALTER TABLE nfitem_persons
  MODIFY id bigint UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfitem_socialinteract
--
ALTER TABLE nfitem_socialinteract
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nflinkage
--
ALTER TABLE nflinkage
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfliveitems
--
ALTER TABLE nfliveitems
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nflocations
--
ALTER TABLE nflocations
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfmediums
--
ALTER TABLE nfmediums
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfpersons
--
ALTER TABLE nfpersons
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfproblematic
--
ALTER TABLE nfproblematic
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table nfpublish
--
ALTER TABLE nfpublish
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table owners
--
ALTER TABLE owners
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table podcasts
--
ALTER TABLE podcasts
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table pubsub
--
ALTER TABLE pubsub
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table subgenres
--
ALTER TABLE subgenres
  MODIFY id int NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table tos_accept
--
ALTER TABLE tos_accept
  MODIFY id bigint NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table users
--
ALTER TABLE users
  MODIFY id int NOT NULL AUTO_INCREMENT;

--
-- Constraints for dumped tables
--

--
-- Constraints for table api_tokens
--
ALTER TABLE api_tokens
  ADD CONSTRAINT api_tokens_ibfk_1 FOREIGN KEY (userid) REFERENCES developers (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table category_map
--
ALTER TABLE category_map
  ADD CONSTRAINT category_map_ibfk_1 FOREIGN KEY (categoryid) REFERENCES categories (id) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT category_map_ibfk_2 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table feeds_added
--
ALTER TABLE feeds_added
  ADD CONSTRAINT feeds_added_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT feeds_added_ibfk_2 FOREIGN KEY (userid) REFERENCES users (id) ON DELETE SET NULL ON UPDATE CASCADE,
  ADD CONSTRAINT feeds_added_ibfk_3 FOREIGN KEY (developerid) REFERENCES developers (id) ON DELETE SET NULL ON UPDATE CASCADE;

--
-- Constraints for table genres
--
ALTER TABLE genres
  ADD CONSTRAINT subgenre FOREIGN KEY (subgenre) REFERENCES genres (id) ON DELETE RESTRICT ON UPDATE CASCADE;

--
-- Constraints for table newsfeeds
--
ALTER TABLE newsfeeds
  ADD CONSTRAINT newsfeeds_ibfk_3 FOREIGN KEY (itunes_id) REFERENCES directory_apple (itunes_id) ON DELETE SET NULL ON UPDATE CASCADE;

--
-- Constraints for table nfcategories
--
ALTER TABLE nfcategories
  ADD CONSTRAINT nfcategories_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT nfcategories_ibfk_10 FOREIGN KEY (catid9) REFERENCES categories (id) ON DELETE SET NULL ON UPDATE CASCADE,
  ADD CONSTRAINT nfcategories_ibfk_11 FOREIGN KEY (catid10) REFERENCES categories (id) ON DELETE SET NULL ON UPDATE CASCADE,
  ADD CONSTRAINT nfcategories_ibfk_2 FOREIGN KEY (catid1) REFERENCES categories (id) ON DELETE SET NULL ON UPDATE CASCADE,
  ADD CONSTRAINT nfcategories_ibfk_3 FOREIGN KEY (catid2) REFERENCES categories (id) ON DELETE SET NULL ON UPDATE CASCADE,
  ADD CONSTRAINT nfcategories_ibfk_4 FOREIGN KEY (catid3) REFERENCES categories (id) ON DELETE SET NULL ON UPDATE CASCADE,
  ADD CONSTRAINT nfcategories_ibfk_5 FOREIGN KEY (catid4) REFERENCES categories (id) ON DELETE SET NULL ON UPDATE CASCADE,
  ADD CONSTRAINT nfcategories_ibfk_6 FOREIGN KEY (catid5) REFERENCES categories (id) ON DELETE SET NULL ON UPDATE CASCADE,
  ADD CONSTRAINT nfcategories_ibfk_7 FOREIGN KEY (catid6) REFERENCES categories (id) ON DELETE SET NULL ON UPDATE CASCADE,
  ADD CONSTRAINT nfcategories_ibfk_8 FOREIGN KEY (catid7) REFERENCES categories (id) ON DELETE SET NULL ON UPDATE CASCADE,
  ADD CONSTRAINT nfcategories_ibfk_9 FOREIGN KEY (catid8) REFERENCES categories (id) ON DELETE SET NULL ON UPDATE CASCADE;

--
-- Constraints for table nfenclosures
--
ALTER TABLE nfenclosures
  ADD CONSTRAINT nfenclosures_ibfk_1 FOREIGN KEY (itemid) REFERENCES nfitems (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nffunding
--
ALTER TABLE nffunding
  ADD CONSTRAINT nffunding_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfguids
--
ALTER TABLE nfguids
  ADD CONSTRAINT nfguids_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfhashes
--
ALTER TABLE nfhashes
  ADD CONSTRAINT nfhashes_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfimages
--
ALTER TABLE nfimages
  ADD CONSTRAINT nfimages_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfitems
--
ALTER TABLE nfitems
  ADD CONSTRAINT nfitems_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfitem_chapters
--
ALTER TABLE nfitem_chapters
  ADD CONSTRAINT nfitem_chapters_ibfk_1 FOREIGN KEY (itemid) REFERENCES nfitems (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfitem_images
--
ALTER TABLE nfitem_images
  ADD CONSTRAINT nfitem_images_ibfk_1 FOREIGN KEY (episodeid) REFERENCES nfitems (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfitem_persons
--
ALTER TABLE nfitem_persons
  ADD CONSTRAINT nfitem_persons_ibfk_1 FOREIGN KEY (itemid) REFERENCES nfitems (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfitem_socialinteract
--
ALTER TABLE nfitem_socialinteract
  ADD CONSTRAINT nfitem_socialinteract_ibfk_1 FOREIGN KEY (itemid) REFERENCES nfitems (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfitem_soundbites
--
ALTER TABLE nfitem_soundbites
  ADD CONSTRAINT nfitem_soundbites_ibfk_1 FOREIGN KEY (itemid) REFERENCES nfitems (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfitem_transcripts
--
ALTER TABLE nfitem_transcripts
  ADD CONSTRAINT nfitem_transcripts_ibfk_1 FOREIGN KEY (itemid) REFERENCES nfitems (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfitem_value
--
ALTER TABLE nfitem_value
  ADD CONSTRAINT nfitem_value_ibfk_1 FOREIGN KEY (itemid) REFERENCES nfitems (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfliveitems
--
ALTER TABLE nfliveitems
  ADD CONSTRAINT nfliveitems_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nflocations
--
ALTER TABLE nflocations
  ADD CONSTRAINT nflocations_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfmediums
--
ALTER TABLE nfmediums
  ADD CONSTRAINT nfmediums_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfproblematic
--
ALTER TABLE nfproblematic
  ADD CONSTRAINT nfproblematic_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfpublish
--
ALTER TABLE nfpublish
  ADD CONSTRAINT nfpublish_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfschedule
--
ALTER TABLE nfschedule
  ADD CONSTRAINT nfschedule_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfsphinx
--
ALTER TABLE nfsphinx
  ADD CONSTRAINT nfsphinx_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfsubscriptions
--
ALTER TABLE nfsubscriptions
  ADD CONSTRAINT nfsubscriptions_ibfk_1 FOREIGN KEY (apitoken) REFERENCES api_tokens (id) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT nfsubscriptions_ibfk_2 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table nfvalue
--
ALTER TABLE nfvalue
  ADD CONSTRAINT nfvalue_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table owners
--
ALTER TABLE owners
  ADD CONSTRAINT owners_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table podcasts
--
ALTER TABLE podcasts
  ADD CONSTRAINT podcasts_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE RESTRICT ON UPDATE RESTRICT,
  ADD CONSTRAINT podcasts_ibfk_2 FOREIGN KEY (ownerid) REFERENCES owners (id) ON DELETE RESTRICT ON UPDATE RESTRICT;

--
-- Constraints for table prefs
--
ALTER TABLE prefs
  ADD CONSTRAINT prefs_ibfk_1 FOREIGN KEY (id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table pubsub
--
ALTER TABLE pubsub
  ADD CONSTRAINT pubsub_ibfk_1 FOREIGN KEY (feedid) REFERENCES newsfeeds (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table sessions
--
ALTER TABLE sessions
  ADD CONSTRAINT sessions_ibfk_1 FOREIGN KEY (userid) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table tos_accept
--
ALTER TABLE tos_accept
  ADD CONSTRAINT tos_accept_ibfk_1 FOREIGN KEY (userid) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table users
--
ALTER TABLE users
  ADD CONSTRAINT users_ibfk_1 FOREIGN KEY (developer) REFERENCES developers (id) ON DELETE SET NULL ON UPDATE CASCADE;
