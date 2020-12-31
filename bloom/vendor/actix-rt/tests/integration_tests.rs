use std::time::{Duration, Instant};

#[test]
fn start_and_stop() {
    actix_rt::System::new("start_and_stop").block_on(async move {
        assert!(
            actix_rt::Arbiter::is_running(),
            "System doesn't seem to have started"
        );
    });
    assert!(
        !actix_rt::Arbiter::is_running(),
        "System doesn't seem to have stopped"
    );
}

#[test]
fn await_for_timer() {
    let time = Duration::from_secs(2);
    let instant = Instant::now();
    actix_rt::System::new("test_wait_timer").block_on(async move {
        tokio::time::delay_for(time).await;
    });
    assert!(
        instant.elapsed() >= time,
        "Block on should poll awaited future to completion"
    );
}

#[test]
fn join_another_arbiter() {
    let time = Duration::from_secs(2);
    let instant = Instant::now();
    actix_rt::System::new("test_join_another_arbiter").block_on(async move {
        let mut arbiter = actix_rt::Arbiter::new();
        arbiter.send(Box::pin(async move {
            tokio::time::delay_for(time).await;
            actix_rt::Arbiter::current().stop();
        }));
        arbiter.join().unwrap();
    });
    assert!(
        instant.elapsed() >= time,
        "Join on another arbiter should complete only when it calls stop"
    );

    let instant = Instant::now();
    actix_rt::System::new("test_join_another_arbiter").block_on(async move {
        let mut arbiter = actix_rt::Arbiter::new();
        arbiter.exec_fn(move || {
            actix_rt::spawn(async move {
                tokio::time::delay_for(time).await;
                actix_rt::Arbiter::current().stop();
            });
        });
        arbiter.join().unwrap();
    });
    assert!(
        instant.elapsed() >= time,
        "Join on a arbiter that has used actix_rt::spawn should wait for said future"
    );

    let instant = Instant::now();
    actix_rt::System::new("test_join_another_arbiter").block_on(async move {
        let mut arbiter = actix_rt::Arbiter::new();
        arbiter.send(Box::pin(async move {
            tokio::time::delay_for(time).await;
            actix_rt::Arbiter::current().stop();
        }));
        arbiter.stop();
        arbiter.join().unwrap();
    });
    assert!(
        instant.elapsed() < time,
        "Premature stop of arbiter should conclude regardless of it's current state"
    );
}

#[test]
fn join_current_arbiter() {
    let time = Duration::from_secs(2);

    let instant = Instant::now();
    actix_rt::System::new("test_join_current_arbiter").block_on(async move {
        actix_rt::spawn(async move {
            tokio::time::delay_for(time).await;
            actix_rt::Arbiter::current().stop();
        });
        actix_rt::Arbiter::local_join().await;
    });
    assert!(
        instant.elapsed() >= time,
        "Join on current arbiter should wait for all spawned futures"
    );

    let large_timer = Duration::from_secs(20);
    let instant = Instant::now();
    actix_rt::System::new("test_join_current_arbiter").block_on(async move {
        actix_rt::spawn(async move {
            tokio::time::delay_for(time).await;
            actix_rt::Arbiter::current().stop();
        });
        let f = actix_rt::Arbiter::local_join();
        actix_rt::spawn(async move {
            tokio::time::delay_for(large_timer).await;
            actix_rt::Arbiter::current().stop();
        });
        f.await;
    });
    assert!(
        instant.elapsed() < large_timer,
        "local_join should await only for the already spawned futures"
    );
}
