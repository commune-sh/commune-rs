use std::sync::Arc;

use matrix::client::resources::login::Login;
use matrix::Client as MatrixAdminClient;

use crate::util::secret::Secret;
use crate::Result;

pub struct MailService {
    admin: Arc<MatrixAdminClient>,
}
