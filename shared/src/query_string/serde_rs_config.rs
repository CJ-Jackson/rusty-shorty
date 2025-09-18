use poem::{Endpoint, Request};
use serde_qs::Config;

struct SerdeRsConfig<E: Endpoint>(Config, E);

impl<E: Endpoint> Endpoint for SerdeRsConfig<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        req.set_data(self.0.clone());

        self.1.call(req).await
    }
}

pub fn with_serde_rs_config<E: Endpoint>(config: Config, endpoint: E) -> impl Endpoint {
    SerdeRsConfig(config, endpoint)
}