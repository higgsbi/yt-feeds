use std::{
    process::{Command, Stdio},
    rc::Rc,
    thread,
};

use crossterm::style::Stylize;

use crate::{
    config::Config,
    loading::cmd_while_loading,
    view::{Error, Message, ViewPage},
    yt::{Channels, Video, VideoIndex},
};

use super::{View, ViewInput};

pub fn show(
    channels: &Channels,
    index: VideoIndex,
    last_view: &ViewPage,
    config: &Config,
) -> Message {
    let channel = channels.channel(index.into()).unwrap();
    let video = channel.video(index).unwrap();

    let mut view = View::new(
        format!("\"{}\" - {}", video.title, channel.name),
        "(p)lay, (d)etach, (s)ave, (i)nformation, (b)ack, (q)uit".to_owned(),
        "▶".to_owned(),
    );

    let last_view = last_view.or_inner();

    loop {
        view.clear_content();

        match view.show() {
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
                'i' => return Message::Information(index, Rc::new(last_view.clone())),
                'b' => match last_view {
                    ViewPage::FeedChannel(channel_index) => {
                        return Message::ChannelFeed(*channel_index)
                    }
                    ViewPage::MixedFeed => return Message::MixedFeed,
                    _ => panic!(),
                },
                'p' => {
                    if let Err(Error::CommandFailed(e)) = play(video) {
                        view.set_error(&format!("Could not run play command: mpv.\nError: {}", e));
                    } else {
                        view.clear_error();
                    }
                }
                's' => {
                    if let Err(Error::CommandFailed(e)) = download(video, config) {
                        view.set_error(&format!("Could not run download video\nError: {}", e));
                    } else {
                        view.clear_error();
                    }
                }
                'd' => {
                    let url = video.url();
                    thread::spawn(|| {
                        Command::new("mpv")
                            .arg(url)
                            .stdout(Stdio::null())
                            .stderr(Stdio::null())
                            .spawn()
                    });
                    view.clear_error();
                }
                input => {
                    view.set_error(&format!("{} is not a valid option!", input));
                }
            },
            ViewInput::Num(num) => {
                view.set_error(&format!("{} is not a valid option!", num));
            }
        }
    }
}

fn download(video: &Video, config: &Config) -> Result<(), Error> {
    let title = video.title.clone();
    let url = video.url();

    cmd_while_loading(
        Command::new("yt-dlp")
            .arg("-o")
            .arg(format!("{}%(title)s.%(ext)s", config.saved_video_path))
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn(),
        move || {
            print!("\r\n{}\r\n\r\n", title.as_str().cyan().bold());
            print!("{} '{}'", "Downloading ".green(), title.as_str().yellow());
        },
    )
}

fn play(video: &Video) -> Result<(), Error> {
    let title = video.title.clone();
    let url = video.url();

    cmd_while_loading(
        Command::new("mpv")
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn(),
        move || {
            print!("\r\n{}\r\n\r\n", title.as_str().cyan().bold());
            print!("{} '{}'", "Playing ".green(), title.as_str().yellow());
        },
    )
}
