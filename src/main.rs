use regex;

#[tokio::main]
async fn main() {
    env_logger::init();

    let output = String::from_utf8(
        std::process::Command::new("ipconfig")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let caps = {
        let mut caps = None;
        for line in output.lines() {
            caps = regex::Regex::new(r##".*(?<ip>192[^\s]+).*"##)
                .unwrap()
                .captures(line);
            if caps.is_some() {
                log::info!("{}", line.trim());
                break;
            }
        }
        caps
    };

    if caps.is_none() {
        log::error!("Could not obtain the IP address");
        return;
    }

    let caps = caps.unwrap();
    let ip = &caps["ip"];

    let token = std::env::var("DUCKDNS_TOKEN")
        .unwrap_or("".to_owned())
        .trim()
        .to_owned();

    if token.is_empty() {
        log::error!("DUCKDNS_TOKEN environment variable is not specified");
        return;
    }

    let domain = std::env::var("DUCKDNS_DOMAIN")
        .unwrap_or("".to_owned())
        .trim()
        .to_owned();

    if domain.is_empty() {
        log::error!("DUCKDNS_DOMAIN environment variable is not specified");
        return;
    }

    let url = format!(
        concat!(
            "https://www.duckdns.org/update?domains={DUCKDNS_DOMAIN}",
            "&token={DUCKDNS_TOKEN}&ip={IP}"
        ),
        DUCKDNS_DOMAIN = domain,
        DUCKDNS_TOKEN = token,
        IP = ip,
    );

    log::info!("URL={}", url);
    let result = reqwest::get(url).await.unwrap().text().await.unwrap();
    log::info!("{}", result);
}
