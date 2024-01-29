use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub struct WebSocketQuerier {
    url: String,
    socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl WebSocketQuerier {
    pub fn new(url: String) -> Self {
        Self { url, socket: None }
    }

    pub async fn connect(
        &mut self,
        path: &str,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        let url = format!("{}{}", self.url, path);
        let (socket, _) = tokio_tungstenite::connect_async(url).await?;
        self.socket = Some(socket);
        Ok(())
    }
}
