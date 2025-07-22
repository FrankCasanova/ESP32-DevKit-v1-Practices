use core::sync::atomic::Ordering;

use defmt::info;
use embassy_net::Stack;
use embassy_time::Duration;
use esp_alloc as _;
use esp_println::println;
use picoserve::request::{self, Request};
use picoserve::response::IntoResponse;
use picoserve::{response::File, routing, AppBuilder, AppRouter, Router};
use picoserve::{response::ResponseWriter, routing::RequestHandlerService, ResponseSent};

pub struct Application;
struct LoggingFileService {
    html_content: &'static str,
}

impl<State, PathParameters> RequestHandlerService<State, PathParameters> for LoggingFileService {
    async fn call_request_handler_service<
        R: picoserve::io::Read,
        W: ResponseWriter<Error = R::Error>,
    >(
        &self,
        _state: &State,
        _path_parameters: PathParameters,
        request: Request<'_, R>,
        response_writer: W,
    ) -> Result<ResponseSent, W::Error> {
        // Log the request information
        info!("Got request");
        println!("Headers: {:?}", request.parts.headers());
        println!("Method: {:?}", request.parts.method());
        println!("Path: {:?}", request.parts.path());
        println!("Query: {:?}", request.parts.query());
        println!("Http Version: {:?}", request.parts.http_version());
        println!("Fragments: {:?}", request.parts.fragments());

        // Serve the HTML file
        File::html(self.html_content)
            .call_request_handler_service(_state, _path_parameters, request, response_writer)
            .await
    }
}

impl AppBuilder for Application {
    type PathRouter = impl routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        picoserve::Router::new()
        .route(
            "/",
            routing::get_service(LoggingFileService {
                html_content: include_str!("index.html"),
            }))
        .route(
            "/led", routing::post(led_handler)
        )

        
    }
}

#[derive(serde::Deserialize)]
struct LedRequest {
    is_on: bool,
}

#[derive(serde::Serialize)]
struct LedResponse {
    success: bool,
}
async fn led_handler(input: picoserve::extract::Json<LedRequest, 0>) -> impl IntoResponse {
    crate::led::LED_STATE.store(input.0.is_on, Ordering::Relaxed);
    info!("post request recived");
    picoserve::response::Json(LedResponse { success: true })
}



pub const WEB_TASK_POOL_SIZE: usize = 2;

pub struct WebApp {
    pub router: &'static Router<<Application as AppBuilder>::PathRouter>,
    pub config: &'static picoserve::Config<Duration>,
}

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
pub async fn web_task(
    id: usize,
    stack: Stack<'static>,
    router: &'static AppRouter<Application>,
    config: &'static picoserve::Config<Duration>,
) -> ! {
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];
    picoserve::listen_and_serve(
        id,
        router,
        config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await
}

impl Default for WebApp {
    fn default() -> Self {
        let router = picoserve::make_static!(AppRouter<Application>, Application.build_app());

        let config = picoserve::make_static!(
            picoserve::Config<Duration>,
            picoserve::Config::new(picoserve::Timeouts {
                start_read_request: Some(Duration::from_secs(5)),
                persistent_start_read_request: Some(Duration::from_secs(3)),
                read_request: Some(Duration::from_secs(1)),
                write: Some(Duration::from_secs(1)),
            })
            .keep_connection_alive()
        );

        Self {
            router: router,
            config: config,
        }
    }
}
