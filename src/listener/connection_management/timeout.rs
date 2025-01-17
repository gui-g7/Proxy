// Implementação de timeouts para conexões.

use std::time::Duration;

pub fn check_timeout(last_activity: Duration, timeout_limit: Duration) -> bool {
    last_activity > timeout_limit
}

