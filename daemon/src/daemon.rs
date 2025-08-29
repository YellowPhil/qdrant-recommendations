use bincode::config::Config;
use eyre::{Result, WrapErr};
use std::fs::File;

use daemonize::Daemonize;
use embedding::EmbeddingModel;
use storage_client::TopicStorage;

use std::io::{self, BufReader, prelude::*};

use interprocess::local_socket::{GenericNamespaced, ListenerOptions, prelude::*};

use crate::{
    CreateTopicResponse, ListTopicResponse, Request, Response, SearchTopicResponse,
    UpdateTopicResponse,
};

pub struct Daemon<T: EmbeddingModel, C: Config> {
    storage: TopicStorage<T>,
    config: C,
    //TODO: Add persistent storage for topics
}

pub const PRINT_NAME: &str = "qdrant-cli-daemon.sock";

impl<T: EmbeddingModel, C: Config> Daemon<T, C> {
    pub fn new(storage: TopicStorage<T>, config: C) -> Self {
        Self { storage, config }
    }
    pub fn update_storage(&mut self, storage: TopicStorage<T>) {
        self.storage = storage;
    }

    /// Listen for incoming connections on the daemon socket
    ///
    /// # Warning
    ///
    /// This function runs in an infinite loop listening for connections and will
    /// not release the thread until an error occurs or the process is terminated.
    /// Consider running this in a dedicated thread or using proper async runtime
    /// management to avoid blocking the main thread.
    pub async fn listen(&self) -> Result<()> {
        let name = PRINT_NAME
            .to_ns_name::<GenericNamespaced>()
            .wrap_err("Failed to create namespaced name")?;
        let opts = ListenerOptions::new().name(name);
        let listener = match opts.create_sync() {
            Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
                eprintln!(
                    "Error: could not start server because the socket file is occupied. Please check if {PRINT_NAME} is in use by another process and try again."
                );
                return Err(e.into());
            }
            x => x?,
        };

        let mut buf: Vec<u8> = Vec::with_capacity(std::mem::size_of::<Response>());

        for conn in listener.incoming() {
            let mut stream = BufReader::new(conn?);
            let request: Request = bincode::decode_from_reader(&mut stream, self.config)?;
            let response = self.process_request(request).await?;
            buf.clear();
            bincode::encode_into_slice(&response, &mut buf, self.config)?;
            stream.get_mut().write_all(&buf)?;
        }
        Ok(())
    }
    async fn process_request(&self, request: Request) -> Result<Response> {
        match request {
            Request::CreateTopic(request) => {
                self.storage
                    .create_topic(&request.topic_name, &request.content)
                    .await?;
                Ok(Response::CreateTopic(CreateTopicResponse { success: true }))
            }
            Request::UpdateTopic(request) => {
                self.storage
                    .update_topic(&request.topic_name, &request.content)
                    .await?;
                Ok(Response::UpdateTopic(UpdateTopicResponse { success: true }))
            }
            Request::SearchTopic(request) => {
                let results = self
                    .storage
                    .search_topic(request.topic_name.as_deref(), &request.query, request.limit)
                    .await?;
                Ok(Response::SearchTopic(SearchTopicResponse { results }))
            }
            Request::ListTopic(request) => {
                let results = self
                    .storage
                    .list_topic(&request.topic_name, request.limit)
                    .await?;
                Ok(Response::ListTopic(ListTopicResponse { results }))
            }
        }
    }
}

fn run_daemon<T: EmbeddingModel, C: Config>(daemon: Daemon<T, C>) -> eyre::Result<Daemonize<()>> {
    let stdout = File::create("/tmp/qdrant-cli-daemon.log")?;
    let stderr = File::create("/tmp/qdrant-cli-daemon.log")?;
    let daemonize = Daemonize::new()
        .pid_file("/tmp/qdrant-cli-daemon.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .stdout(stdout)
        .stderr(stderr);

    Ok(daemonize)
}
