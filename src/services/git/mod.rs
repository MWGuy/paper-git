use std::process::{Command, Stdio};
use std::io::Write;

pub enum GitPackType {
    Upload,
    Receive,
}

pub struct GitService {
    pub repository: String
}

impl GitService {
    pub fn info_refs(&self, pack_type: GitPackType) -> InfoRefsOutput {
        let mut output: String = match pack_type {
            GitPackType::Upload => "001e# service=git-upload-pack\n".into(),
            GitPackType::Receive => "001f# service=git-receive-pack\n".into(),
        };

        let git_pack: &str = match pack_type {
            GitPackType::Upload => "upload-pack",
            GitPackType::Receive => "receive-pack",
        };

        let command = Command::new("git")
            .arg(git_pack)
            .arg("--stateless-rpc")
            .arg("--advertise-refs")
            .arg(self.repository.as_str())
            .output()
            .expect("failed to execute git process");

        if !command.status.success() {
            let err = String::from_utf8(command.stderr).unwrap();
            panic!(err);
        }

        output.push_str("0000");
        output.push_str(String::from_utf8_lossy(&*command.stdout).as_ref());

        InfoRefsOutput {
            body: output,
            content_type: match pack_type {
                GitPackType::Upload => String::from("application/x-git-upload-pack-advertisement"),
                GitPackType::Receive => String::from("application/x-git-receive-pack-advertisement")
            },
        }
    }

    pub fn stateless_rpc(&self, pack_type: GitPackType, body: Vec<u8>) -> StatelessRpcOutput {
        let git_pack: &str = match pack_type {
            GitPackType::Upload => "upload-pack",
            GitPackType::Receive => "receive-pack",
        };

        let mut child = Command::new("git")
            .arg(git_pack)
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
            },
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
