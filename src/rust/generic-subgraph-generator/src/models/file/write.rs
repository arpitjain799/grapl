use graph_descriptions::file::FileState;
use graph_descriptions::graph_description::*;
use graph_descriptions::node::NodeT;
use graph_descriptions::process::ProcessState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct FileWrite {
    writer_pid: u64,
    writer_process_name: Option<String>,
    path: String,
    hostname: String,
    timestamp: u64,
}

impl From<FileWrite> for Graph {
    fn from(file_write: FileWrite) -> Self {
        let deleter = ProcessBuilder::default()
            .process_name(file_write.writer_process_name.unwrap_or_default())
            .hostname(file_write.hostname.clone())
            .state(ProcessState::Existing)
            .process_id(file_write.writer_pid)
            .last_seen_timestamp(file_write.timestamp)
            .build()
            .unwrap();

        let file = FileBuilder::default()
            .hostname(file_write.hostname)
            .state(FileState::Existing)
            .last_seen_timestamp(file_write.timestamp)
            .file_path(file_write.path)
            .build()
            .unwrap();

        let mut graph = Graph::new(file_write.timestamp);

        graph.add_edge(
            "wrote_files",
            deleter.clone_node_key(),
            file.clone_node_key(),
        );
        graph.add_node(deleter);
        graph.add_node(file);

        graph
    }
}
