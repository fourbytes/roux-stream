use futures::{Stream, StreamExt, channel::mpsc};
use roux::{Subreddit, subreddit::responses::{SubmissionsData, SubredditCommentsData}};
use tokio;

use subreddit_dumper;


async fn submission_reader(stream: &mut (dyn Stream<Item=SubmissionsData> + Unpin)) {
    while let Some(submission) = stream.next().await {
        println!("New submission by {}", submission.author);
    }
}

async fn comment_reader(stream: &mut (dyn Stream<Item=SubredditCommentsData> + Unpin)) {
    while let Some(comment) = stream.next().await {
        println!("New comment by {}", comment.author.as_ref().unwrap());
    }
}

#[tokio::main]
async fn main() {
    let subreddit = Subreddit::new("AskReddit");

    let (mut submission_sender, mut submission_receiver) = mpsc::unbounded();
    let (mut comment_sender, mut comment_receiver) = mpsc::unbounded();

    tokio::join!(
        subreddit_dumper::stream_subreddit_submissions(&subreddit, &mut submission_sender),
        submission_reader(&mut submission_receiver),
        subreddit_dumper::stream_subreddit_comments(&subreddit, &mut comment_sender),
        comment_reader(&mut comment_receiver),
    );
}