use linkify::LinkFinder;
use serenity::model::channel::Message;

#[derive(Default)]
pub struct URLExtractInfo {
    pub urls: Vec<String>,
    pub n_attachments: u8,
    pub n_links: u16,
    pub rn_attachments: Option<u8>,
    pub rn_links: Option<u16>,
}

pub trait ExtractInfo {
    fn extract_urls(&self) -> URLExtractInfo;
}

// Priority: Text > Attachments > Reference
impl ExtractInfo for Message {
    fn extract_urls(&self) -> URLExtractInfo {
        let mut ret = URLExtractInfo::default();
        let finder = LinkFinder::new();
        let find_links = |x| finder.links(x).map(|x| x.as_str().to_string()).collect();

        ret.urls = find_links(&self.content);

        ret.n_links = ret.urls.len() as u16;

        ret.urls.extend(
            self.attachments
                .iter()
                .map(|x| x.url.clone())
                .collect::<Vec<String>>(),
        );

        ret.n_attachments = self.attachments.len() as u8;

        if let Some(msg) = &self.referenced_message {
            let msg_links: Vec<String> = find_links(&msg.content);

            ret.rn_attachments = Some(msg.attachments.len() as u8);
            ret.rn_links = Some(msg_links.len() as u16);
            ret.urls.extend(msg_links);
            ret.urls.extend(
                msg.attachments
                    .iter()
                    .map(|x| x.url.clone())
                    .collect::<Vec<String>>(),
            );
        }

        ret
    }
}
