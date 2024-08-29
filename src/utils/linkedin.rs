
extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
          .add(b'<').add(b'>').add(b'`');

     
pub fn construct_linkedin_url(query:&str) -> String {
    // TODO: Convert parsing checks into enums
    if query == "li" {
        let linkedin_dotcom = "https://www.linkedin.com";
        linkedin_dotcom.to_string()
    
    } else if &query[..5] == "li p " {
        construct_linkedin_profile_url(&query[5..])
    } else {
        if query.len() == 7 && &query[query.len() - 4..] == "jobs"
        {
            construct_linkedin_jobs_url()
        } else{
            construct_linkedin_search_url(&query[3..]) 
        }
    }
}

pub fn construct_linkedin_search_url(query:&str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let linkedin_search_url = format!("https://www.linkedin.com/search/results/all/?keywords={}", 
                                    encoded_query);
    linkedin_search_url

}

pub fn construct_linkedin_profile_url(query:&str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let linked_profile_url = format!("https://www.linkedin.com/in/{}", encoded_query);
    linked_profile_url
}

pub fn construct_linkedin_jobs_url() -> String {
    let linkedin_jobs_url  = "https://www.linkedin.com/jobs/".to_string();
    linkedin_jobs_url
}