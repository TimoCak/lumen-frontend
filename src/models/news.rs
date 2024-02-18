use serde::{Deserialize,Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Source {
    pub id: Option<String>,    
    pub name: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct NewsArticle {
    pub source: Source,
    pub author: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub urlToImage: Option<String>,
    pub publishedAt: Option<String>,
    pub content: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct News {
    pub status: String,
    pub totalResults: usize,
    pub articles: Vec<NewsArticle>,
}

impl News {
    pub fn set_articles(&mut self, articles: Vec<NewsArticle>) {
        self.articles = articles;
    }
}