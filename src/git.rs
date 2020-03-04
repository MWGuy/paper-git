use std::process::{Command, Stdio};
use std::io::Write;

pub enum GitPackType {
    Upload,
    Receive,
}

pub struct SmartGitRequest {
    pub repository: String
}

impl SmartGitRequest {
    pub fn info_refs(&self, pack_type: GitPackType) -> InfoRefsOutput {
        let mut output: String = String::new();

        match pack_type {
            GitPackType::Upload => output = string_push_bytes(output, "001e# service=git-upload-pack\n".as_bytes().to_vec()),
            GitPackType::Receive => output = string_push_bytes(output, "001f# service=git-receive-pack\n".as_bytes().to_vec())
        };

        let git_pack: String = match pack_type {
            GitPackType::Upload => String::from("upload-pack"),
            GitPackType::Receive => String::from("receive-pack")
        };

        let command = Command::new("git")
            .arg(git_pack.as_str())
            .arg("--stateless-rpc")
            .arg("--advertise-refs")
            .arg(self.repository.as_str()).output();

        output = string_push_bytes(output, "0000".as_bytes().to_vec());
        output = string_push_bytes(output, command
            .expect("failed to execute git process")
            .stdout);

        InfoRefsOutput {
            body: output,
            content_type: match pack_type {
                GitPackType::Upload => String::from("application/x-git-upload-pack-advertisement"),
                GitPackType::Receive => String::from("application/x-git-receive-pack-advertisement")
            }
        }
    }

    pub fn stateless_rpc(&self, pack_type: GitPackType, body: Vec<u8>) -> StatelessRpcOutput {
        let git_pack: String = match pack_type {
            GitPackType::Upload => String::from("upload-pack"),
            GitPackType::Receive => String::from("receive-pack")
        };

        let mut child = Command::new("git")
            .arg(git_pack.as_str())
            .arg("--stateless-rpc")
            .arg(self.repository.as_str())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().unwrap();

        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(body.as_ref()).unwrap();
        let child_output = child.wait_with_output().unwrap();

        StatelessRpcOutput {
            body: child_output.stdout,
            content_type: match pack_type {
                GitPackType::Upload => String::from("application/x-git-upload-pack-result"),
                GitPackType::Receive => String::from("application/x-git-receive-pack-result")
            }
        }
    }
}

pub struct InfoRefsOutput {
    pub body: String,
    pub content_type: String,
}

pub struct StatelessRpcOutput {
    pub body: Vec<u8>,
    pub content_type: String,
}

fn string_push_bytes(mut output: String, bytes: Vec<u8>) -> String {
    for byte in bytes {
        output.push(byte as char);
    }

    output
}
