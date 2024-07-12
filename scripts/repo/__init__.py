INTERNAL_SERVER = "115.157.210.249"
INTERNAL_SERVER_PORT = "8080"
REPOSITORY_NAMESPACE = "bigherox-robocup"
REPOSITORY_NAME = "scripts"

class Repository:
    def __init__(
        self,
        server = INTERNAL_SERVER,
        server_port = INTERNAL_SERVER_PORT,
        repository_namespace = REPOSITORY_NAMESPACE,
        repository_name = REPOSITORY_NAME,
        folder_name = None,
        default_branch = "main",
    ) -> None:
        self.server = server
        self.server_port = server_port
        self.repository_namespace = repository_namespace
        self.repository_name = repository_name
        self.folder_name = repository_name
        self.default_branch = default_branch
        if folder_name is not None:
            self.folder_name = folder_name

    def git_remote_addr(self) -> str:
        port_str = f":{self.server_port}" if self.server_port is not None else ""
        return f"http://{self.server}{port_str}/{self.repository_namespace}/{self.repository_name}.git"


REPOSITORIES = [
    Repository(repository_name="bigherox-robocup"),
    Repository(repository_name="test-cpp-input", default_branch="master"),
    Repository(repository_name="test-cpp-input-rs", default_branch="master"),
    Repository(repository_name="test-serial-port", default_branch="master"),
]
