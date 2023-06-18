use crate::api::lemmy::fetcher::Fetcher;
use super::analyizer::Analyizer;

pub struct Crawler {
    pub instance : String,
    
    fetcher : Fetcher,
    analyizer : Analyizer 
}

impl Crawler {

    pub fn new(instacne : String) -> Self {
        Crawler {
            instance: instacne.clone(),
            fetcher: Fetcher::new(instacne),
            analyizer: Analyizer::new()
        }
    }

    pub async fn crawl(
        &self
    ) {
        let number_of_comments = self.fetcher.fetch_site_data()
            .await
            .site_view
            .counts
            .comments;

        for page in 0..(number_of_comments / Fetcher::DEFAULT_LIMIT) {
            let comments = self.fetcher.fetch_comments(page)
                .await;

            for comment in comments {
                let worlds = self.analyizer.get_distinct_words_in_comment(
                    comment.comment
                );
            }
        }

        

        // let site_data = self.fetch_site_data()
        //     .await.site_view;

        // let number_of_communities = site_data.counts.communities;

        // let _ = self.fetch_all_communities(number_of_communities);
    }
}