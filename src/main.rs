#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

use {
    std::{
        convert::TryInto as _,
        io,
        num::ParseIntError,
    },
    chrono::{
        Duration,
        prelude::*,
    },
    chrono_tz::Tz,
    clap::builder::ArgPredicate,
    futures::stream::TryStreamExt as _,
    is_terminal::IsTerminal as _,
    itertools::Itertools as _,
    tokio::io::{
        AsyncBufReadExt as _,
        BufReader,
        stdin,
    },
    tokio_stream::wrappers::LinesStream,
};

const DATACENTER_ID_BITS: u8 = 5;
const WORKER_ID_BITS: u8 = 5;
const SEQUENCE_ID_BITS: u8 = 12;
const MAX_DATACENTER_ID: u64 = 1 << DATACENTER_ID_BITS;
const MAX_WORKER_ID: u64 = 1 << WORKER_ID_BITS;
const MAX_SEQUENCE_ID: u64 = 1 << SEQUENCE_ID_BITS;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] Io(#[from] io::Error),
    #[error(transparent)] ParseInt(#[from] ParseIntError),
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum DiscordFormat {
    #[clap(alias("d"))]
    ShortDate,
    #[clap(alias("D"))]
    LongDate,
    #[clap(alias("t"))]
    ShortTime,
    #[clap(alias("T"))]
    LongTime,
    #[clap(alias("f"))]
    ShortDateTime,
    #[clap(alias("F"))]
    LongDateTime,
    #[clap(alias("R"))]
    Relative,
}

#[derive(clap::Parser)]
struct Arguments {
    /// Give the timestamps in a format which if included in a Discord message will be rendered according to the viewer's locale.
    #[structopt(short, long, value_enum, conflicts_with("epoch"), conflicts_with("format"), conflicts_with("human"))]
    discord_format: Option<DiscordFormat>,
    /// The epoch on which the snowflakes are based. Can be `twitter` `discord`, or a UNIX timestamp.
    #[structopt(short, long, default_value = "twitter", default_value_if("discord-format", ArgPredicate::IsPresent, Some("discord")), value_parser = parse_epoch)]
    epoch: DateTime<Utc>,
    /// Format each timestamp like this, as defined at <https://docs.rs/chrono/0.4/chrono/format/strftime/index.html>. Additionally, %^d, %^w, and %^s can be used to insert the datacenter, worker, and sequence ID, respectively. Defaults to a UNIX timestamp with fractional part.
    #[structopt(short, long, group("allow_timezone"), default_value = "%s%.f", default_value_if("human", ArgPredicate::IsPresent, Some("%Y-%m-%d %H:%M:%S")))]
    format: String,
    /// Show the timestamps in a human-readable format. Short for `--format="%Y-%m-%d %H:%M:%S"`.
    #[structopt(short = 'H', group("allow_timezone"), conflicts_with("format"))]
    #[allow(unused)] // used in default_value_if of format arg
    human: bool,
    /// The timezone in which the timestamps will be shown.
    #[structopt(short = 'z', long, default_value = "Etc/UTC", requires("allow_timezone"))]
    timezone: Tz,
    /// An optional list of snowflakes to melt. Snowflakes can also be piped into `melt`, separated by line breaks.
    flakes: Vec<u64>,
}

struct SnowflakeParts {
    timestamp: DateTime<Utc>,
    data_center: u8,
    worker: u8,
    sequence: u16,
}

impl SnowflakeParts {
    /// Breaks a snowflake into its component parts.
    fn melt(flake: u64, epoch: DateTime<Utc>) -> SnowflakeParts {
        let sequence = (flake & (MAX_SEQUENCE_ID - 1)) as u16;
        let worker = ((flake >> SEQUENCE_ID_BITS) & (MAX_WORKER_ID - 1)) as u8;
        let data_center = ((flake >> SEQUENCE_ID_BITS >> WORKER_ID_BITS) & (MAX_DATACENTER_ID - 1)) as u8;
        let timestamp_ms = flake >> SEQUENCE_ID_BITS >> WORKER_ID_BITS >> DATACENTER_ID_BITS;
        let timestamp = epoch + Duration::milliseconds(timestamp_ms.try_into().expect("snowflake timestamp should fit into i64"));
        SnowflakeParts { timestamp, data_center, worker, sequence }
    }

    fn format(&self, format: &str, timezone: Tz) -> String {
        let format = format.split("%%")
            .map(|part| part
                .replace("%^d", &self.data_center.to_string())
                .replace("%^w", &self.worker.to_string())
                .replace("%^s", &self.sequence.to_string())
            )
            .join("%%");
        self.timestamp.with_timezone(&timezone).format(&format).to_string()
    }
}

fn parse_epoch(epoch_str: &str) -> chrono::ParseResult<DateTime<Utc>> {
    Ok(match epoch_str {
        "twitter" => Utc.timestamp_millis_opt(1288834974657).single().expect("wrong hardcoded datetime"),
        "discord" => Utc.timestamp_opt(1420070400, 0).single().expect("wrong hardcoded datetime"),
        _ => Utc.datetime_from_str(epoch_str, "%s%.f")?
    })
}

#[wheel::main]
async fn main(args: Arguments) -> Result<(), Error> {
    let format = match args.discord_format {
        Some(DiscordFormat::ShortDate) => "<t:%s:d>",
        Some(DiscordFormat::LongDate) => "<t:%s:D>",
        Some(DiscordFormat::ShortTime) => "<t:%s:t>",
        Some(DiscordFormat::LongTime) => "<t:%s:T>",
        Some(DiscordFormat::ShortDateTime) => "<t:%s:f>",
        Some(DiscordFormat::LongDateTime) => "<t:%s:F>",
        Some(DiscordFormat::Relative) => "<t:%s:R>",
        None => &args.format,
    };
    let mut flakes = args.flakes;
    {
        let stdin = stdin();
        if stdin.is_terminal() {
            flakes.extend(
                LinesStream::new(BufReader::new(stdin).lines())
                    .err_into()
                    .and_then(|line| async move { line.trim().parse::<u64>().map_err(Error::from) })
                    .try_collect::<Vec<_>>().await?
            );
        }
    }
    for flake in flakes {
        println!("{}", SnowflakeParts::melt(flake, args.epoch).format(format, args.timezone))
    }
    Ok(())
}

#[test]
fn readme_examples() {
    assert_eq!(SnowflakeParts::melt(844882040566702080, parse_epoch("twitter").unwrap()).format("%s%.f", Tz::Etc__UTC), "1490270550.277");
    assert_eq!(SnowflakeParts::melt(844882040566702080, parse_epoch("twitter").unwrap()).format("%d.%m.%Y %H:%M:%S", Tz::Etc__UTC), "23.03.2017 12:02:30");
    assert_eq!(SnowflakeParts::melt(844882040566702080, parse_epoch("twitter").unwrap()).format("%d.%m.%Y %H:%M:%S", Tz::Europe__Berlin), "23.03.2017 13:02:30");
    assert_eq!(SnowflakeParts::melt(86841168427495424, parse_epoch("discord").unwrap()).format("%s%.f", Tz::Etc__UTC), "1440774947.984");
}
