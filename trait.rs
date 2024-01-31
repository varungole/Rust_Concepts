
pub struct Tweet
{
    pub tweet_id: i32,
    pub content: String,
}

pub struct NewsArticle 
{
   pub author: String,
}

pub trait Summary
{
    fn summarize(&self) -> String
    {
        String::from("Hello, WOrld!") //Default implementation
    }
}

impl Summary for Tweet
{
    fn summarize(&self) -> String {
        format!("{} by {}" , self.tweet_id , self.content)
    }
}


fn main()
{
 let tweet = Tweet {tweet_id : 1 , content: String::from("Varun")};

 println!("{}" , tweet.summarize());
}