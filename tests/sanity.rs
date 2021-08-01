use companion_service::{Service, SERVICES};
use linkme::distributed_slice;
use std::sync::atomic::{AtomicIsize, Ordering};

const TEST_SERVICE_NAME: &str = "test service";

struct TestService {
    start_stop_count: AtomicIsize,
}

impl TestService {
    const fn new() -> Self {
        Self {
            start_stop_count: AtomicIsize::new(0),
        }
    }

    fn start_stop_count(&self) -> isize {
        self.start_stop_count.load(Ordering::SeqCst)
    }
}

impl Service for TestService {
    fn name(&self) -> &str {
        TEST_SERVICE_NAME
    }

    fn start(&self) {
        self.start_stop_count.fetch_add(1, Ordering::SeqCst);
    }

    fn stop(&self) {
        self.start_stop_count.fetch_sub(1, Ordering::SeqCst);
    }
}

static TEST_SERVICE_IMPL: TestService = TestService::new();

#[distributed_slice(SERVICES)]
static TEST_SERVICE: &(dyn Service + Sync) = &TEST_SERVICE_IMPL;

#[test]
fn test() {
    assert_eq!(TEST_SERVICE_IMPL.start_stop_count(), 1);
    companion_service::stop(TEST_SERVICE_NAME);
    assert_eq!(TEST_SERVICE_IMPL.start_stop_count(), 0);
    companion_service::start(TEST_SERVICE_NAME);
    assert_eq!(TEST_SERVICE_IMPL.start_stop_count(), 1);
    companion_service::restart(TEST_SERVICE_NAME);
    assert_eq!(TEST_SERVICE_IMPL.start_stop_count(), 1);

    // Unfortunately we can't test the destructor
}
