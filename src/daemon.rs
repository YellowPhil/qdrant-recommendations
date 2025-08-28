use eyre::{Result, WrapErr};
use std::fs::File;

use daemonize::Daemonize;
use embedding::EmbeddingModel;
use storage_client::TopicStorage;

use std::io::{self, BufReader, prelude::*};

use interprocess::local_socket::{GenericNamespaced, ListenerOptions, Stream, prelude::*};

pub struct Daemon<T: EmbeddingModel> {
    storage: TopicStorage<T>,
    //TODO: Add persistent storage for topics
}

pub const PRINT_NAME: &str = "qdrant-cli-daemon.sock";

impl<T: EmbeddingModel> Daemon<T> {
    pub fn new(storage: TopicStorage<T>) -> Self {
        Self { storage }
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

        let mut line = Vec::with_capacity(1024);
        for conn in listener.incoming() {
            let mut stream = BufReader::new(conn?);
            stream.read(&mut line)?;
            line.clear();
            todo!("Main loop of processing the messages");
        }

        Ok(())
    }
}

fn run_daemon<T: EmbeddingModel>(daemon: Daemon<T>) -> eyre::Result<Daemonize<()>> {
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
