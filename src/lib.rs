macro_rules! cfg_websocket {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "websocket")]
            #[cfg_attr(docsrs, doc(cfg(feature = "websocket")))]
            $item
        )*
    }
}

macro_rules! cfg_http {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "http")]
            #[cfg_attr(docsrs, doc(cfg(feature = "http")))]
            $item
        )*
    }
}

cfg_http! {
    pub mod http;
}

cfg_websocket! {
    pub mod websocket;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
