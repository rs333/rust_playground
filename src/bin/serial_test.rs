use derive_new::new;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

#[derive(new)]
pub struct Config<G, T>
where
    T: AsyncReadExt + AsyncWriteExt + std::marker::Unpin,
    G: FnMut(String, u32) -> T,
{
    device: String,
    rate: u32,
    initializer: G,
}

#[derive(new)]
pub struct Service<G, T>
where
    T: AsyncWriteExt + AsyncReadExt + std::marker::Unpin,
    G: FnMut(String, u32) -> T,
{
    config: Config<G, T>,
}
impl<G, T> Service<G, T>
where
    T: AsyncWriteExt + AsyncReadExt + std::marker::Unpin,
    G: FnMut(String, u32) -> T,
{
    async fn run(&mut self) {
        let mut stream = (self.config.initializer)(self.config.device.clone(), self.config.rate);
        // let _dog = stream.write_all(b"Hello").await;

        let mut buf: [u8; 5] = [0; 5];
        let _b = stream.read_exact(&mut buf).await;
        println!("{:?}", buf);
    }
}
pub fn get_serial_stream(name: String, rate: u32) -> SerialStream {
    tokio_serial::new(name, rate).open_native_async().unwrap()
}

#[tokio::main]
pub async fn main() {
    let mut service = Service::new(Config::new("/dev/dog".to_string(), 42, get_serial_stream));
    service.run().await;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_write_all() {
        let mut service = Service::new(Config::new("/dev/dog".to_string(), 42, get_serial_stream));
        service.run();
    }

    struct Mock {
        a: Option<tokio_serial::SerialStream>,
        b: Option<tokio_serial::SerialStream>,
    }

    #[cfg(unix)]
    impl Mock {
        pub fn new() -> Self {
            let (a, b) = tokio_serial::SerialStream::pair().unwrap();

            Self {
                a: Some(a),
                b: Some(b),
            }
        }

        pub fn old(&mut self) -> tokio_serial::SerialStream {
            self.b.take().unwrap()
        }
        pub fn smol(&mut self) -> tokio_serial::SerialStream {
            self.a.take().unwrap()
        }
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_write_all_2() {
        let mut dog = Mock::new();
        let mut rx = dog.smol();

        let mut service = Service::new(Config::new("/dev/dog".to_string(), 42, move |_, _| {
            dog.old()
        }));
        let _dog = rx.write_all(b"Hello").await;
        let _c = service.run().await;
    }
}
